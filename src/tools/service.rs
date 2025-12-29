// SPDX-License-Identifier: AGPL-3.0-or-later
//! Service management tools (like Autoruns)

use anyhow::Result;
use crate::storage::Storage;
use crate::cache::Cache;
use crate::ServiceAction;

pub async fn handle(action: ServiceAction, _storage: &Storage, _cache: &Cache) -> Result<()> {
    match action {
        ServiceAction::List { failed } => list_services(failed).await?,
        ServiceAction::Status { name } => show_status(&name).await?,
        ServiceAction::Startup => list_startup().await?,
        ServiceAction::Deps { name } => show_deps(&name).await?,
    }
    Ok(())
}

async fn list_services(failed_only: bool) -> Result<()> {
    let args = if failed_only {
        vec!["--user", "--failed"]
    } else {
        vec!["--user", "list-units", "--type=service"]
    };

    let output = tokio::process::Command::new("systemctl")
        .args(&args)
        .output()
        .await?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

async fn show_status(name: &str) -> Result<()> {
    let output = tokio::process::Command::new("systemctl")
        .args(["--user", "status", name])
        .output()
        .await?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

async fn list_startup() -> Result<()> {
    println!("Startup items (Autoruns equivalent):");
    println!("{}", "=".repeat(50));

    // XDG autostart
    println!("\n[XDG Autostart]");
    for dir in &[
        "/etc/xdg/autostart",
        &format!("{}/.config/autostart", std::env::var("HOME").unwrap_or_default()),
    ] {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                println!("  {}", entry.path().display());
            }
        }
    }

    // Systemd user units
    println!("\n[Systemd User Units]");
    let output = tokio::process::Command::new("systemctl")
        .args(["--user", "list-unit-files", "--state=enabled"])
        .output()
        .await?;
    println!("{}", String::from_utf8_lossy(&output.stdout));

    // Profile scripts
    println!("\n[Shell Profile Scripts]");
    let home = std::env::var("HOME").unwrap_or_default();
    for script in &[
        format!("{}/.profile", home),
        format!("{}/.bashrc", home),
        format!("{}/.bash_profile", home),
        format!("{}/.zshrc", home),
    ] {
        if std::path::Path::new(script).exists() {
            println!("  {} (exists)", script);
        }
    }

    Ok(())
}

async fn show_deps(name: &str) -> Result<()> {
    println!("Dependencies for {}:", name);

    let output = tokio::process::Command::new("systemctl")
        .args(["--user", "list-dependencies", name])
        .output()
        .await?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
