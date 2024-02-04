#[derive(Debug, Clone)]
pub struct Config {
    pub timestep: f64,
    pub steps: i64,
    pub export_step: i64,
    pub export_file_name_prefix: String,
}
