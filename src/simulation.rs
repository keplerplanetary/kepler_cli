use planety_core::{system_timestep, System};

use crate::{configsystem::Config, export::export_system_to_csv};

pub fn run_simulation(config: Config, initial_system: System) {
    let mut system = initial_system.clone();

    let mut time = 0.0;
    match export_system_to_csv(config.clone(), system.clone(), 0, time) {
        Ok(_) => {
            println!("Exported 0, time {time}s");
        }
        Err(e) => {
            println!("error while exporting {e}");
            return;
        }
    };
    for i in 1..config.steps + 1 {
        system = system_timestep(system, config.timestep);
        time += config.timestep;

        if i % config.export_step == 0 {
            match export_system_to_csv(config.clone(), system.clone(), i, time) {
                Ok(_) => {
                    println!("Exported {i}, time {time}s");
                }
                Err(e) => {
                    println!("error while exporting {e}");
                    return;
                }
            };
        }
    }
}
