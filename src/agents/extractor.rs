use crate::config::{AgentConfig, RoutingConfig};
use crate::agents::{AgentRegistry, AgentRouter, registry::{TaskInfo, TaskStatus}};
use crate::mcp::{DelegateTaskArgs, DelegateTaskOutput};
use crate::rate_limit::RateLimitTracker;
use anyhow::{Result, Context, bail};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::process::{Command, ChildStdin, ChildStdout};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use serde_json::Value;
use uuid::Uuid;

// --- ส่วนที่เพิ่มเข้ามา: Conditional Import ---
// บรรทัดนี้จะถูกคอมไพล์ก็ต่อเมื่อฟีเจอร์ `bundle-pmat` ถูกเปิดใช้งาน
// เราจะสมมติว่า pmat-core มีฟังก์ชัน `run_context_analysis` ที่รับ prompt และคืนค่า Result<String>
#[cfg(feature = "bundle-pmat")]
use pmat_core::run_context_analysis;


pub struct AgentExecutor {
    agent_registry: Arc<RwLock<AgentRegistry>>,
    rate_limiter: Arc<RwLock<RateLimitTracker>>,
    router: AgentRouter,
}

impl AgentExecutor {
    pub fn new(
        agent_registry: Arc<RwLock<AgentRegistry>>,
        rate_limiter: Arc<RwLock<RateLimitTracker>>,
        routing_config: RoutingConfig,
    ) -> Self {
        let router = AgentRouter::new(routing_config);

        Self {
            agent_registry,
            rate_limiter,
            router,
        }
    }

    /// Delegate a task to an appropriate sub-agent using ACP
    pub async fn delegate_task(&self, args: DelegateTaskArgs) -> pmcp::Result<DelegateTaskOutput> {
        // Generate task ID
        let task_id = Uuid::new_v4().to_string();

        // Select agent
        let registry = self.agent_registry.read().await;

        let agent = if let Some(agent_id) = &args.agent_id {
            registry.get_agent(agent_id)
                .ok_or_else(|| pmcp::Error::validation(format!("Agent not found: {}", agent_id)))?
        } else {
            // Auto-select based on task_type
            let all_agents = registry.get_agents_by_priority();
            let agent_refs: Vec<&AgentConfig> = all_agents.iter().copied().collect();

            self.router.select_agent(&args.task_type, &args.prompt, &agent_refs)
                .map_err(|e| pmcp::Error::internal(e.to_string()))?
        };

        let agent_id = agent.id.clone();
        let agent_config = agent.clone();

        // Register task
        drop(registry);
        let mut registry = self.agent_registry.write().await;
        registry.register_task(TaskInfo {
            task_id: task_id.clone(),
            agent_id: agent_id.clone(),
            task_type: args.task_type.clone(),
            status: TaskStatus::Pending,
        });
        drop(registry);

        // Check rate limit
        let mut rate_limiter = self.rate_limiter.write().await;
        if !rate_limiter.check_and_increment(&agent_id).await {
            return Err(pmcp::Error::custom(
                -32000, // RATE_LIMIT_EXCEEDED
                format!("Rate limit exceeded for agent: {}", agent_id)
            ));
        }
        drop(rate_limiter);

        // Execute task
        if args.background {
            // Spawn background task
            let executor = self.clone_for_background();
            tokio::spawn(async move {
                if let Err(e) = executor.execute_agent_task(
                    task_id,
                    agent_config,
                    args.prompt,
                    args.context,
                ).await {
                    tracing::error!("Background task failed: {}", e);
                }
            });

            Ok(DelegateTaskOutput {
                task_id,
                agent_id,
                status: "pending".to_string(),
                result: None,
            })
        } else {
            // Execute synchronously
            let result = self.execute_agent_task(
                task_id.clone(),
                agent_config,
                args.prompt,
                args.context,
            ).await.map_err(|e| pmcp::Error::internal(e.to_string()))?;

            Ok(DelegateTaskOutput {
                task_id,
                agent_id,
                status: "completed".to_string(),
                result: Some(result),
            })
        }
    }

    /// Execute task on a specific agent using ACP protocol
    async fn execute_agent_task(
        &self,
        task_id: String,
        agent: AgentConfig,
        prompt: String,
        context: Option<Value>,
    ) -> Result<String> {
        // Update status to running
        let mut registry = self.agent_registry.write().await;
        registry.update_task_status(&task_id, TaskStatus::Running)?;
        drop(registry);

        tracing::info!("Executing task {} on agent {}", task_id, agent.id);

        // --- ส่วนที่แก้ไข: เพิ่มการจัดการ Agent ประเภทใหม่ ---
        // เราจะเพิ่ม agent_type ใหม่ชื่อ "internal" สำหรับ pmat ที่บันเดิลมาด้วย
        let result = match agent.agent_type.as_str() {
            "cli" => self.execute_cli_agent(&agent, &prompt, context).await,
            "gemini-extension" => self.execute_gemini_extension(&agent, &prompt).await,
            
            // --- ตรรกะใหม่สำหรับ Internal Agent ---
            "internal" => {
                // ตรวจสอบว่า command คือ pmat หรือไม่ (เพื่อความปลอดภัย)
                if agent.command.as_deref() == Some("pmat-internal") {
                    self.execute_internal_pmat_agent(&prompt).await
                } else {
                    bail!("Unsupported internal agent: {:?}", agent.command)
                }
            },

            _ => bail!("Unsupported agent type: {}", agent.agent_type),
        };

        // Update final status
        let mut registry = self.agent_registry.write().await;
        match &result {
            Ok(_) => registry.update_task_status(&task_id, TaskStatus::Completed)?,
            Err(_) => registry.update_task_status(&task_id, TaskStatus::Failed)?,
        }

        result
    }

    // --- ฟังก์ชันใหม่: สำหรับเรียกใช้ pmat ที่บันเดิลมา ---
    // ฟังก์ชันนี้จะถูกคอมไพล์ก็ต่อเมื่อฟีเจอร์ `bundle-pmat` ถูกเปิดใช้งาน
    #[cfg(feature = "bundle-pmat")]
    async fn execute_internal_pmat_agent(&self, prompt: &str) -> Result<String> {
        tracing::debug!("Executing bundled PMAT agent with prompt: {}", prompt);
        
        // นี่คือจุดที่เราเรียกใช้ฟังก์ชันจาก pmat-core library โดยตรง
        // เราสมมติว่า pmat-core มีฟังก์ชัน `run_context_analysis` ที่ทำงานแบบ async
        // และคืนค่าเป็น Result<String, pmat_core::Error>
        // เราต้องแปลง Error ของ pmat ให้เป็น anyhow::Error
        let result = run_context_analysis(prompt).await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(result)
    }

    // --- ฟังก์ชันสำรอง (Fallback): เมื่อไม่ได้เปิดฟีเจอร์ `bundle-pmat` ---
    // ฟังก์ชันนี้จะถูกคอมไพล์เมื่อ *ไม่ได้* เปิดฟีเจอร์ `bundle-pmat`
    // มันจะคืนค่าเป็น Error เสมอ เพื่อป้องกันการเรียกใช้ Agent ที่ไม่มีอยู่จริง
    #[cfg(not(feature = "bundle-pmat"))]
    async fn execute_internal_pmat_agent(&self, _prompt: &str) -> Result<String> {
        bail!("Internal PMAT agent called, but the 'bundle-pmat' feature is not enabled. Please compile with --features bundle-pmat or use the CLI version of pmat.")
    }


    /// Execute CLI-based agent using ACP over stdio
    async fn execute_cli_agent(
        &self,
        agent: &AgentConfig,
        prompt: &str,
        context: Option<Value>,
    ) -> Result<String> {
        let command = agent.command.as_ref()
            .context("CLI agent requires command")?;

        tracing::debug!("Spawning process: {} {:?}", command, agent.args);

        let mut child = Command::new(command)
            .args(agent.args.as_deref().unwrap_or(&[]))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn agent process")?;

        let stdin = child.stdin.take().context("Failed to get stdin")?;
        let stdout = child.stdout.take().context("Failed to get stdout")?;

        // Send ACP request (JSON-RPC 2.0)
        // *** แก้ไขเล็กน้อย: ส่ง prompt ไปใน arguments โดยตรงตามที่ pmat คาดหวัง ***
        let acp_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "context", // สมมติว่า pmat รับ method `context`
            "params": {
                "prompt": prompt,
                "context": context,
                "format": "llm-optimized" // อาจจะใส่ค่า default ที่ดีไปเลย
            }
        });

        // Write request to stdin
        let request_line = serde_json::to_string(&acp_request)? + "\n";
        self.write_to_agent(stdin, &request_line).await?;

        // Read response from stdout
        let response = self.read_from_agent(stdout).await?;

        // Wait for process to complete
        let status = child.wait().await?;
        if !status.success() {
            bail!("Agent process exited with error: {}", status);
        }

        Ok(response)
    }

    /// Write JSON-RPC request to agent's stdin
    async fn write_to_agent(&self, mut stdin: ChildStdin, data: &str) -> Result<()> {
        stdin.write_all(data.as_bytes()).await?;
        stdin.flush().await?;
        Ok(())
    }

    /// Read JSON-RPC response from agent's stdout
    async fn read_from_agent(&self, stdout: ChildStdout) -> Result<String> {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        reader.read_line(&mut line).await?;

        let response: Value = serde_json::from_str(&line)
            .context("Failed to parse agent response")?;

        // Extract result from JSON-RPC response
        if let Some(result) = response.get("result") {
            // ผลลัพธ์จาก pmat อาจจะเป็น JSON object ที่มี context อยู่ข้างใน หรือเป็น string ตรงๆ
            // ทำให้มันเป็น string เสมอเพื่อความเข้ากันได้
            if result.is_string() {
                Ok(result.as_str().unwrap().to_string())
            } else {
                Ok(result.to_string())
            }
        } else if let Some(error) = response.get("error") {
            bail!("Agent returned error: {}", error);
        } else {
            bail!("Invalid JSON-RPC response");
        }
    }

    /// Execute Gemini extension-based agent
    async fn execute_gemini_extension(
        &self,
        agent: &AgentConfig,
        prompt: &str,
    ) -> Result<String> {
        let extension_name = agent.extension_name.as_ref()
            .context("Extension agent requires extension_name")?;

        tracing::info!("Calling Gemini extension: {}", extension_name);

        // TODO: Implement Gemini extension calling mechanism
        // This would involve calling the Gemini CLI with the extension
        bail!("Gemini extension support not yet implemented")
    }

    /// Clone for background execution (Arc cloning)
    fn clone_for_background(&self) -> Self {
        Self {
            agent_registry: self.agent_registry.clone(),
            rate_limiter: self.rate_limiter.clone(),
            router: AgentRouter::new(RoutingConfig { rules: vec![] }),
        }
    }
}
