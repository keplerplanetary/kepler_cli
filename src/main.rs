use crate::{simulation::run_simulation, types::Config};
use maths_rs::Vec2d;
use planety_core::{Body, System};

mod export;
mod simulation;
mod types;
fn main() {
    let config = Config {
        timestep: 86400.0, // one day
        steps: 1000,
        export_step: 100,
        export_file_name_prefix: "export_files/KeplerSimulation".to_owned(),
    };

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
    println!("Running Simulation 🚀");
    run_simulation(config, system);
    println!("Done 🥳");
}
