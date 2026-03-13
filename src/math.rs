use crate::types::StockObservation;

pub fn calculate_ema(
    data: &[StockObservation], 
    smoothing_constant: f64
) -> f64 {
    let ema = data.iter().fold(None, |accumulating_variable: Option<f64>, obs| {
        match accumulating_variable {
            None => {
                Some(obs.close)
            }, 
            Some(previous_ema) => {
                Some(smoothing_constant * obs.close + (1.0 - smoothing_constant) * previous_ema)
            }
        }
    });
    ema.unwrap_or(0.0)
}
// skriv unit test