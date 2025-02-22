use crate::Float;

/// Provide current state `x` and time `t`, return updated state after step size `dt`.
///
/// `system`: when provided state `x` and time `t`, writes corresponding values into `dx`.
pub fn step<const N: usize, F>(system: F, x: [Float; N], t: Float, dt: Float) -> [Float; N]
where
    F: Fn(&[Float; N], Float, &mut [Float; N]),
{
    let half = Float::from(0.5);
    let six = Float::from(6.0);
    let three = Float::from(3.0);

    let mut xn = x;
    let mut kn = [Float::NAN; N];
    let mut new_x = x;

    system(&xn, t, &mut kn);
    for i in 0..N {
        xn[i] += kn[i] * half * dt;
        new_x[i] += kn[i] / six * dt;
    }

    system(&xn, t + dt * half, &mut kn);
    xn = x;
    for i in 0..N {
        xn[i] += kn[i] * half * dt;
        new_x[i] += kn[i] / three * dt
    }

    system(&xn, t + dt * half, &mut kn);
    xn = x;
    for i in 0..N {
        xn[i] += kn[i] * dt;
        new_x[i] += kn[i] / three * dt
    }

    system(&xn, t + dt, &mut kn);
    for i in 0..N {
        new_x[i] += kn[i] / six * dt
    }

    new_x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp() {
        fn system(y: &[Float; 1], _x: Float, dy: &mut [Float; 1]) {
            dy[0] = y[0];
        }

        let x = Float::from(10);

        let dx = Float::from(0.000001);
        let y = {
            let mut y = [Float::from(1)];
            for i in 0..(x / dx).floor() as u32 {
                y = step(system, y, Float::from(i) * dx, dx);
            }
            y
        };

        assert!((y[0] - x.exp()).abs() < 1e-5);
    }

    #[test]
    fn u() {
        fn system(_y: &[Float; 1], x: Float, dy: &mut [Float; 1]) {
            dy[0] = if x > Float::from(1.0) && x <= Float::from(6.0) {
                Float::from(1.0)
            } else {
                Float::from(0.0)
            }
        }

        let x = Float::from(10);
        let dx = Float::from(0.000001);
        let y = {
            let mut y = [Float::from(0.0)];
            for i in 0..(x / dx).floor() as u32 {
                y = step(system, y, Float::from(i) * dx, dx);
            }
            y
        };

        assert!((y[0] - Float::from(5.0)).abs() < 1e-5);
    }
}
