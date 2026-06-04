//! Encom OS first-boot wizard.
//!
//! Two modes, decided up front:
//!
//!   1. Portable — live USB, RAM-only, optional encrypted persistence. Cloud
//!      model adapters only (no Ollama).
//!   2. Full — installed system, all cloud adapters PLUS Ollama with an
//!      installer-time-downloaded default model.
//!
//! Phase 0 ships the CLI surface and writes a config skeleton. Phase 3 wires
//! it to a GTK/iced GUI and the actual archinstall + Ollama download.

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "encom-firstboot", about = "First-boot wizard for Encom OS")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the wizard interactively (TTY for now; GUI in Phase 3).
    Run {
        /// Skip prompts, write the indicated mode straight to disk.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
    },
    /// Print the generated /etc/encom-os/install.toml without writing it.
    Plan,
}

#[derive(Copy, Clone, Debug, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Mode {
    Portable,
    Full,
}

#[derive(Serialize)]
struct InstallPlan {
    mode: Mode,
    providers: Vec<&'static str>,
    ollama_default_model: Option<String>,
}

fn cloud_providers() -> Vec<&'static str> {
    vec![
        "openai", "anthropic", "xai", "gemini", "mistral",
        "cohere", "groq", "deepseek", "perplexity",
    ]
}

fn build_plan(mode: Mode) -> InstallPlan {
    InstallPlan {
        mode,
        providers: cloud_providers(),
        ollama_default_model: match mode {
            Mode::Portable => None,
            Mode::Full => Some("llama3.3".into()),
        },
    }
}

fn config_path() -> PathBuf {
    PathBuf::from("/etc/encom-os/install.toml")
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run { mode } => {
            let mode = mode.unwrap_or(Mode::Full);
            let plan = build_plan(mode);
            let toml = toml::to_string_pretty(&plan)?;
            tracing::info!(?mode, "writing install plan");
            // Phase 0: print only. Phase 3: write to /etc/encom-os/install.toml
            // and trigger archinstall + Ollama model pull.
            println!("{toml}");
            Ok(())
        }
        Cmd::Plan => {
            let plan = build_plan(Mode::Full);
            println!("# would write to: {}", config_path().display());
            println!("{}", toml::to_string_pretty(&plan)?);
            Ok(())
        }
    }
}
