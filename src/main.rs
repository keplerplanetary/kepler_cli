use crate::simulation::run_simulation;
use clap::Parser;
use configsystem::ConfigSystem;

mod configsystem;
mod error;
mod export;
mod simulation;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the toml file with the simulation details
    #[arg(short, long)]
    filename: String,
}

fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    match ConfigSystem::parse(args.filename) {
        Ok(configsystem) => {
            let system = configsystem.system;
            let config = configsystem.config;
            tracing::event!(tracing::Level::DEBUG, "⚙️ Config \n{:#?}", &config);
            tracing::event!(tracing::Level::DEBUG, "🪐 System \n{:#?}", &system);

            tracing::event!(tracing::Level::INFO, "Running Simulation 🚀");
            run_simulation(config, system);
            tracing::event!(tracing::Level::INFO, "Done 🥳");
        }
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Error when generating config: {e}");
        }
    };
}
