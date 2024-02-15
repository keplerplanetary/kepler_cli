use kepler_core::{system_timestep, System};
use maths_rs::num::Cast;

use crate::{
    configsystem::Config, export::export_system_snapshot_to_csv,
    export::export_system_to_csv_by_body,
};

pub fn run_simulation(config: Config, initial_system: System) {
    let mut system = initial_system.clone();

    let mut time = 0.0;

    match export_system_snapshot_to_csv(config.clone(), system.clone(), 0, time) {
        Ok(_) => {
            tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
        }
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "error while exporting {e}");

            println!("error while exporting {e}");
            return;
        }
    };
    match export_system_to_csv_by_body(config.clone(), system.clone(), 0, time) {
        Ok(_) => {
            tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
        }
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "error while exporting {e}");

            println!("error while exporting {e}");
            return;
        }
    };
    for i in 1..config.steps + 1 {
        system = system_timestep(system, config.timestep);
        time += config.timestep;

        if i % config.export_step == 0 {
            match export_system_snapshot_to_csv(config.clone(), system.clone(), i, time) {
                Ok(_) => {
                    tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
                }
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "error while exporting {e}");
                    return;
                }
            };
            match export_system_to_csv_by_body(config.clone(), system.clone(), i, time) {
                Ok(_) => {
                    tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
                }
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "error while exporting {e}");
                    return;
                }
            };

            let human_readable_time = format_time(time.as_u64());
            let progress = i.as_f64() / config.steps.as_f64() * 100.0;
            let energy: f64 = kepler_core::calculate_system_energy(system.clone());

            tracing::event!(
                tracing::Level::INFO,
                "Progress: {:.2}%, time: {}, energy: {}",
                progress,
                human_readable_time,
                energy
            );
        }
    }
}

fn format_time(time: u64) -> String {
    if time < 60 {
        format!("{:.2}s", time)
    } else if 60 <= time && time < 3600 {
        // 1 minute to 1 hour
        format!("{:.2}min", time.as_f64() / 60.0)
    } else if 3600 <= time && time < 68400 {
        // 1 hour to 1 day
        format!("{:.2}h", time.as_f64() / 3600.0)
    } else if 68400 <= time && time < 2592000 {
        // 1 day to 1 month
        format!("{:.2}months", time.as_f64() / 68400.0)
    } else if 2592000 <= time && time < 31104000 {
        // 1 month to 1 year
        format!("{:.2}y", time.as_f64() / 2592000.0)
    } else {
        return "?".to_owned();
    }
}
