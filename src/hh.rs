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
