// SPDX-License-Identifier: AGPL-3.0-or-later
//! Personal Sysadmin (PSA) - AI-assisted Linux system administration toolkit
//!
//! A learning system administration tool that:
//! - Monitors and manages system resources (Sysinternals-like)
//! - Learns from solutions via miniKanren reasoning
//! - Searches forums and documentation for fixes
//! - Shares solutions across device mesh via P2P
//! - Uses local SLM with Claude fallback for complex issues

use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod reasoning;
mod storage;
mod cache;
mod tools;
mod ai;
mod forum;
mod p2p;

#[derive(Parser)]
#[command(name = "psa")]
#[command(author = "hyperpolymath")]
#[command(version)]
#[command(about = "Personal Sysadmin - AI-assisted system administration")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Process management (like Process Explorer)
    #[command(alias = "ps")]
    Process {
        #[command(subcommand)]
        action: ProcessAction,
    },

    /// Network diagnostics (like TCPView)
    #[command(alias = "net")]
    Network {
        #[command(subcommand)]
        action: NetworkAction,
    },

    /// Disk and storage management
    #[command(alias = "df")]
    Disk {
        #[command(subcommand)]
        action: DiskAction,
    },

    /// Service management (like Autoruns)
    #[command(alias = "svc")]
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },

    /// Security scanning and hardening
    #[command(alias = "sec")]
    Security {
        #[command(subcommand)]
        action: SecurityAction,
    },

    /// Diagnose a problem (AI-assisted)
    Diagnose {
        /// Problem description
        problem: String,
        /// Use only local SLM (no Claude)
        #[arg(long)]
        local_only: bool,
    },

    /// Search for solutions in knowledge base and forums
    Search {
        /// Search query
        query: String,
        /// Search online forums too
        #[arg(long)]
        online: bool,
    },

    /// Learn from a solution (store in knowledge base)
    Learn {
        /// Problem category
        #[arg(short, long)]
        category: String,
        /// Solution file or stdin
        solution: Option<String>,
    },

    /// P2P mesh commands
    Mesh {
        #[command(subcommand)]
        action: MeshAction,
    },

    /// Interactive monitoring dashboard
    Monitor,

    /// Show system health summary
    Health,
}

#[derive(Subcommand)]
enum ProcessAction {
    /// List all processes with resource usage
    List {
        /// Sort by (cpu, mem, pid, name)
        #[arg(short, long, default_value = "cpu")]
        sort: String,
        /// Show only top N processes
        #[arg(short = 'n', long)]
        top: Option<usize>,
    },
    /// Show process tree
    Tree,
    /// Find processes by name or pattern
    Find { pattern: String },
    /// Show detailed info for a process
    Info { pid: u32 },
    /// Kill a process
    Kill { pid: u32 },
    /// Watch a process for anomalies (uses ESN/LSM)
    Watch { pid: u32 },
}

#[derive(Subcommand)]
enum NetworkAction {
    /// Show all connections (like TCPView)
    Connections {
        /// Filter by state (LISTEN, ESTABLISHED, etc.)
        #[arg(short, long)]
        state: Option<String>,
    },
    /// Show listening ports
    Listen,
    /// Show bandwidth usage per process
    Bandwidth,
    /// Test connectivity
    Ping { host: String },
    /// DNS diagnostics
    Dns { domain: String },
    /// Watch for suspicious connections (anomaly detection)
    Watch,
}

#[derive(Subcommand)]
enum DiskAction {
    /// Show disk usage summary
    Usage,
    /// Find large files
    Large {
        /// Minimum size (e.g., "100M", "1G")
        #[arg(short, long, default_value = "100M")]
        min_size: String,
        /// Path to search
        #[arg(default_value = ".")]
        path: String,
    },
    /// Show disk I/O per process
    Io,
    /// Find duplicate files
    Duplicates { path: String },
    /// Analyze disk health (SMART)
    Health,
}

#[derive(Subcommand)]
enum ServiceAction {
    /// List all services
    List {
        /// Show only failed
        #[arg(long)]
        failed: bool,
    },
    /// Show service status
    Status { name: String },
    /// List startup items (like Autoruns)
    Startup,
    /// Analyze service dependencies
    Deps { name: String },
}

#[derive(Subcommand)]
enum SecurityAction {
    /// Scan for common vulnerabilities
    Scan,
    /// Check file permissions
    Perms { path: String },
    /// Audit system configuration
    Audit,
    /// Check for rootkits
    Rootkit,
    /// List open ports and assess exposure
    Exposure,
}

#[derive(Subcommand)]
enum MeshAction {
    /// Discover other PSA instances on network
    Discover,
    /// Join a mesh network
    Join { peer: String },
    /// Share a solution with the mesh
    Share { solution_id: String },
    /// Sync knowledge base with peers
    Sync,
    /// Show mesh status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = match cli.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| log_level.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize storage and cache connections
    let storage = storage::Storage::new().await?;
    let cache = cache::Cache::new().await?;

    match cli.command {
        Commands::Process { action } => {
            tools::process::handle(action, &storage, &cache).await?;
        }
        Commands::Network { action } => {
            tools::network::handle(action, &storage, &cache).await?;
        }
        Commands::Disk { action } => {
            tools::disk::handle(action, &storage, &cache).await?;
        }
        Commands::Service { action } => {
            tools::service::handle(action, &storage, &cache).await?;
        }
        Commands::Security { action } => {
            tools::security::handle(action, &storage, &cache).await?;
        }
        Commands::Diagnose { problem, local_only } => {
            ai::diagnose(&problem, local_only, &storage, &cache).await?;
        }
        Commands::Search { query, online } => {
            forum::search(&query, online, &storage, &cache).await?;
        }
        Commands::Learn { category, solution } => {
            reasoning::learn(&category, solution, &storage).await?;
        }
        Commands::Mesh { action } => {
            p2p::handle(action, &storage, &cache).await?;
        }
        Commands::Monitor => {
            tools::monitor::run(&storage, &cache).await?;
        }
        Commands::Health => {
            tools::health::show(&storage, &cache).await?;
        }
    }

    Ok(())
}
