use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OUConfig {
    pub theta: f64,
    pub mu: f64,
    pub sigma: f64,
    pub x0: f64,
    pub final_time: f64,
    pub maximum_timestep: u32,
    pub micro_timestep: u32,
    pub seed: u64,
}
