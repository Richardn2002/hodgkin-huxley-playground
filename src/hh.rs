use crate::{Float, rate, rk4};

pub mod consts {
    use super::Float;

    pub const E_NA: Float = 115.0;
    pub const E_K: Float = -12.0;
    pub const E_L: Float = 10.6;

    pub const G_NA_MAX: Float = 120.0;
    pub const G_K_MAX: Float = 36.0;
    pub const G_L_MAX: Float = 0.3;

    pub const C_M: Float = 1.0;
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
    simulating: bool,
    pub points_avail: usize,
    pub history: Vec<Axon>,
}

impl State {
    pub fn simulating(&self) -> bool {
        self.simulating
    }

    pub fn init(&mut self) {
        if self.setup.total_steps() == 0 {
            return;
        }

        self.history
            .resize(self.setup.total_steps(), Axon::default());
        self.history[0] = Axon {
            data: [
                self.setup.v0,
                m_inf(self.setup.v0),
                h_inf(self.setup.v0),
                n_inf(self.setup.v0),
            ],
        };
        self.points_avail = 1;
        self.simulating = true;
    }

    /// do nothing if not already simulating
    ///
    /// continue simulating for `steps`. if upper limit met, break early and end the simulation
    pub fn step(&mut self) {
        if !self.simulating {
            return;
        }

        for _ in 0..self.setup.steps_per_frame {
            if self.points_avail == self.setup.total_steps() {
                self.simulating = false;
                return;
            }

            let system = |state: &[Float; 4], t: Float, d_state: &mut [Float; 4]| {
                let i = if t >= self.setup.pulse.start && t < self.setup.pulse.end {
                    self.setup.pulse.magnitude
                } else {
                    0.0
                };

                let axon = Axon { data: *state };
                d_state[0] =
                    (-axon.i_na() - axon.i_k() + consts::G_L_MAX * (consts::E_L - axon.v()) + i)
                        / consts::C_M;
                d_state[1] = (-axon.m() + axon.m_inf()) / tau_m(axon.v());
                d_state[2] = (-axon.h() + axon.h_inf()) / tau_h(axon.v());
                d_state[3] = (-axon.n() + axon.n_inf()) / tau_n(axon.v());
            };

            let new_state = rk4::step(
                system,
                self.history[self.points_avail - 1].data,
                self.setup.dt * self.points_avail as Float,
                self.setup.dt,
            );
            self.history[self.points_avail] = Axon { data: new_state };
            self.points_avail += 1;
        }
    }
}
