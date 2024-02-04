use crate::simulation::run_simulation;
use clap::Parser;
use configsystem::ConfigSystem;

mod configsystem;
mod error;
mod export;
mod simulation;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the toml file with the simulation details
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    match ConfigSystem::parse(args.filename) {
        Ok(configsystem) => {
            let system = configsystem.system;
            let config = configsystem.config;
            println!("⚙️ Config \n{:#?}", &config);
            println!("🪐 System \n{:#?}", &system);
            /*
            let system = System {
                bodies: vec![
                    Body {
                        mass: 1.989e30,
                        name: "Sun".to_owned(),
                        position: Vec2d { x: 0.0, y: 0.0 },
                        velocity: Vec2d { x: 0.0, y: 0.0 },
                    },
                    Body {
                        mass: 5.972e24,
                        name: "Kerbin".to_owned(),
                        position: Vec2d {
                            x: 1.495978707e11,
                            y: 0.0,
                        },
                        velocity: Vec2d { x: 0.0, y: 4e5 },
                    },
                ],
            };
            */
            println!("Running Simulation 🚀");
            run_simulation(config, system);
            println!("Done 🥳");
        }
        Err(e) => {
            println!("Error when generating config: {e}");
        }
    };
}
