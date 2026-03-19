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




pub fn value_at_risk(
    data: &[StockObservation], 
    alpha: f64
) -> f64 {

    let mut loss_data: Vec<f64> = data
        .windows(2)
        .map(|windows| {
            let x_n_minus_1 = windows[0].close;
            let x_n = windows[1].close;
            let loss_data = -1.0* ( x_n.ln() - x_n_minus_1.ln());
            loss_data
        })
        .collect();
    
    // now we find the quantile
    // we sort the data and then we fint the inf { x \in R : P(X > L) \leq \alpha}
    // since f64 can be na, we need to tell the compiler how we want to sort 
    loss_data.sort_by(|a,b | a.partial_cmp(b).unwrap());
    /*
    We need to find the quantile.
    We apply the estimator 
    1/n \sum_{i=1}^n 1_{L_i \leq x} 
    
    This is obtained at exactly k/n \geq \alpha
    we check k = ceil(n \alpha) 
    */
    let n: usize = loss_data.len();
    if n == 0 { return 0.0;}
    let index_float = (n as f64 * alpha).ceil();
    let index = index_float as usize; // we convert to integer since alpha is float

    // since rust starts at index 0 we need to do this in a safe manner 
    let res = if index == 0 {
        return 0.0;
    } else if index < n  {
        index - 1
    } else {
        n - 1
    };
    return loss_data[res]
}



// laver unit til value_at_risk

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StockObservation;


    #[test]
    fn test_value_at_risk() {
        let data = vec![
            StockObservation { date: "2024-01-01".to_string(), close: 100.0},
            StockObservation { date: "2024-01-02".to_string(), close: 90.0},
            StockObservation { date: "2024-01-03".to_string(), close: 95.0},
            StockObservation { date: "2024-01-04".to_string(), close: 85.0},
        ];

        let alpha = 0.75;
        let result = value_at_risk(&data, alpha);
        //let result = 10.0;

        let expected_loss = -1.0 * (85.0_f64.ln() - 95.0_f64.ln());

        let epsilon = 1e-10;
        assert!(
            (result - expected_loss).abs() < epsilon, 
            "Error in VaR: Expected {}, realized {}", 
            expected_loss, 
            result
        );
        
    }
}