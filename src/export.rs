use std::error::Error;

use planety_core::System;

use crate::types::Config;

pub fn export_system_to_csv(
    config: Config,
    system: System,
    step: i64,
    time: f64,
) -> Result<(), Box<dyn Error>> {
    let filename = format! {"{}_{}.csv", config.export_file_name_prefix, step};
    let mut wtr = csv::Writer::from_path(filename)?;

    wtr.write_record(&["Time", "Name", "Mass", "x", "y", "vx", "vy"])?;

    for body in system.bodies {
        wtr.serialize((
            time,
            body.name,
            body.mass,
            body.position.x,
            body.position.y,
            body.velocity.x,
            body.velocity.y,
        ))?;
    }
    wtr.flush()?;

    Ok(())
}
