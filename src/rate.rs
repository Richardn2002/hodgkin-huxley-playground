use crate::Float;

pub fn alpha_m(v: Float) -> Float {
    let v_minus_25 = v - 25.0;
    if v_minus_25.abs() < 0.01 {
        1. + v_minus_25 / 20. + v_minus_25 * v_minus_25 / 1200.
    } else {
        0.1 * v_minus_25 / (1. - (-v_minus_25 / 10.).exp())
    }
}

pub fn beta_m(v: Float) -> Float {
    4.0 * Float::exp(-v / 18.0)
}

pub fn alpha_h(v: Float) -> Float {
    0.07 * Float::exp(-v / 20.0)
}

pub fn beta_h(v: Float) -> Float {
    1.0 / (Float::exp(3.0 - v / 10.0) + 1.0)
}

pub fn alpha_n(v: Float) -> Float {
    let v_minus_10 = v - 10.0;
    if v_minus_10.abs() < 0.01 {
        0.1 + v_minus_10 / 200. + v_minus_10 * v_minus_10 / 12000.
    } else {
        0.01 * v_minus_10 / (1. - (-v_minus_10 / 10.).exp())
    }
}

pub fn beta_n(v: Float) -> Float {
    Float::exp(-v / 80.0) / 8.0
}
