use crate::configsystem::Config;
use maths_rs::min;
use plotters::{prelude::*, style::text_anchor::Pos};
use std::{fs::DirBuilder, path::Path};

fn format_label(number: &f64) -> String {
    // Use scientific notation for very large or very small numbers
    if number.abs() > 1e5 || number.abs() < 1e-5 {
        return format!("{:.2e}", number);
    }
    format!("{:.3}", number)
}

#[derive(Debug, Clone, Copy)]
pub struct PlotDatum {
    pub time: f64,
    pub total_energy: f64,
    pub kinetic_energy: f64,
    pub potential_energy: f64,
}

pub fn plot_total_energy(
    data: Vec<PlotDatum>,
    config: &Config,
) -> Result<(), DrawingAreaErrorKind<std::io::Error>> {
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

    root_drawing_area
        .fill(&WHITE)
        .expect("Should be able to fill the drawing area with white");
    let root_drawing_area = root_drawing_area.margin(20, 20, 20, 40);
    let total_energy_color = RED;
    let potential_energy_color = BLUE;
    let kinetic_energy_color = GREEN;

    let mut x_min = data
        .iter()
        .map(|e| e.time)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let x_max = data
        .iter()
        .map(|e| e.time)
        .fold(-f64::INFINITY, |a, b| a.max(b));
    let mut y_max_total_energy = data
        .iter()
        .map(|e| e.total_energy)
        .fold(-f64::INFINITY, |a, b| a.max(b));
    let mut y_min_potential_energy = data
        .iter()
        .map(|e| e.potential_energy)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let mut y_min_kinetic_energy = data
        .iter()
        .map(|e| e.kinetic_energy)
        .fold(f64::INFINITY, |a, b| a.min(b));
    // add 5% padding around the max and min values
    x_min *= 0.95;
    // x_max *= 1.05; // max time should not be padded
    y_max_total_energy *= 1.05;
    y_min_potential_energy *= 0.95;
    y_min_kinetic_energy *= 0.95;
    let y_min_energy = min(y_min_potential_energy, y_min_kinetic_energy);

    let label_size = root_drawing_area
        .estimate_text_size(
            &format_label(&y_max_total_energy),
            &TextStyle {
                font: ("Sans-serif", 20).into_font(),
                color: BLACK.to_backend_color(),
                pos: Pos {
                    h_pos: plotters::style::text_anchor::HPos::Center,
                    v_pos: plotters::style::text_anchor::VPos::Center,
                },
            },
        )
        .expect("Should be able to estimate the text size");

    let mut chart_context = ChartBuilder::on(&root_drawing_area)
        .caption("System Energy over Time", ("Sans-serif", 20).into_font())
        .x_label_area_size(label_size.0)
        .y_label_area_size(label_size.0)
        .build_cartesian_2d(x_min..x_max, y_min_energy..y_max_total_energy)?;

    chart_context
        .configure_mesh()
        .x_labels(6)
        .y_labels(6)
        .x_label_formatter(&format_label)
        .y_label_formatter(&format_label)
        .x_desc("Time (s)")
        .y_desc("Energy (J?)")
        .draw()?;

    let total_energy_series_annotation = chart_context.draw_series(LineSeries::new(
        data.iter().map(|d| (d.time, d.total_energy)),
        &total_energy_color,
    ))?;
    total_energy_series_annotation
        .label("Total Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], total_energy_color));

    let potential_energy_series_annotation = chart_context.draw_series(LineSeries::new(
        data.iter().map(|d| (d.time, d.potential_energy)),
        &potential_energy_color,
    ))?;
    potential_energy_series_annotation
        .label("Potential Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], potential_energy_color));

    let kinetic_energy_series_annotation = chart_context.draw_series(LineSeries::new(
        data.iter().map(|d| (d.time, d.kinetic_energy)),
        &kinetic_energy_color,
    ))?;
    kinetic_energy_series_annotation
        .label("Kinetic Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], kinetic_energy_color));
    chart_context
        .configure_series_labels()
        .position(SeriesLabelPosition::MiddleRight)
        .border_style(BLACK)
        .legend_area_size(50)
        .draw()?;
    Ok(())
}
