mod godspeed;
mod mutate;
mod pareto;
mod roles;
mod router;
mod sekhmet;
mod spawn;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "xbgst", about = "Grok Bot native xbgst — no xask, no Claude")]
struct Cli {
    #[arg(long, default_value = "config/xbgst.toml")]
    config: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Route {
        #[command(subcommand)]
        action: RouteCmd,
    },
    Mutate {
        #[arg(long, default_value = ".")]
        crate_dir: PathBuf,
        #[arg(long, default_value_t = 4)]
        max: usize,
    },
    Sekhmet {
        #[arg(long)]
        task: String,
        #[arg(long)]
        dry_run: bool,
    },
    Spawn {
        #[arg(long)]
        role: String,
        #[arg(long)]
        node: Option<String>,
        #[arg(last = true, required = true)]
        argv: Vec<String>,
    },
}

#[derive(Subcommand)]
enum RouteCmd {
    Show,
    Set { role: String, lane: String },
    DryRun { #[arg(long)] role: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Route { action } => match action {
            RouteCmd::Show => print!("{}", router::load(&cli.config)?.display()),
            RouteCmd::Set { role, lane } => {
                let mut table = router::load(&cli.config)?;
                table.set(&role, &lane)?;
                table.save(&cli.config)?;
                println!("{role} -> {lane}");
            }
            RouteCmd::DryRun { role } => {
                let table = router::load(&cli.config)?;
                match table.lane_for(&role) {
                    Some(lane) => println!("role={role} mailbox=native lane={lane}"),
                    None => {
                        eprintln!("unknown role: {role}");
                        std::process::exit(2);
                    }
                }
            }
        },
        Cmd::Mutate { crate_dir, max } => print!("{}", mutate::run(&crate_dir, max)?),
        Cmd::Sekhmet { task, dry_run } => {
            println!("{}", sekhmet::plan_run(&task, dry_run)?);
        }
        Cmd::Spawn { role, node, argv } => {
            print!("{}", spawn::plan(&spawn::SpawnSpec { role, argv, node })?);
        }
    }
    Ok(())
}
