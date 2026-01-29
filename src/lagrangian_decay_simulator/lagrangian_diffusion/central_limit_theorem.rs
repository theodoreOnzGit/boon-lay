use uom::si::f64::*;


/// Compute per-component variance sigma2 for the Gaussian displacement after n isotropic steps.
/// General case: sigma2 = n * E[S^2] / 3.
/// For exponential step lengths with mean lambda, E[S^2] = 2 lambda^2 ⇒ sigma2 = n * 2 lambda^2 / 3.
///
pub fn per_component_variance_from_second_moment(
    n: u64, e_s2: Area
) -> Area {
    (n as f64) * e_s2 / 3.0
}

pub fn per_component_variance_exponential(n: u64, lambda: Length) -> Area {
    let e_s2: Area = 2.0 * lambda * lambda;
    per_component_variance_from_second_moment(n, e_s2)
}


