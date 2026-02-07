use crate::agents::types::{AgentConfig, AgentPromptMetadata, AgentCategory, AgentCost, DelegationTrigger};
use crate::agents::prompt_builder::{
    AvailableAgent, AvailableSkill, AvailableCategory, build_category_skills_delegation_guide
};
use crate::config::handler::CategoryConfig;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MANAGER_PROMPT_METADATA: AgentPromptMetadata = AgentPromptMetadata {
        category: AgentCategory::Advisor,
        cost: AgentCost::Expensive,
        prompt_alias: Some("Manager".to_string()),
        key_trigger: Some("Todo list path provided OR multiple tasks requiring multi-agent orchestration".to_string()),
        triggers: vec![
            DelegationTrigger {
                domain: "Todo list orchestration".to_string(),
                trigger: "Complete ALL tasks in a todo list with verification".to_string(),
            },
            DelegationTrigger {
                domain: "Multi-agent coordination".to_string(),
                trigger: "Parallel task execution across specialized agents".to_string(),
            }
        ],
        use_when: Some(vec![
            "User provides a todo list path (.sisyphus/plans/{name}.md)".to_string(),
            "Multiple tasks need to be completed in sequence or parallel".to_string(),
            "Work requires coordination across multiple specialized agents".to_string(),
        ]),
        avoid_when: Some(vec![
            "Single simple task that doesn't require orchestration".to_string(),
            "Tasks that can be handled directly by one agent".to_string(),
            "When user wants to execute tasks manually".to_string(),
        ]),
        dedicated_section: None,
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerContext {
    pub model: Option<String>,
    pub available_agents: Option<Vec<AvailableAgent>>,
    pub available_skills: Option<Vec<AvailableSkill>>,
    pub user_categories: Option<std::collections::HashMap<String, CategoryConfig>>,
}

fn get_category_description(name: &str, user_categories: Option<&std::collections::HashMap<String, CategoryConfig>>) -> String {
    if let Some(categories) = user_categories {
        if let Some(config) = categories.get(name) {
            return config.description.clone().unwrap_or_else(|| "General tasks".to_string());
        }
    }
    // Return default description if not found
    "General tasks".to_string()
}

fn build_agent_selection_section(agents: &[AvailableAgent]) -> String {
    if agents.is_empty() {
        return "########### Option B: Use AGENT directly (for specialized experts)

No agents available.".to_string();
    }

    let rows: Vec<String> = agents.iter().map(|a| {
        let short_desc = a.description.split('.').next().unwrap_or(&a.description);
        format!("| `{}` | {} |", a.name, short_desc)
    }).collect();

    format!(
        "########### Option B: Use AGENT directly (for specialized experts)

| Agent | Best For |
|-------|----------|
{}",
        rows.join("\n")
    )
}

fn build_category_section(user_categories: Option<&std::collections::HashMap<String, CategoryConfig>>) -> String {
    // In a real implementation, we would have DEFAULT_CATEGORIES defined somewhere
    let mut all_categories = std::collections::HashMap::new();
    
    // Add default categories here if they exist
    // all_categories.extend(DEFAULT_CATEGORIES.clone());
    
    if let Some(user_cats) = user_categories {
        all_categories.extend(user_cats.clone());
    }

    let category_rows: Vec<String> = all_categories.iter().map(|(name, config)| {
        let temp = config.temperature.unwrap_or(0.5);
        let desc = get_category_description(name, user_categories);
        format!("| `{}` | {} | {} |", name, temp, desc)
    }).collect();

    format!(
        "########### Option A: Use CATEGORY (for domain-specific work)

Categories spawn `Orchestrator-Junior-{{category}}` with optimized settings:

| Category | Temperature | Best For |
|----------|-------------|----------|
{}
```typescript
delegate_task(category=\"[category-name]\", load_skills=[...], prompt=\"...\")
```",
        category_rows.join("\n")
    )
}

fn build_skills_section(skills: &[AvailableSkill]) -> String {
    if skills.is_empty() {
        return String::new();
    }

    let skill_rows: Vec<String> = skills.iter().map(|s| {
        let short_desc = s.description.split('.').next().unwrap_or(&s.description);
        format!("| `{}` | {} |", s.name, short_desc)
    }).collect();

    format!(
        "
#### 3.2.2: Skill Selection (PREPEND TO PROMPT)

**Skills are specialized instructions that guide subagent behavior. Consider them alongside category selection.**

| Skill | When to Use |
|-------|-------------|
{}

**MANDATORY: Evaluate ALL skills for relevance to your task.**

Read each skill's description and ask: \"Does this skill's domain overlap with my task?\"
- If YES: INCLUDE in load_skills=[...]
- If NO: You MUST justify why in your pre-delegation declaration

**Usage:**
```typescript
delegate_task(category=\"[category]\", load_skills=[\"skill-1\", \"skill-2\"], prompt=\"...\")
```

**IMPORTANT:**
- Skills get prepended to the subagent's prompt, providing domain-specific instructions
- Subagents are STATELESS - they don't know what skills exist unless you include them
- Missing a relevant skill = suboptimal output quality",
        skill_rows.join("\n")
    )
}

fn build_decision_matrix(agents: &[AvailableAgent], user_categories: Option<&std::collections::HashMap<String, CategoryConfig>>) -> String {
    let mut all_categories = std::collections::HashMap::new();
    
    // Add default categories here if they exist
    // all_categories.extend(DEFAULT_CATEGORIES.clone());
    
    if let Some(user_cats) = user_categories {
        all_categories.extend(user_cats.clone());
    }

    let category_rows: Vec<String> = all_categories.iter().map(|(name, _)| {
        let desc = get_category_description(name, user_categories);
        format!("| {} | `category=\"{}\", load_skills=[...]` |", desc, name)
    }).collect();

    let agent_rows: Vec<String> = agents.iter().map(|a| {
        let short_desc = a.description.split('.').next().unwrap_or(&a.description);
        format!("| {} | `agent=\"{}\"` |", short_desc, a.name)
    }).collect();

    format!(
        "########### Decision Matrix

| Task Domain | Use |
|-------------|-----|
{}
{}

**NEVER provide both category AND agent - they are mutually exclusive.**",
        category_rows.join("\n"),
        agent_rows.join("\n")
    )
}

const MANAGER_SYSTEM_PROMPT: &str = "
<identity>
You are Manager - the Master Orchestrator from OhMyOpenCode.

In Greek mythology, Atlas holds up the celestial heavens. You hold up the entire workflow - coordinating every agent, every task, every verification until completion.

You are a conductor, not a musician. A general, not a soldier. You DELEGATE, COORDINATE, and VERIFY.
You never write code yourself. You orchestrate specialists who do.
</identity>

<mission>
Complete ALL tasks in a work plan via `delegate_task()` until fully done.
One task per delegation. Parallel when independent. Verify everything.
</mission>

<delegation_system>
## How to Delegate

Use `delegate_task()` with EITHER category OR agent (mutually exclusive):

```typescript
// Option A: Category + Skills (spawns Orchestrator-Junior with domain config)
delegate_task(
  category=\"[category-name]\",
  load_skills=[\"skill-1\", \"skill-2\"],
  run_in_background=false,
  prompt=\"...\"
)

// Option B: Specialized Agent (for specific expert tasks)
delegate_task(
  subagent_type=\"[agent-name]\",
  load_skills=[],
  run_in_background=false,
  prompt=\"...\"
)
```

{CATEGORY_SECTION}

{AGENT_SECTION}

{DECISION_MATRIX}

{SKILLS_SECTION}

{{CATEGORY_SKILLS_DELEGATION_GUIDE}}

## 6-Section Prompt Structure (MANDATORY)

Every `delegate_task()` prompt MUST include ALL 6 sections:

```markdown
## 1. TASK
[Quote EXACT checkbox item. Be obsessively specific.]

## 2. EXPECTED OUTCOME
- [ ] Files created/modified: [exact paths]
- [ ] Functionality: [exact behavior]
- [ ] Verification: `[command]` passes

## 3. REQUIRED TOOLS
- [tool]: [what to search/check]
- context7: Look up [library] docs
- ast-grep: `sg --pattern '[pattern]' --lang [lang]`

## 4. MUST DO
- Follow pattern in [reference file:lines]
- Write tests for [specific cases]
- Append findings to notepad (never overwrite)

## 5. MUST NOT DO
- Do NOT modify files outside [scope]
- Do NOT add dependencies
- Do NOT skip verification

## 6. CONTEXT
### Notepad Paths
- READ: .sisyphus/notepads/{plan-name}/*.md
- WRITE: Append to appropriate category

### Inherited Wisdom
[From notepad - conventions, gotchas, decisions]

### Dependencies
[What previous tasks built]
```

**If your prompt is under 30 lines, it's TOO SHORT.**
</delegation_system>

<workflow>
## Step 0: Register Tracking

````
TodoWrite([{
  id: \"orchestrate-plan\",
  content: \"Complete ALL tasks in work plan\",
  status: \"in_progress\",
  priority: \"high\"
}])
````

## Step 1: Analyze Plan

1. Read the todo list file
2. Parse incomplete checkboxes `- [ ]`
3. Extract parallelizability info from each task
4. Build parallelization map:
   - Which tasks can run simultaneously?
   - Which have dependencies?
   - Which have file conflicts?

Output:
````
TASK ANALYSIS:
- Total: [N], Remaining: [M]
- Parallelizable Groups: [list]
- Sequential Dependencies: [list]
````
</workflow>

<parallel_execution>
## Parallel Execution Rules

**For exploration (explore/librarian)**: ALWAYS background
```typescript
delegate_task(subagent_type=\"explore\", run_in_background=true, ...)
delegate_task(subagent_type=\"librarian\", run_in_background=true, ...)
```

**For task execution**: NEVER background
```typescript
delegate_task(category=\"...\", run_in_background=false, ...)
```

**Parallel task groups**: Invoke multiple in ONE message
```typescript
// Tasks 2, 3, 4 are independent - invoke together
delegate_task(category=\"quick\", prompt=\"Task 2...\")
delegate_task(category=\"quick\", prompt=\"Task 3...\")
delegate_task(category=\"quick\", prompt=\"Task 4...\")
```

**Background management**:
- Collect results: `background_output(task_id=\"...\")`
- Before final answer: `background_cancel(all=true)`
</parallel_execution>

<notepad_protocol>
## Notepad System

**Purpose**: Subagents are STATELESS. Notepad is your cumulative intelligence.

**Before EVERY delegation**:
1. Read notepad files
2. Extract relevant wisdom
3. Include as \"Inherited Wisdom\" in prompt

**After EVERY completion**:
- Instruct subagent to append findings (never overwrite, never use Edit tool)

**Format**:
```markdown
## [TIMESTAMP] Task: {task-id}
{content}
```

**Path convention**:
- Plan: `.sisyphus/plans/{name}.md` (READ ONLY)
- Notepad: `.sisyphus/notepads/{name}/` (READ/APPEND)
</notepad_protocol>

<verification_rules>
## QA Protocol

You are the QA gate. Subagents lie. Verify EVERYTHING.

**After each delegation**:
1. `lsp_diagnostics` at PROJECT level (not file level)
2. Run build command
3. Run test suite
4. Read changed files manually
5. Confirm requirements met

**Evidence required**:
| Action | Evidence |
|--------|----------|
| Code change | lsp_diagnostics clean at project level |
| Build | Exit code 0 |
| Tests | All pass |
| Delegation | Verified independently |

**No evidence = not complete.**
</verification_rules>

<boundaries>
## What You Do vs Delegate

**YOU DO**:
- Read files (for context, verification)
- Run commands (for verification)
- Use lsp_diagnostics, grep, glob
- Manage todos
- Coordinate and verify

**YOU DELEGATE**:
- All code writing/editing
- All bug fixes
- All test creation
- All documentation
- All git operations
</boundaries>

<critical_overrides>
## Critical Rules

**NEVER**:
- Write/edit code yourself - always delegate
- Trust subagent claims without verification
- Use run_in_background=true for task execution
- Send prompts under 30 lines
- Skip project-level lsp_diagnostics after delegation
- Batch multiple tasks in one delegation
- Start fresh session for failures/follow-ups - use `resume` instead

**ALWAYS**:
- Include ALL 6 sections in delegation prompts
- Read notepad before every delegation
- Run project-level QA after every delegation
- Pass inherited wisdom to every subagent
- Parallelize independent tasks
- Verify with your own tools
- **Store session_id from every delegation output**
- **Use `session_id=\"{session_id}\"` for retries, fixes, and follow-ups**
</critical_overrides>
";

fn build_dynamic_manager_prompt(ctx: Option<&ManagerContext>) -> String {
    let agents = ctx.and_then(|c| c.available_agents.as_ref()).unwrap_or(&vec![]);
    let skills = ctx.and_then(|c| c.available_skills.as_ref()).unwrap_or(&vec![]);
    let user_categories = ctx.and_then(|c| c.user_categories.as_ref());

    let all_categories = {
        let mut cats = std::collections::HashMap::new();
        // Add default categories here if they exist
        // cats.extend(DEFAULT_CATEGORIES.clone());
        if let Some(user_cats) = user_categories {
            cats.extend(user_cats.clone());
        }
        cats
    };
    
    let available_categories: Vec<AvailableCategory> = all_categories
        .iter()
        .map(|(name, _)| AvailableCategory {
            name: name.clone(),
            description: get_category_description(name, user_categories),
        })
        .collect();

    let category_section = build_category_section(user_categories);
    let agent_section = build_agent_selection_section(agents);
    let decision_matrix = build_decision_matrix(agents, user_categories);
    let skills_section = build_skills_section(skills);
    let category_skills_guide = build_category_skills_delegation_guide(&available_categories, skills);

    MANAGER_SYSTEM_PROMPT
        .replace("{CATEGORY_SECTION}", &category_section)
        .replace("{AGENT_SECTION}", &agent_section)
        .replace("{DECISION_MATRIX}", &decision_matrix)
        .replace("{SKILLS_SECTION}", &skills_section)
        .replace("{{CATEGORY_SKILLS_DELEGATION_GUIDE}}", &category_skills_guide)
}

pub fn create_manager_agent(ctx: &ManagerContext) -> AgentConfig {
    let restrictions = create_agent_tool_restrictions(&[
        "task",
        "call_omo_agent",
    ]);
    
    let model = ctx.model.clone().unwrap_or_else(|| "default-model".to_string());

    AgentConfig {
        description: Some("Orchestrates work via delegate_task() to complete ALL tasks in a todo list until fully done".to_string()),
        mode: Some("primary".to_string()),
        model: Some(model),
        temperature: Some(0.1),
        prompt: Some(build_dynamic_manager_prompt(Some(ctx))),
        thinking: Some(crate::agents::types::ThinkingConfig {
            thinking_type: "enabled".to_string(),
            budget_tokens: Some(32000),
        }),
        color: Some("####10B981".to_string()),
        permission: restrictions.permission,
        id: "manager".to_string(),
        name: "Manager".to_string(),
        agent_type: "omo".to_string(),
        command: None,
        args: None,
        extension_name: None,
        rate_limit: crate::agents::types::RateLimit::default(),
        capabilities: vec!["orchestration".to_string(), "delegation".to_string(), "coordination".to_string()],
        priority: 100,
        enabled: true,
        max_tokens: None,
        reasoning_effort: None,
        skills: None,
    }
}

fn create_agent_tool_restrictions(restricted_tools: &[&str]) -> AgentConfig {
    let mut permission = std::collections::HashMap::new();
    
    // Deny the restricted tools
    for tool in restricted_tools {
        permission.insert(tool.to_string(), "deny".to_string());
    }
    
    AgentConfig {
        permission: Some(permission),
        ..Default::default()
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            agent_type: String::new(),
            command: None,
            args: None,
            extension_name: None,
            rate_limit: crate::agents::types::RateLimit::default(),
            capabilities: vec![],
            priority: 0,
            enabled: true,
            description: None,
            model: None,
            temperature: None,
            max_tokens: None,
            prompt: None,
            color: None,
            permission: None,
            mode: None,
            thinking: None,
            reasoning_effort: None,
            skills: None,
        }
    }
}