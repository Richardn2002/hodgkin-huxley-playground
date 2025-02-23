use crate::{Float, rate};

pub fn tau_m(v: Float) -> Float {
    1.0 / (rate::alpha_m(v) + rate::beta_m(v))
}

pub fn m_inf(v: Float) -> Float {
    rate::alpha_m(v) / (rate::alpha_m(v) + rate::beta_m(v))
}

pub fn tau_h(v: Float) -> Float {
    1.0 / (rate::alpha_h(v) + rate::beta_h(v))
}

pub fn h_inf(v: Float) -> Float {
    rate::alpha_h(v) / (rate::alpha_h(v) + rate::beta_h(v))
}

pub fn tau_n(v: Float) -> Float {
    1.0 / (rate::alpha_n(v) + rate::beta_n(v))
}

pub fn n_inf(v: Float) -> Float {
    rate::alpha_n(v) / (rate::alpha_n(v) + rate::beta_n(v))
}

pub struct Pulse {
    pub start: Float,
    pub end: Float,
    pub magnitude: Float,
}
pub struct Setup {
    pub v0: Float,
    pub end: Float,
    pub dt: Float,
    pub steps_per_frame: usize,
    pub pulse: Pulse,
}

impl Setup {
    pub fn total_steps(&self) -> usize {
        (self.end / self.dt).floor() as usize
    }
}

impl Default for Setup {
    fn default() -> Self {
        Self {
            v0: 0.0,
            end: 10.0,
            dt: 0.01,
            steps_per_frame: 1000,
            pulse: Pulse {
                start: 0.0,
                end: 1.0,
                magnitude: 10.0,
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Axon {
    /// V, m, h, n
    pub data: [Float; 4],
}

#[derive(Default)]
pub struct State {
    pub setup: Setup,
    pub simulating: bool,
    pub points_avail: usize,
    pub history: Vec<Axon>,
}
