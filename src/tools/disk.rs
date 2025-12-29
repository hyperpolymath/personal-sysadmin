// SPDX-License-Identifier: AGPL-3.0-or-later
//! Disk and storage management tools

use anyhow::Result;
use crate::storage::Storage;
use crate::cache::Cache;
use crate::DiskAction;

pub async fn handle(action: DiskAction, _storage: &Storage, _cache: &Cache) -> Result<()> {
    match action {
        DiskAction::Usage => show_usage().await?,
        DiskAction::Large { min_size, path } => find_large(&min_size, &path).await?,
        DiskAction::Io => show_io().await?,
        DiskAction::Duplicates { path } => find_duplicates(&path).await?,
        DiskAction::Health => show_health().await?,
    }
    Ok(())
}

async fn show_usage() -> Result<()> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_disks_list();

    println!("{:<20} {:>10} {:>10} {:>10} {:>6}", "FILESYSTEM", "SIZE", "USED", "AVAIL", "USE%");
    println!("{}", "-".repeat(60));

    for disk in sys.disks() {
        let total = disk.total_space();
        let avail = disk.available_space();
        let used = total - avail;
        let use_pct = (used as f64 / total as f64) * 100.0;

        println!(
            "{:<20} {:>10} {:>10} {:>10} {:>5.1}%",
            disk.mount_point().display(),
            format_size(total),
            format_size(used),
            format_size(avail),
            use_pct
        );
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1}T", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

async fn find_large(min_size: &str, path: &str) -> Result<()> {
    let min_bytes = parse_size(min_size)?;
    println!("Finding files larger than {} in {}...", min_size, path);

    // Would use walkdir crate for recursive search
    // For now, use find command
    let output = tokio::process::Command::new("find")
        .args([path, "-type", "f", "-size", &format!("+{}", min_size)])
        .output()
        .await?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim().to_uppercase();
    let (num, suffix) = s.split_at(s.len() - 1);
    let num: u64 = num.parse()?;

    Ok(match suffix {
        "K" => num * 1024,
        "M" => num * 1024 * 1024,
        "G" => num * 1024 * 1024 * 1024,
        "T" => num * 1024 * 1024 * 1024 * 1024,
        _ => num,
    })
}

async fn show_io() -> Result<()> {
    println!("Disk I/O by process (requires root for some info):");
    // Would use /proc/[pid]/io for per-process I/O
    Ok(())
}

async fn find_duplicates(path: &str) -> Result<()> {
    println!("Finding duplicate files in {}...", path);
    // Would hash files and group by hash
    Ok(())
}

async fn show_health() -> Result<()> {
    println!("Disk health (SMART data):");
    // Would use smartctl
    Ok(())
}
