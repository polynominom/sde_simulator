mod configs;

use configs::ou_config::OUConfig;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TimeSeries {
    timesteps: Vec<f64>,
    values: Vec<f64>,
}

#[wasm_bindgen]
impl TimeSeries {
    #[wasm_bindgen(getter)]
    pub fn timesteps(&self) -> Vec<f64> {
        self.timesteps.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Vec<f64> {
        self.values.clone()
    }
}

#[wasm_bindgen]
pub fn ou_euler_maruyama(
    theta: f64,
    mu: f64,
    sigma: f64,
    x0: f64,
    tmax: f64,
    n: u32,
    r: u32,
    seed: u64,
) -> TimeSeries {
    let mut rand_gen_with_seed = StdRng::seed_from_u64(seed);

    let dt = tmax / (n as f64);
    let dt_micro = dt / (r as f64);

    let mut x_vec: Vec<f64> = Vec::new();
    let mut timesteps: Vec<f64> = Vec::new();

    x_vec.resize(n as usize, 0.0);
    timesteps.resize(n as usize, 0.0);
    x_vec[0] = x0;
    timesteps[0] = 0.0;

    // Obtain dW
    let normal = Normal::new(0.0, 1.0).unwrap();
    for t in 1..n {
        let t = t as usize;
        let mut dW: f64 = 0.0;
        for _ in 0..r {
            dW = dW + normal.sample(&mut rand_gen_with_seed);
        }
        dW = dW * dt_micro.sqrt();

        let dx = theta * (mu - x_vec[t - 1]) * dt + sigma * dW;
        x_vec[t] = x_vec[t - 1] + dx;
        timesteps[t] = t as f64 * dt;
    }

    TimeSeries {
        timesteps,
        values: x_vec,
    }
}

fn main() -> std::io::Result<()> {
    let file_content = fs::read_to_string("parameters.json")?;
    let json_value: Value = serde_json::from_str(&file_content)?;

    // Extract the "ou" field, return fallback if not found
    let ou_value = match json_value.get("ou") {
        Some(value) if value.is_object() => value,
        _ => {
            println!("ou field is missing or not a valid object in the JSON file");
            return Ok(()); // Exit early
        }
    };
    let ou_config: OUConfig = serde_json::from_value(ou_value.clone())?;

    let timeseries = ou_euler_maruyama(
        ou_config.theta,
        ou_config.mu,
        ou_config.sigma,
        ou_config.x0,
        ou_config.final_time,
        ou_config.maximum_timestep,
        ou_config.micro_timestep,
        ou_config.seed,
    );
    Ok(())
}
