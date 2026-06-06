//! Encom OS first-boot wizard.
//!
//! Two modes, decided up front:
//!
//!   1. Portable — live USB, RAM-only, optional encrypted persistence. Cloud
//!      model adapters only (no Ollama).
//!   2. Full — installed system, all cloud adapters PLUS Ollama with an
//!      installer-time-downloaded default model.
//!
//! Phase 1 ships the CLI surface and produces a structured install plan that
//! later phases consume:
//!   - `run --mode <mode> [--out PATH]` writes `install.toml` to PATH (default
//!     `/etc/encom-os/install.toml`).
//!   - `plan` prints the plan without writing.
//! Phase 3 wires it to a GTK/iced GUI and the actual archinstall + Ollama
//! download.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "encom-firstboot", about = "First-boot wizard for Encom OS")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the wizard. Today this is non-interactive: pass `--mode`. Phase 3
    /// adds the GUI prompt path.
    Run {
        /// Skip prompts and write the indicated mode straight to disk.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// Override the install.toml output path. Useful for tests.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Print the generated install.toml without writing it.
    Plan {
        #[arg(long, value_enum, default_value_t = Mode::Full)]
        mode: Mode,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Mode {
    Portable,
    Full,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstallPlan {
    mode: Mode,
    providers: Vec<String>,
    ollama_default_model: Option<String>,
    /// Where the encom daemon will bind for IPC. Encom Phase 1 default is
    /// 127.0.0.1:8765 over TCP loopback.
    daemon_addr: String,
    /// Schema version of this file. Bump when the layout changes; phase-2
    /// shell + phase-4 settings panel both read this.
    schema_version: u32,
}

const SCHEMA_VERSION: u32 = 1;
const DEFAULT_DAEMON_ADDR: &str = "127.0.0.1:8765";

fn cloud_providers() -> Vec<String> {
    [
        "openai", "anthropic", "xai", "gemini", "mistral",
        "cohere", "groq", "deepseek", "perplexity",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn build_plan(mode: Mode) -> InstallPlan {
    InstallPlan {
        mode,
        providers: cloud_providers(),
        ollama_default_model: match mode {
            Mode::Portable => None,
            Mode::Full => Some("llama3.3".into()),
        },
        daemon_addr: DEFAULT_DAEMON_ADDR.into(),
        schema_version: SCHEMA_VERSION,
    }
}

fn default_config_path() -> PathBuf {
    PathBuf::from("/etc/encom-os/install.toml")
}

fn write_plan(plan: &InstallPlan, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let toml = toml::to_string_pretty(plan).context("serialising install plan")?;
    std::fs::write(path, toml).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run { mode, out } => {
            let mode = mode.unwrap_or(Mode::Full);
            let plan = build_plan(mode);
            let path = out.unwrap_or_else(default_config_path);
            tracing::info!(?mode, path = %path.display(), "writing install plan");
            write_plan(&plan, &path)?;
            println!("wrote {}", path.display());
            Ok(())
        }
        Cmd::Plan { mode } => {
            let plan = build_plan(mode);
            let toml = toml::to_string_pretty(&plan)?;
            print!("{toml}");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn portable_plan_omits_ollama() {
        let plan = build_plan(Mode::Portable);
        assert_eq!(plan.mode, Mode::Portable);
        assert!(plan.ollama_default_model.is_none());
        assert!(plan.providers.contains(&"anthropic".to_string()));
        assert_eq!(plan.daemon_addr, DEFAULT_DAEMON_ADDR);
        assert_eq!(plan.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn full_plan_has_default_local_model() {
        let plan = build_plan(Mode::Full);
        assert_eq!(plan.mode, Mode::Full);
        assert_eq!(plan.ollama_default_model.as_deref(), Some("llama3.3"));
    }

    #[test]
    fn write_then_read_round_trip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested").join("install.toml");
        let plan = build_plan(Mode::Full);
        write_plan(&plan, &path).unwrap();

        let raw = std::fs::read_to_string(&path).unwrap();
        let parsed: InstallPlan = toml::from_str(&raw).unwrap();
        assert_eq!(parsed.mode, Mode::Full);
        assert_eq!(parsed.daemon_addr, plan.daemon_addr);
        assert_eq!(parsed.providers, plan.providers);
        assert_eq!(parsed.schema_version, SCHEMA_VERSION);
    }
}
