use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio::process::Command;
use std::path::PathBuf;
use tokio::fs;
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub version: Option<String>,
    pub available: bool,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryReport {
    pub timestamp: DateTime<Utc>,
    pub ai_clis: Vec<ToolInfo>,
    pub vcs: Vec<ToolInfo>,
    pub package_managers: Vec<ToolInfo>,
}

pub struct DiscoveryEngine;

impl DiscoveryEngine {
    /// Scans the system for known AI CLIs, version control systems, and package managers and aggregates the results into a discovery report.
    ///
    —
    /// # Returns
    ///
    /// A `DiscoveryReport` containing the timestamped results with detected tools grouped into `ai_clis`, `vcs`, and `package_managers`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Run the async scan and assert it returns a report with the expected sections.
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    /// let report = rt.block_on(crate::system::discovery::DiscoveryEngine::scan()).unwrap();
    /// assert!(report.ai_clis.len() + report.vcs.len() + report.package_managers.len() > 0);
    /// ```
    pub async fn scan() -> Result<DiscoveryReport> {
        let ai_clis = vec!["gemini", "claude", "qwen", "ollama"];
        let vcs = vec!["git"];
        let package_managers = vec!["npm", "pnpm", "yarn", "cargo"];

        let mut report = DiscoveryReport {
            timestamp: Utc::now(),
            ai_clis: Vec::new(),
            vcs: Vec::new(),
            package_managers: Vec::new(),
        };

        for cli in ai_clis {
            report.ai_clis.push(Self::check_tool(cli).await);
        }

        for v in vcs {
            report.vcs.push(Self::check_tool(v).await);
        }

        for pm in package_managers {
            report.package_managers.push(Self::check_tool(pm).await);
        }

        Ok(report)
    }

    /// Persist a discovery report as pretty-printed JSON at the user's config location.
    ///
    /// The report is written to `discovery.json` inside the application config directory
    /// (typically `~/.config/bl1nk-agents-manager/discovery.json`). The function ensures
    /// the config directory exists, serializes the report to pretty JSON, and writes the file.
    /// Returns an error if the config directory cannot be determined or created, if serialization fails,
    /// or if the file cannot be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Utc;
    /// use tokio::runtime::Runtime;
    ///
    /// let report = DiscoveryReport {
    ///     timestamp: Utc::now(),
    ///     ai_clis: vec![],
    ///     vcs: vec![],
    ///     package_managers: vec![],
    /// };
    ///
    /// let rt = Runtime::new().unwrap();
    /// rt.block_on(async {
    ///     DiscoveryEngine::save(&report).unwrap();
    /// });
    /// ```
    pub async fn save(report: &DiscoveryReport) -> Result<()> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir).await
            .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

        let report_path = config_dir.join("discovery.json");
        let content = serde_json::to_string_pretty(report)
            .context("Failed to serialize discovery report")?;

        fs::write(&report_path, content).await
            .with_context(|| format!("Failed to write discovery report to: {:?}", report_path))?;

        tracing::info!("✅ Discovery report saved to: {:?}", report_path);
        Ok(())
    }

    /// Returns the path to the user's configuration directory for the manager.
    ///
    /// The function attempts to read the `HOME` environment variable and falls back
    /// to `USERPROFILE` on Windows. On success it returns `<home>/.config/bl1nk-agents-manager`.
    ///
    /// # Errors
    ///
    /// Returns an error if neither `HOME` nor `USERPROFILE` is set in the environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use std::path::PathBuf;
    /// // Set a deterministic HOME for the example
    /// env::set_var("HOME", "/tmp/testhome");
    /// let dir = crate::system::discovery::get_config_dir().unwrap();
    /// assert_eq!(dir, PathBuf::from("/tmp/testhome").join(".config/bl1nk-agents-manager"));
    /// ```
    fn get_config_dir() -> Result<PathBuf> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .context("Could not find home directory")?;

        Ok(PathBuf::from(home).join(".config/bl1nk-agents-manager"))
    }

    /// Builds a ToolInfo for the given tool name by locating its binary and probing its version.
    ///
    /// If the tool's binary is found, `available` will be `true`, `path` will contain the first
    /// discovered binary path as a string, and `version` will contain the output of a version probe
    /// if one could be determined; otherwise `available` is `false` and `version` and `path` are `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::system::discovery::DiscoveryEngine;
    /// # #[tokio::test]
    /// # async fn _example_check_tool() {
    /// let info = DiscoveryEngine::check_tool("git").await;
    /// assert_eq!(info.name, "git");
    /// # }
    /// ```
    async fn check_tool(name: &str) -> ToolInfo {
        let path = Self::find_binary(name).await;
        let available = path.is_some();
        let version = if let Some(ref p) = path {
            Self::get_version(p).await
        } else {
            None
        };

        ToolInfo {
            name: name.to_string(),
            version,
            available,
            path: path.map(|p| p.to_string_lossy().to_string()),
        }
    }

    /// Locate an executable by name in the system PATH and return its first matching path.
    ///
    /// Uses the platform-appropriate lookup command (`which` on Unix-like systems, `where` on Windows)
    /// and returns the first path when multiple matches are reported.
    ///
    /// # Examples
    ///
    /// ```
    /// // Synchronously run the async helper to demonstrate usage in docs.
    /// let path_opt = tokio::runtime::Runtime::new()
    ///     .unwrap()
    ///     .block_on(async { crate::system::discovery::find_binary("git").await });
    ///
    /// match path_opt {
    ///     Some(p) => println!("Found: {}", p.display()),
    ///     None => println!("Not found"),
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// `Some(PathBuf)` with the first matching executable path, `None` if no executable was found.
    async fn find_binary(name: &str) -> Option<PathBuf> {
        let cmd = if cfg!(windows) { "where" } else { "which" };
        let output = Command::new(cmd).arg(name).output().await.ok()?;

        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                // 'where' on Windows might return multiple paths, take the first one
                let first_path = path_str.lines().next().unwrap_or(&path_str);
                return Some(PathBuf::from(first_path));
            }
        }
        None
    }

    /// Attempts to determine a binary's version by running it with common version flags.
    ///
    /// Tries `--version` first, then `-v`, and returns the first non-empty stdout output as a trimmed string. Returns `None` if neither flag produces a usable version string or if the process cannot be executed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// # async {
    /// let path = PathBuf::from("/usr/bin/git");
    /// let version = tokio::runtime::Runtime::new().unwrap().block_on(async {
    ///     crate::system::discovery::get_version(&path).await
    /// });
    /// println!("version: {:?}", version);
    /// # };
    /// ```
    async fn get_version(path: &PathBuf) -> Option<String> {
        // Most tools support --version
        let output = Command::new(path).arg("--version").output().await.ok()?;

        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version_str.is_empty() {
                return Some(version_str);
            }
        }

        // Some tools might use -v
        let output = Command::new(path).arg("-v").output().await.ok()?;
        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version_str.is_empty() {
                return Some(version_str);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_serialization() {
        let report = DiscoveryReport {
            timestamp: Utc::now(),
            ai_clis: vec![ToolInfo {
                name: "gemini".to_string(),
                version: Some("1.0.0".to_string()),
                available: true,
                path: Some("/usr/bin/gemini".to_string()),
            }],
            vcs: vec![ToolInfo {
                name: "git".to_string(),
                version: Some("2.34.1".to_string()),
                available: true,
                path: Some("/usr/bin/git".to_string()),
            }],
            package_managers: vec![ToolInfo {
                name: "cargo".to_string(),
                version: Some("1.56.0".to_string()),
                available: true,
                path: Some("/usr/bin/cargo".to_string()),
            }],
        };

        let json = serde_json::to_string(&report).expect("Failed to serialize");
        let deserialized: DiscoveryReport = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(deserialized.ai_clis.len(), 1);
        assert_eq!(deserialized.ai_clis[0].name, "gemini");
        assert_eq!(deserialized.vcs[0].name, "git");
        assert_eq!(deserialized.package_managers[0].name, "cargo");
    }
}
