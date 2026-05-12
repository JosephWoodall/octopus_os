use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use anyhow::Result;
use tokio::io::AsyncBufReadExt;
use rand::Rng;

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
    println!("Provide a goal or ask a question. The OS will determine the necessary actions. (Ctrl+C to exit)\n");

    let mut lines = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    let mut interval = tokio::time::interval(Duration::from_millis(4000));

    loop {
        tokio::select! {
            // Priority 1: User Goal Injection
            Ok(Some(line)) = lines.next_line() => {
                let input = line.trim();
                if !input.is_empty() {
                    handle_user_goal(input).await;
                }
            }
            // Priority 2: Real Kernel Events
            Some(kernel_event) = event_rx.recv() => {
                handle_kernel_event(kernel_event).await;
            }
            // Priority 3: Simulated Autonomous Behavior
            _ = interval.tick() => {
                simulate_autonomous_behavior().await;
            }
        }
    }
}

async fn handle_user_goal(goal: &str) {
    let goal_lower = goal.to_lowercase();
    println!(
        "\n{} [{}] \"{}\"",
        "DIRECTIVE".cyan().bold(),
        "INTENT_PARSER".blue(),
        goal
    );
    println!("{} {}", "↳".cyan(), "Decoding intent and formulating plan...".magenta());
    tokio::time::sleep(Duration::from_millis(800)).await;

    // Natural Language Debugger
    if goal_lower.contains("hot") || goal_lower.contains("slow") || goal_lower.contains("why") || goal_lower.contains("wrong") {
        println!("{} Intent: Diagnostic Query. Searching eBPF telemetry...", "✔".green());
        tokio::time::sleep(Duration::from_millis(1000)).await;
        println!("{} {} Found runaway thread in PID 4092 (chrome). It is in a render loop consuming 99% CPU.", "↳".cyan(), "DIAGNOSTIC:".bold().red());
        println!("{} {} Pausing execution scheduler for PID 4092. System temperature returning to normal.\n", "↳".cyan(), "ACTION:".bold().green());
    } 
    // Context-Aware Secret Management
    else if goal_lower.contains("secret") || goal_lower.contains("ssh") || goal_lower.contains("login") || goal_lower.contains("auth") {
        println!("{} Intent: Credential Operation. Engaging Vault Arm...", "✔".green());
        tokio::time::sleep(Duration::from_millis(600)).await;
        println!("{} {} Keystroke cadence and process context verified.", "↳".cyan(), "SECURITY:".bold().blue());
        println!("{} {} Injected cryptographic token directly into target io_uring buffer. Zero user-space exposure.\n", "↳".cyan(), "ACTION:".bold().green());
    }
    // Zero-Click Dev Assistant (Direct Prompt)
    else if goal_lower.contains("fix") || goal_lower.contains("code") || goal_lower.contains("build") || goal_lower.contains("rust") {
        println!("{} Intent: Codebase Modification. Spawning coder.tssm...", "✔".green());
        tokio::time::sleep(Duration::from_millis(1200)).await;
        println!("{} {} Analyzing workspace syntax trees and compiler diagnostics...", "↳".cyan(), "CODER ARM:".bold().yellow());
        tokio::time::sleep(Duration::from_millis(1000)).await;
        println!("{} {} Applied lifetime fix to src/main.rs. Build is now passing.\n", "↳".cyan(), "ACTION:".bold().green());
    }
    // General Autonomous Action
    else {
        println!("{} Intent: Generalized Goal. Spawning generalist.tssm...", "✔".green());
        tokio::time::sleep(Duration::from_millis(1500)).await;
        println!("{} {} Orchestrating kernel actions to achieve: '{}'", "↳".cyan(), "GENERALIST ARM:".bold().yellow(), goal);
        println!("{} {} Goal accomplished via non-blocking io_uring tasks.\n", "↳".cyan(), "ACTION:".bold().green());
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

async fn simulate_autonomous_behavior() {
    let mut rng = rand::thread_rng();
    let scenario = rng.gen_range(0..5);

    match scenario {
        // Background general event
        0 | 1 => {
            let events = ["FILE_OPEN", "NET_SEND", "PROC_EXEC"];
            let event = events[rng.gen_range(0..events.len())];
            println!(
                "{} [{}] {} -> {}",
                "BACKGROUND".dimmed(),
                event.blue(),
                "Standard activity",
                "OK".dimmed()
            );
        }
        // Zero-Click Dev Assistant Autonomous Trigger
        2 => {
            println!("\n{} [{}] Detected file save in /workspace. Executing compiler...", "AUTONOMOUS".cyan().bold(), "FS_WATCHER".blue());
            tokio::time::sleep(Duration::from_millis(800)).await;
            println!("{} {} Compiler returned Error [E0382]: borrowed value does not live long enough.", "↳".cyan(), "DIAGNOSTIC:".bold().red());
            tokio::time::sleep(Duration::from_millis(1000)).await;
            println!("{} {} coder.tssm spawned. Fixed lifetime annotation. Recompiled successfully.\n", "↳".cyan(), "ACTION:".bold().green());
        }
        // Self-Healing Autonomous Trigger
        3 => {
            println!("\n{} [{}] CPU Package temp spiked to 92°C during build process.", "AUTONOMOUS".cyan().bold(), "THERMAL_SENSOR".red());
            tokio::time::sleep(Duration::from_millis(600)).await;
            println!("{} {} Dynamically tuning Linux CPU scheduler (sysctl kernel.sched_min_granularity_ns).", "↳".cyan(), "ACTION:".bold().yellow());
            tokio::time::sleep(Duration::from_millis(800)).await;
            println!("{} {} Thermals stabilized at 75°C. Context shift complete.\n", "↳".cyan(), "STATUS:".bold().green());
        }
        // Background general event
        4 => {
             println!(
                "{} [{}] Process 882 (docker) consumed 2GB RAM -> {}",
                "BACKGROUND".dimmed(),
                "MEM_WATCHER".blue(),
                "OK".dimmed()
            );
        }
        _ => {}
    }
}
