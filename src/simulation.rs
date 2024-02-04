use planety_core::{system_timestep, System};

use crate::{export::export_system_to_csv, types::Config};

pub fn run_simulation(config: Config, initial_system: System) {
    let mut system = initial_system.clone();

    let mut time = 0.0;

    for i in 1..config.steps {
        system = system_timestep(system, config.timestep);
        time += config.timestep;

        if i % config.export_step == 0 {
            match export_system_to_csv(config.clone(), system.clone(), i, time) {
                Ok(_) => {}
                Err(e) => {
                    println!("error while exporting {e}");
                    return;
                }
            };
        }
    }

    println!("System final state: \n{system:#?}");
}
