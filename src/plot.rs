use crate::configsystem::Config;
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
}

// TODO: add potential and kinetic energy
pub fn plot_total_energy(data: Vec<PlotDatum>, config: &Config) {
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
    let root_drawing_area = root_drawing_area.margin(20, 20, 20, 20);
    let total_energy_color = RED;

    let mut x_min = data
        .iter()
        .map(|e| e.time)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let mut x_max = data
        .iter()
        .map(|e| e.time)
        .fold(-f64::INFINITY, |a, b| a.max(b));
    let mut y_min = data
        .iter()
        .map(|e| e.total_energy)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let mut y_max = data
        .iter()
        .map(|e| e.total_energy)
        .fold(-f64::INFINITY, |a, b| a.max(b));
    // add 5% padding around the max and min values
    x_min *= 0.95;
    x_max *= 1.05;
    y_min *= 0.95;
    y_max *= 1.05;

    let y_label_size = root_drawing_area
        .estimate_text_size(
            &format_label(&y_max),
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

    let x_label_size = root_drawing_area
        .estimate_text_size(
            &format_label(&y_max),
            &TextStyle {
                font: ("Sans-serif", 15).into_font(),
                color: BLACK.to_backend_color(),
                pos: Pos {
                    h_pos: plotters::style::text_anchor::HPos::Center,
                    v_pos: plotters::style::text_anchor::VPos::Center,
                },
            },
        )
        .expect("Should be able to estimate the text size");
    let mut chart_context = match ChartBuilder::on(&root_drawing_area)
        .caption("System Energy over Time", ("Sans-serif", 20).into_font())
        .x_label_area_size(x_label_size.0)
        .y_label_area_size(y_label_size.0)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
    {
        Ok(chart) => chart,
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Error while creating chart: {e}");
            return;
        }
    };

    match chart_context
        .configure_mesh()
        .x_labels(6)
        .y_labels(6)
        .x_label_formatter(&format_label)
        .y_label_formatter(&format_label)
        .y_desc("Time (s)")
        .x_desc("Total Energy (J?)")
        .draw()
    {
        Ok(()) => (),
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Error while configuring mesh: {e}");
            return;
        }
    };
    let series_annotation = match chart_context.draw_series(LineSeries::new(
        data.iter().map(|d| (d.time, d.total_energy)),
        &total_energy_color,
    )) {
        Ok(series) => series,
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Error while drawing series: {e}");
            return;
        }
    };
    series_annotation
        .label("Total Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], total_energy_color));
    match chart_context
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(BLACK)
        .legend_area_size(50)
        .draw()
    {
        Ok(()) => (),
        Err(e) => {
            tracing::event!(
                tracing::Level::ERROR,
                "Error while configuring series labels: {e}"
            )
        }
    };
}
