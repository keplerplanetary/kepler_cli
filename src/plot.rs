use std::{fs::DirBuilder, path::Path};
use plotters::prelude::*;
use crate::configsystem::Config;

fn format_label(number: &f64) -> String {
    format!("{:.3}", number)
}

// TODO: replace the tuple with a struct and add potential and kinetic energy
pub fn plot_total_energy(data: Vec<(f64, f64)>, config: &Config) {
    let path = Path::new(&config.export_directory);

    if !path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(path)
            .expect("That the export path could be created.");
    }

    let filename = format! {"{}_Energy.svg", config.export_file_name_prefix};
    let filename_path = Path::new(&filename);
    let fullpath = path.join(filename_path);

    let root_drawing_area = SVGBackend::new(&fullpath, (640, 480)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();
    let root_drawing_area = root_drawing_area.margin(10, 10, 10, 10);

    // TODO add like 5% padding around the max and min values
    let x_min = data
        .iter()
        .map(|e| e.0)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let x_max = data
        .iter()
        .map(|e| e.0)
        .fold(-f64::INFINITY, |a, b| a.max(b));
    let y_min = data
        .iter()
        .map(|e| e.1)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let y_max = data
        .iter()
        .map(|e| e.1)
        .fold(-f64::INFINITY, |a, b| a.max(b));

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Energy", ("Sans-serif", 20).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(6)
        .y_labels(6)
        .x_label_formatter(&|x| format_label(x))
        .y_label_formatter(&|y| format_label(y))
        .draw()
        .unwrap();
    chart
        .draw_series(LineSeries::new(data, &RED))
        .unwrap()
        .label("Total Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    chart
        .configure_series_labels()
        .border_style(BLACK)
        .legend_area_size(50)
        .draw()
        .unwrap();
}
