use kepler_core::{
    energy::{calculate_kinetic_energy, calculate_potential_energy, calculate_system_energy},
    mover::system_timestep,
    types::System,
};
use maths_rs::num::Cast;

use crate::{
    configsystem::Config,
    export::{
        export_system_parameters_to_csv, export_system_snapshot_to_csv,
        export_system_to_csv_by_body,
    },
    plot::{plot_total_energy, PlotDatum},
};

pub fn run_simulation(config: Config, initial_system: System) {
    let mut energy_plot_data: Vec<PlotDatum> = vec![];

    let mut system = initial_system.clone();

    let mut time = 0.0;
    if config.export_system_parameters_history {
        match export_system_parameters_to_csv(&config, &system, 0, time) {
            Ok(_) => {
                tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "error while exporting {e}");
                return;
            }
        };
    }
    if config.export_system_state {
        match export_system_snapshot_to_csv(&config, &system, 0, time) {
            Ok(_) => {
                tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "error while exporting {e}");

                println!("error while exporting {e}");
                return;
            }
        };
    }

    if config.export_body_history {
        match export_system_to_csv_by_body(&config, &system, 0, time) {
            Ok(_) => {
                tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "error while exporting {e}");

                println!("error while exporting {e}");
                return;
            }
        };
    }

    for i in 1..config.steps + 1 {
        system = system_timestep(system, config.timestep);
        time += config.timestep;

        if i % config.export_step == 0 {
            if config.plot_system {
                // save data for plotting
                let kinetic_energy = match config.plot_system_kinetic_energy {
                    true => Some(system.bodies.iter().map(calculate_kinetic_energy).sum()),
                    false => None,
                };
                let potential_energy = match config.plot_system_potential_energy {
                    true => Some(
                        system
                            .bodies
                            .iter()
                            .map(|body| {
                                system
                                    .bodies
                                    .iter()
                                    .map(|other| calculate_potential_energy(body, other))
                                    .sum::<f64>()
                            })
                            .sum::<f64>(),
                    ),
                    false => None,
                };
                energy_plot_data.push(PlotDatum {
                    time,
                    total_energy: calculate_system_energy(&system),
                    kinetic_energy,
                    potential_energy,
                });
            }

            // writing to file
            if config.export_system_parameters_history {
                match export_system_parameters_to_csv(&config, &system, i, time) {
                    Ok(_) => {
                        tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
                    }
                    Err(e) => {
                        tracing::event!(tracing::Level::ERROR, "error while exporting {e}");
                        return;
                    }
                };
            }
            if config.export_system_state {
                match export_system_snapshot_to_csv(&config, &system, i, time) {
                    Ok(_) => {
                        tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
                    }
                    Err(e) => {
                        tracing::event!(tracing::Level::ERROR, "error while exporting {e}");
                        return;
                    }
                };
            }

            if config.export_body_history {
                match export_system_to_csv_by_body(&config, &system, i, time) {
                    Ok(_) => {
                        tracing::event!(tracing::Level::DEBUG, "Exported 0, time {time}s");
                    }
                    Err(e) => {
                        tracing::event!(tracing::Level::ERROR, "error while exporting {e}");
                        return;
                    }
                };
            }

            let human_readable_time = format_time(time.as_u64());
            let progress = i.as_f64() / config.steps.as_f64() * 100.0;

            tracing::event!(
                tracing::Level::INFO,
                "Progress: {:.2}%, time: {}",
                progress,
                human_readable_time,
            );
        }
    }

    if config.plot_system {
        match plot_total_energy(energy_plot_data, &config) {
            Ok(_) => {
                tracing::event!(tracing::Level::INFO, "Plotted total energy");
            }
            Err(e) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    "Error while plotting total energy: {e}"
                );
            }
        };
    }
}

/// This function formats time in seconds in a human readable format.
/// It assumes one month is 30 days and one year is 12 * 30 days,
/// so it's not extremely precise.
fn format_time(time: u64) -> String {
    let one_min = 60;
    let one_hour = one_min * 60; // 3600 seconds
    let one_day = one_hour * 24; // 86_400 seconds
    let one_month = one_day * 30; // 2_592_000 seconds
    let one_year = one_month * 12; // 31_104_000 seconds
    if time < one_min {
        format!("{:.2}s", time)
    } else if (one_min..one_hour).contains(&time) {
        // 1 minute to 1 hour
        format!("{:.2}min", time.as_f64() / one_min as f64)
    } else if (one_hour..one_day).contains(&time) {
        // 1 hour to 1 day
        format!("{:.2}h", time.as_f64() / one_hour as f64)
    } else if (one_day..one_month).contains(&time) {
        // 1 day to 1 month
        format!("{:.2}days", time.as_f64() / one_day as f64)
    } else if (one_month..one_year).contains(&time) {
        // 1 month to 1 year
        format!("{:.2}months", time.as_f64() / one_month as f64)
    } else if time > one_year {
        format!("{:.2}y", time.as_f64() / one_year as f64)
    } else {
        "?".to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::format_time;

    #[test]
    pub fn correctly_formats_time() {
        assert_eq!("1.00days".to_string(), format_time(86_400));
        assert_eq!("24.00h".to_string(), format_time(86_399));
        assert_eq!("23.97h".to_string(), format_time(86_300));
        assert_eq!("6.00min".to_string(), format_time(360));
        assert_eq!("28.94days".to_string(), format_time(2_500_000));
        assert_eq!("1.00months".to_string(), format_time(2_592_000));
        assert_eq!(
            "15.50y".to_string(),
            format_time((31_104_000.0 * 15.5) as u64)
        );
        assert_eq!("39.69y".to_string(), format_time(1_234_567_890));
    }
}
