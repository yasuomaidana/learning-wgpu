pub fn handle_pressure_event(
    pressure:f32,
    previous_pressure: f64,
) -> f64{
    if pressure == 0.0 && (previous_pressure - pressure as f64).abs() > 0.2 {
        previous_pressure - 0.1
    } else {
        pressure as f64
    }
}