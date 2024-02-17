use kepler_core::{mover::system_timestep, types::System};
use maths_rs::num::Cast;

use crate::{
    configsystem::Config,
    export::{
        export_system_parameters_to_csv, export_system_snapshot_to_csv,
        export_system_to_csv_by_body,
    },
};

pub fn run_simulation(config: Config, initial_system: System) {
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
        let seconds = 86_400;
        let expected_output = "1.00days".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
        let seconds = 86_399;
        let expected_output = "24.00h".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
        let seconds = 86_300;
        let expected_output = "23.97h".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
        let seconds = 360;
        let expected_output = "6.00min".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
        let seconds = 2_500_000;
        let expected_output = "28.94days".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
        let seconds = 2_592_000;
        let expected_output = "1.00months".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
        let seconds = (31_104_000.0 * 15.5) as u64; // 15.5 years
        let expected_output = "15.50y".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
        let seconds = 1234567890;
        let expected_output = "39.69y".to_string();
        let actual_output = format_time(seconds);
        assert_eq!(expected_output, actual_output);
    }
}
