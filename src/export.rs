use kepler_core::types::System;
use std::{
    error::Error,
    fs::{DirBuilder, OpenOptions},
    io::Write,
    path::Path,
};

use crate::configsystem::Config;

pub fn export_system_snapshot_to_csv(
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

pub fn export_system_to_csv_by_body(
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

    for body in system.bodies {
        let filename = format! {"{}.csv", body.name};
        let filename_path = Path::new(&filename);
        let fullpath = path.join(filename_path);
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize((
            time,
            body.mass,
            body.position.x,
            body.position.y,
            body.velocity.x,
            body.velocity.y,
        ))?;
        // TODO: Remove newlines from csv writer generated lines

        let text = String::from_utf8(wtr.into_inner()?)?;

        match std::fs::metadata(&fullpath) {
            Ok(metadata) => {
                if metadata.is_file() {
                    let mut file = OpenOptions::new().write(true).append(true).open(fullpath)?;
                    writeln!(file, "{}", text)?;
                }
            }
            Err(_) => {
                let mut wtr = csv::Writer::from_path(fullpath)?;
                wtr.write_record(["Time", "Mass", "x", "y", "vx", "vy"])?;
                wtr.serialize((
                    time,
                    body.mass,
                    body.position.x,
                    body.position.y,
                    body.velocity.x,
                    body.velocity.y,
                ))?;
                wtr.flush()?;
            }
        }
    }

    Ok(())
}
