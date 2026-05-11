use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .tssm model file
    #[arg(short, long, default_value = "main_brain.tssm")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    println!("{}", "--- OCTOPUS OS INITIALIZING ---".bold().green());
    
    // 1. Load the Main Brain
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{spinner:.green} {msg}")?);
    pb.set_message("Loading Central Ganglion (TSSM)...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Check if model exists, if not generate a dummy one for demonstration
    if !std::path::Path::new(&args.model).exists() {
        pb.set_message("No model found. Generating dummy .tssm for verification...");
        octopus_math::loader::generate_dummy_tssm(&args.model)?;
    }

    let model = octopus_math::TssmModel::load(&args.model)?;
    pb.finish_with_message(format!("{} Central Ganglion Loaded: {:?}", "✔".green(), model.header));

    // 2. Initialize Sensory Tentacles (eBPF)
    println!("{}", "Injecting eBPF Sensory Tentacles into Kernel...".dimmed());
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(100);
    
    // Attempt to load real sensory tentacles. If it fails (e.g. no root, no BPF),
    // we continue with just simulated activity.
    let _sensory = match octopus_ebpf::SensorySystem::init(event_tx).await {
        Ok(s) => {
            println!("{} Sensory System Active.", "✔".green());
            Some(s)
        },
        Err(e) => {
            println!("{} Sensory System Bypassed ({}). Using Demo Mode only.", "⚠".yellow(), e);
            None
        }
    };
    
    // 3. Main Loop
    println!("\n{}", "OCTOPUS OS ACTIVE".bold().cyan());
    println!("Monitoring system events. Press Ctrl+C to shutdown.\n");

    loop {
        tokio::select! {
            // Priority 1: Real Kernel Events (via eBPF)
            Some(kernel_event) = event_rx.recv() => {
                handle_kernel_event(kernel_event).await;
            }
            // Priority 2: Simulated Events (for demo/verification)
            _ = tokio::time::sleep(Duration::from_millis(3000)) => {
                simulate_activity().await;
            }
        }
    }
}

async fn handle_kernel_event(event: octopus_ebpf_common::KernelEvent) {
    println!(
        "{} [{}] PID: {} -> {}",
        "REAL-SENSORY".green().bold(),
        "KERNEL_EVENT".blue(),
        event.pid,
        "EVALUATING".magenta()
    );
}

async fn simulate_activity() {
    let events = ["FILE_OPEN", "NET_SEND", "PROC_EXEC"];
    let event = events[rand::random::<usize>() % events.len()];
    println!(
        "{} [{}] {} -> {}",
        "DEMO-SENSORY".yellow().bold(),
        event.blue(),
        "Simulated system activity",
        "EVALUATING".magenta()
    );
}
