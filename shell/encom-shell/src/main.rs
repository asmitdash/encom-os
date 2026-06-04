//! Encom Shell — the agent surface for Encom OS.
//!
//! Phase 0 is a stub: it logs that it would start a Wayland compositor and
//! exits. Phase 2 brings up a real smithay-based compositor with:
//!   - one privileged surface for the agent prompt (Super-key launcher)
//!   - notification ownership (the agent triages, not the user)
//!   - settings panel (model picker, skill manager, permissions)
//!   - app surfaces for legacy XWayland clients

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing::info!("encom-shell phase 0 stub — would start compositor here");
    Ok(())
}
