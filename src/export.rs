use kepler_core::{
    center_of_mass::calculate_center_of_mass, energy::calculate_system_energy,
    impulse::calculate_total_impulse, types::System,
};
use std::{
    error::Error,
    fs::{DirBuilder, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use crate::configsystem::Config;

pub fn export_system_snapshot_to_csv(
    config: Config,
    system: System,
    step: i64,
    time: f64,
) -> Result<(), Box<dyn Error>> {
    let headers: Vec<String> = vec!["Time", "Name", "Mass", "x", "y", "vx", "vy"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
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

    wtr.write_record(&headers)?;

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
    let headers: Vec<String> = vec!["Step", "Time", "Mass", "x", "y", "vx", "vy"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

    let path = Path::new(&config.export_directory);

    if !path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(path)
            .expect("That the export path could be created.");
    }

    for body in system.bodies {
        let filename = format! {"{}_{}.csv", config.export_file_name_prefix, body.name};
        let filename_path = Path::new(&filename);
        let fullpath = path.join(filename_path);

        // first, check if the object we want to write to exists, and if it does, if it is a file
        match std::fs::metadata(&fullpath) {
            Ok(metadata) => {
                if metadata.is_file() {
                    if step == 0 {
                        // overwrite the file with fresh headers
                        write_csv_headers(&fullpath, &headers)?;
                    }
                    // nothing to do, we can go ahead
                } else {
                    // we would like to write to something that exists, but it's not a file
                    // so we return a file not found error
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!(
                            "The destination file object already exists, but it is not a file: {}",
                            fullpath
                                .to_str()
                                .expect("that the path can be formatted as str")
                        ),
                    )));
                }
            }
            Err(_e) => {
                // if the fs object we want to write to does not exist, we create a file and write the csv headers
                write_csv_headers(&fullpath, &headers)?;
            }
        }

        // in any case, we write a new line to the export file, possibly after creating it first
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize((
            time,
            body.mass,
            body.position.x,
            body.position.y,
            body.velocity.x,
            body.velocity.y,
        ))?;

        // here we remove the generated newline character from the csv library so that we can use writeln below.
        let text = String::from_utf8(wtr.into_inner()?)?.replace("\n", "");

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&fullpath)?;
        writeln!(file, "{}", text)?;
    }

    Ok(())
}

pub fn export_system_parameters_to_csv(
    config: &Config,
    system: &System,
    step: i64,
    time: f64,
) -> Result<(), Box<dyn Error>> {
    let headers: Vec<String> = vec![
        "Step",
        "Time",
        "Energy",
        "Impulse x",
        "Impulse y",
        "Center of mass x",
        "Center of mass y",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();

    let path = Path::new(&config.export_directory);

    if !path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(path)
            .expect("That the export path could be created.");
    }

    let filename = format! {"{}_system_parameters.csv", config.export_file_name_prefix};
    let filename_path = Path::new(&filename);
    let fullpath = path.join(filename_path);

    // first, check if the object we want to write to exists, and if it does, if it is a file
    match std::fs::metadata(&fullpath) {
        Ok(metadata) => {
            if metadata.is_file() {
                if step == 0 {
                    // overwrite the file with fresh headers
                    write_csv_headers(&fullpath, &headers)?;
                }
                // nothing to do, we can go ahead
            } else {
                // we would like to write to something that exists, but it's not a file
                // so we return a file not found error
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!(
                        "The destination file object already exists, but it is not a file: {}",
                        fullpath
                            .to_str()
                            .expect("that the path can be formatted as str")
                    ),
                )));
            }
        }
        Err(_e) => {
            // if the fs object we want to write to does not exist, we create a file and write the csv headers
            write_csv_headers(&fullpath, &headers)?;
        }
    }

    // in any case, we write a new line to the export file, possibly after creating it first
    let mut wtr = csv::Writer::from_writer(vec![]);

    let total_impulse = calculate_total_impulse(system);
    let center_of_mass = calculate_center_of_mass(system);
    wtr.serialize((
        step,
        time,
        calculate_system_energy(system),
        total_impulse.x,
        total_impulse.y,
        center_of_mass.x,
        center_of_mass.y,
    ))?;

    // here we remove the generated newline character from the csv library so that we can use writeln below.
    let text = String::from_utf8(wtr.into_inner()?)?.replace("\n", "");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&fullpath)?;
    writeln!(file, "{}", text)?;

    Ok(())
}

fn write_csv_headers(fullpath: &PathBuf, headers: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(&fullpath)?;
    wtr.write_record(headers)?;
    wtr.flush()?;
    Ok(())
}
