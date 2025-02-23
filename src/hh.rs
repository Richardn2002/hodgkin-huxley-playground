use crate::{Float, rate};

pub mod consts {
    use super::Float;

    pub const E_NA: Float = 115.0;
    pub const E_K: Float = -12.0;
    pub const E_L: Float = 10.6;

    pub const G_NA_MAX: Float = 120.0;
    pub const G_K_MAX: Float = 36.0;
    pub const G_L_MAX: Float = 0.3;
}

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
    data: [Float; 4],
}

impl Axon {
    pub fn v(&self) -> Float {
        self.data[0]
    }
    pub fn m(&self) -> Float {
        self.data[1]
    }
    pub fn h(&self) -> Float {
        self.data[2]
    }
    pub fn n(&self) -> Float {
        self.data[3]
    }

    pub fn cond_na(&self) -> Float {
        consts::G_NA_MAX * self.m().powi(3) * self.h()
    }

    pub fn cond_k(&self) -> Float {
        consts::G_K_MAX * self.n().powi(4)
    }

    pub fn i_na(&self) -> Float {
        self.cond_na() * (self.v() - consts::E_NA)
    }

    pub fn i_k(&self) -> Float {
        self.cond_k() * (self.v() - consts::E_K)
    }

    pub fn m_inf(&self) -> Float {
        m_inf(self.v())
    }

    pub fn h_inf(&self) -> Float {
        h_inf(self.v())
    }

    pub fn n_inf(&self) -> Float {
        n_inf(self.v())
    }
}

#[derive(Default)]
pub struct State {
    pub setup: Setup,
    pub simulating: bool,
    pub points_avail: usize,
    pub history: Vec<Axon>,
}
