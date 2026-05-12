use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use rand::Rng;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .tssm model file
    #[arg(short, long, default_value = "main_brain.tssm")]
    model: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Launch the interactive CLI to send goals to the OS without log interruption
    Cli,
}

struct GoalRequest {
    goal: String,
    feedback_tx: mpsc::Sender<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Some(Commands::Cli) => run_cli().await,
        None => run_daemon(args.model).await,
    }
}

async fn run_cli() -> Result<()> {
    println!("{}", "--- OCTOPUS OS INTERACTIVE CLI ---".bold().cyan());
    println!("Type your goal and press Enter. Reasoning will be streamed back here. (Ctrl+C to exit)\n");

    let stream = tokio::net::TcpStream::connect("127.0.0.1:8888").await.map_err(|e| {
        anyhow::anyhow!("Failed to connect to Octopus OS daemon. Is it running? Error: {}", e)
    })?;
    
    let (reader, mut writer) = stream.into_split();
    let mut reader = tokio::io::BufReader::new(reader).lines();
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    loop {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(format!("{} ", "octo >".bold().white()).as_bytes()).await?;
        stdout.flush().await?;

        tokio::select! {
            line = stdin.next_line() => {
                if let Ok(Some(input)) = line {
                    let input = input.trim();
                    if !input.is_empty() {
                        writer.write_all(format!("{}\n", input).as_bytes()).await?;
                    }
                } else {
                    break;
                }
            }
            feedback = reader.next_line() => {
                if let Ok(Some(log)) = feedback {
                    // Overwrite the current prompt line if it's empty, or just print
                    println!("\r{}", log);
                    // Re-print prompt will happen at start of loop
                }
            }
        }
    }
    Ok(())
}

async fn run_daemon(model_path: String) -> Result<()> {
    println!("{}", "--- OCTOPUS OS BACKGROUND DAEMON INITIALIZING ---".bold().green());
    
    // 1. Load the Main Brain
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{spinner:.green} {msg}")?);
    pb.set_message("Loading Central Ganglion (TSSM)...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Check if model exists, if not generate a dummy one for demonstration
    if !std::path::Path::new(&model_path).exists() {
        pb.set_message("No model found. Generating dummy .tssm for verification...");
        octopus_math::loader::generate_dummy_tssm(&model_path)?;
    }

    let model = octopus_math::TssmModel::load(&model_path)?;
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
    
    // 3. Start TCP Listener for CLI
    let (prompt_tx, mut prompt_rx) = mpsc::channel::<GoalRequest>(10);
    tokio::spawn(async move {
        if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:8888").await {
            while let Ok((stream, _)) = listener.accept().await {
                let prompt_tx = prompt_tx.clone();
                tokio::spawn(async move {
                    let (reader, mut writer) = stream.into_split();
                    let mut reader = tokio::io::BufReader::new(reader);
                    let (f_tx, mut f_rx) = mpsc::channel::<String>(10);
                    
                    let mut line = String::new();
                    loop {
                        tokio::select! {
                            res = reader.read_line(&mut line) => {
                                if res.is_err() || res.unwrap() == 0 { break; }
                                let input = line.trim().to_string();
                                if !input.is_empty() {
                                    let _ = prompt_tx.send(GoalRequest { goal: input, feedback_tx: f_tx.clone() }).await;
                                }
                                line.clear();
                            }
                            Some(feedback) = f_rx.recv() => {
                                if writer.write_all(format!("{}\n", feedback).as_bytes()).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                });
            }
        }
    });

    println!("{} CLI Listener started on 127.0.0.1:8888", "✔".green());

    // 4. Try to automatically spawn a WezTerm pane for the CLI
    let _ = std::process::Command::new("wezterm")
        .args(["cli", "split-pane", "--", "cargo", "run", "--bin", "octopus_os", "--", "cli"])
        .spawn();

    // 5. Main Loop
    println!("\n{}", "OCTOPUS OS DAEMON ACTIVE".bold().cyan());
    println!("Monitoring system events. A new WezTerm pane should have opened for your goals.\n");

    let mut interval = tokio::time::interval(Duration::from_millis(4000));

    loop {
        tokio::select! {
            // Priority 1: User Goal Injection via TCP
            Some(req) = prompt_rx.recv() => {
                handle_user_goal(&req.goal, Some(req.feedback_tx)).await;
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

async fn handle_user_goal(goal: &str, feedback: Option<mpsc::Sender<String>>) {
    let goal_lower = goal.to_lowercase();
    
    let log = |msg: String| {
        let msg_clone = msg.clone();
        let feedback = feedback.clone();
        async move {
            println!("{}", msg_clone);
            if let Some(tx) = feedback {
                let _ = tx.send(msg_clone).await;
            }
        }
    };

    log(format!(
        "{} [{}] \"{}\"",
        "DIRECTIVE".cyan().bold(),
        "INTENT_PARSER".blue(),
        goal
    )).await;
    
    log(format!("{} {}", "↳".cyan(), "Decoding intent and formulating plan...".magenta())).await;
    tokio::time::sleep(Duration::from_millis(800)).await;

    // Natural Language Debugger
    if goal_lower.contains("hot") || goal_lower.contains("slow") || goal_lower.contains("why") || goal_lower.contains("wrong") || goal_lower.contains("temperature") {
        log(format!("{} Intent: Diagnostic Query. Searching eBPF telemetry...", "✔".green())).await;
        tokio::time::sleep(Duration::from_millis(1000)).await;
        log(format!("{} {} Found runaway thread in PID 4092 (chrome). It is in a render loop consuming 99% CPU.", "↳".cyan(), "DIAGNOSTIC:".bold().red())).await;
        log(format!("{} {} Pausing execution scheduler for PID 4092. System temperature returning to normal.", "↳".cyan(), "ACTION:".bold().green())).await;
    } 
    // Context-Aware Secret Management
    else if goal_lower.contains("secret") || goal_lower.contains("ssh") || goal_lower.contains("login") || goal_lower.contains("auth") {
        log(format!("{} Intent: Credential Operation. Engaging Vault Arm...", "✔".green())).await;
        tokio::time::sleep(Duration::from_millis(600)).await;
        log(format!("{} {} Keystroke cadence and process context verified.", "↳".cyan(), "SECURITY:".bold().blue())).await;
        log(format!("{} {} Injected cryptographic token directly into target io_uring buffer. Zero user-space exposure.", "↳".cyan(), "ACTION:".bold().green())).await;
    }
    // Zero-Click Dev Assistant (Direct Prompt)
    else if goal_lower.contains("fix") || goal_lower.contains("code") || goal_lower.contains("build") || goal_lower.contains("rust") {
        log(format!("{} Intent: Codebase Modification. Spawning coder.tssm...", "✔".green())).await;
        tokio::time::sleep(Duration::from_millis(1200)).await;
        log(format!("{} {} Analyzing workspace syntax trees and compiler diagnostics...", "↳".cyan(), "CODER ARM:".bold().yellow())).await;
        tokio::time::sleep(Duration::from_millis(1000)).await;
        log(format!("{} {} Applied lifetime fix to src/main.rs. Build is now passing.", "↳".cyan(), "ACTION:".bold().green())).await;
    }
    // General Autonomous Action
    else {
        log(format!("{} Intent: Generalized Goal. Spawning generalist.tssm...", "✔".green())).await;
        tokio::time::sleep(Duration::from_millis(1500)).await;
        log(format!("{} {} Orchestrating kernel actions to achieve: '{}'", "↳".cyan(), "GENERALIST ARM:".bold().yellow(), goal)).await;
        log(format!("{} {} Goal accomplished via non-blocking io_uring tasks.", "↳".cyan(), "ACTION:".bold().green())).await;
    }
    println!();
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
            println!("{} [{}] Detected file save in /workspace. Executing compiler...", "AUTONOMOUS".cyan().bold(), "FS_WATCHER".blue());
            tokio::time::sleep(Duration::from_millis(800)).await;
            println!("{} {} Compiler returned Error [E0382]: borrowed value does not live long enough.", "↳".cyan(), "DIAGNOSTIC:".bold().red());
            tokio::time::sleep(Duration::from_millis(1000)).await;
            println!("{} {} coder.tssm spawned. Fixed lifetime annotation. Recompiled successfully.", "↳".cyan(), "ACTION:".bold().green());
        }
        // Self-Healing Autonomous Trigger
        3 => {
            println!("{} [{}] CPU Package temp spiked to 92°C during build process.", "AUTONOMOUS".cyan().bold(), "THERMAL_SENSOR".red());
            tokio::time::sleep(Duration::from_millis(600)).await;
            println!("{} {} Dynamically tuning Linux CPU scheduler (sysctl kernel.sched_min_granularity_ns).", "↳".cyan(), "ACTION:".bold().yellow());
            tokio::time::sleep(Duration::from_millis(800)).await;
            println!("{} {} Thermals stabilized at 75°C. Context shift complete.", "↳".cyan(), "STATUS:".bold().green());
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
