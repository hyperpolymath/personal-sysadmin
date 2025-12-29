// SPDX-License-Identifier: AGPL-3.0-or-later
//! System health summary

use anyhow::Result;
use crate::storage::Storage;
use crate::cache::Cache;

pub async fn show(_storage: &Storage, _cache: &Cache) -> Result<()> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    println!("System Health Summary");
    println!("{}", "=".repeat(50));

    // Overall status
    let mut issues = vec![];

    // CPU check
    let cpu = sys.global_cpu_info().cpu_usage();
    if cpu > 90.0 {
        issues.push(format!("High CPU: {:.1}%", cpu));
    }

    // Memory check
    let mem_pct = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;
    if mem_pct > 90.0 {
        issues.push(format!("High Memory: {:.1}%", mem_pct));
    }

    // Disk check
    for disk in sys.disks() {
        let used_pct = 100.0 - (disk.available_space() as f64 / disk.total_space() as f64 * 100.0);
        if used_pct > 90.0 {
            issues.push(format!("Disk {} at {:.1}%", disk.mount_point().display(), used_pct));
        }
    }

    // Load check
    let load = sys.load_average();
    let cpu_count = sys.cpus().len() as f64;
    if load.one > cpu_count * 2.0 {
        issues.push(format!("High load: {:.2}", load.one));
    }

    // Display status
    if issues.is_empty() {
        println!("\n✓ System is healthy");
        println!("\n  CPU:    {:.1}%", cpu);
        println!("  Memory: {:.1}%", mem_pct);
        println!("  Load:   {:.2} {:.2} {:.2}", load.one, load.five, load.fifteen);
    } else {
        println!("\n⚠ Issues detected:");
        for issue in &issues {
            println!("  • {}", issue);
        }
    }

    // Check for failed services
    let output = tokio::process::Command::new("systemctl")
        .args(["--user", "--failed", "--no-legend"])
        .output()
        .await?;
    let failed = String::from_utf8_lossy(&output.stdout);
    if !failed.trim().is_empty() {
        println!("\n⚠ Failed services:");
        for line in failed.lines() {
            println!("  • {}", line.split_whitespace().next().unwrap_or(""));
        }
    }

    Ok(())
}
