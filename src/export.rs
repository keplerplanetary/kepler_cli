use std::{error::Error, fs::DirBuilder, path::Path};

use planety_core::System;

use crate::configsystem::Config;

pub fn export_system_to_csv(
    config: Config,
    system: System,
    step: i64,
    time: f64,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.export_directory);

    if !path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(path)
            .expect("That the export path could be created.");
    }

    let filename = format! {"{}_{}.csv", config.export_file_name_prefix, step};
    let filename_path = Path::new(&filename);
    let fullpath = path.join(filename_path);
    let mut wtr = csv::Writer::from_path(fullpath)?;

    wtr.write_record(["Time", "Name", "Mass", "x", "y", "vx", "vy"])?;

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
