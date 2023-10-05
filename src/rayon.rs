#[cfg(test)]
#[allow(unused_variables, unused_imports)]
mod tests {
    use anyhow::Result;
    use rayon::prelude::*;
    use std::{thread, time::Duration};

    #[test]
    fn compute_pi() {
        let pi = monte_carlo_pi(10_000_000);
        println!("π = {}", pi);
    }

    fn monte_carlo_pi(points: usize) -> f64 {
        let within_circle = (0..points)
            // .into_par_iter()
            .filter_map(|_| {
                let x = rand::random::<f64>() * 2f64 - 1f64;
                let y = rand::random::<f64>() * 2f64 - 1f64;
                if x * x + y * y <= 1f64 {
                    Some(1)
                } else {
                    None
                }
            })
            .count();
        4f64 * within_circle as f64 / points as f64
    }
}
