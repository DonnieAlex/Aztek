//! Hair scattering, for aiStandardHair: the fibre as a rough dielectric cylinder with a tilted cuticle and
//! pigment inside. Light reflects off the surface (R), passes through (TT), reflects once inside (TRT) and more; each is
//! a longitudinal lobe times an azimuthal one, after d'Eon et al. 2011 and Chiang et al. 2016, in the form the PBRT book
//! publishes. Melanin sets the absorption, as the reference renderer's control does.
//!
//! The local frame has x along the fibre, z the normal facing the ray and y across the fibre; `h` is where across the
//! width the ray met it, -1 to 1.

use glam::{Vec2, Vec3};

use crate::math::PI;

const P_MAX: usize = 3;
const SQRT_PI_OVER_8: f32 = 0.626_657_07;

#[derive(Clone, Copy, Debug)]
pub struct HairLobe {
    pub h: f32,
    pub eta: f32,
    pub sigma_a: Vec3,
    v: [f32; P_MAX + 1],
    s: f32,
    sin2k: [f32; 3],
    cos2k: [f32; 3],
    gamma_o: f32,
    /// The tints on the first reflection and on what passes through.
    pub r_tint: Vec3,
    pub tt_tint: Vec3,
    pub trt_tint: Vec3,
}

/// What the fibre is made of, as aiStandardHair describes it.
#[derive(Clone, Copy, Debug)]
pub struct HairParams {
    /// Eumelanin and pheomelanin together, 0 (white) to 1 (black).
    pub melanin: f32,
    /// The share of pheomelanin, which reddens.
    pub melanin_redness: f32,
    /// A dye over the pigment.
    pub base_color: Vec3,
    pub roughness: f32,
    pub roughness_azimuthal: f32,
    pub ior: f32,
    /// The cuticle's tilt, in degrees.
    pub shift: f32,
    pub specular_tint: Vec3,
    pub specular2_tint: Vec3,
    pub transmission_tint: Vec3,
}

impl Default for HairParams {
    fn default() -> Self {
        // aiStandardHair's documented defaults.
        HairParams {
            melanin: 1.0,
            melanin_redness: 0.5,
            base_color: Vec3::ONE,
            roughness: 0.2,
            roughness_azimuthal: 0.2,
            ior: 1.55,
            shift: 3.0,
            specular_tint: Vec3::ONE,
            specular2_tint: Vec3::ONE,
            transmission_tint: Vec3::ONE,
        }
    }
}

fn sqr(x: f32) -> f32 {
    x * x
}

fn safe_sqrt(x: f32) -> f32 {
    x.max(0.0).sqrt()
}

fn safe_asin(x: f32) -> f32 {
    x.clamp(-1.0, 1.0).asin()
}

/// Absorption from melanin concentrations (per unit of the fibre's diameter).
fn sigma_from_melanin(eu: f32, pheo: f32) -> Vec3 {
    Vec3::new(0.419, 0.697, 1.37) * eu + Vec3::new(0.187, 0.4, 1.05) * pheo
}

/// Absorption that gives a colour after many bounces, for a given azimuthal roughness.
fn sigma_from_color(c: Vec3, beta_n: f32) -> Vec3 {
    let d = 5.969 - 0.215 * beta_n + 2.532 * beta_n.powi(2) - 10.73 * beta_n.powi(3) + 5.574 * beta_n.powi(4) + 0.245 * beta_n.powi(5);
    let f = |x: f32| sqr(x.max(1e-4).ln() / d);
    Vec3::new(f(c.x), f(c.y), f(c.z))
}

impl HairLobe {
    pub fn new(p: &HairParams, h: f32) -> HairLobe {
        let beta_m = p.roughness.clamp(0.01, 1.0);
        let beta_n = p.roughness_azimuthal.clamp(0.01, 1.0);
        let v0 = sqr(0.726 * beta_m + 0.812 * sqr(beta_m) + 3.7 * beta_m.powi(20));
        let v = [v0, 0.25 * v0, 4.0 * v0, 4.0 * v0];
        let s = SQRT_PI_OVER_8 * (0.265 * beta_n + 1.194 * sqr(beta_n) + 5.372 * beta_n.powi(22));
        let mut sin2k = [0.0; 3];
        let mut cos2k = [0.0; 3];
        sin2k[0] = p.shift.to_radians().sin();
        cos2k[0] = safe_sqrt(1.0 - sqr(sin2k[0]));
        for i in 1..3 {
            sin2k[i] = 2.0 * cos2k[i - 1] * sin2k[i - 1];
            cos2k[i] = sqr(cos2k[i - 1]) - sqr(sin2k[i - 1]);
        }
        // Melanin runs 0 to 1 in the reference's renderer; a black fibre is about 8 units of eumelanin.
        let total = 8.0 * p.melanin.clamp(0.0, 1.0).powf(1.5);
        let red = p.melanin_redness.clamp(0.0, 1.0);
        let sigma_a = sigma_from_melanin(total * (1.0 - red), total * red) + sigma_from_color(p.base_color.clamp(Vec3::splat(1e-3), Vec3::ONE), beta_n);
        let h = h.clamp(-0.999, 0.999);
        HairLobe { h, eta: p.ior.max(1.0001), sigma_a, v, s, sin2k, cos2k, gamma_o: safe_asin(h), r_tint: p.specular_tint, tt_tint: p.transmission_tint, trt_tint: p.specular2_tint }
    }

    fn rotated(&self, p: usize, sin_o: f32, cos_o: f32) -> (f32, f32) {
        let (s, c) = match p {
            0 => (sin_o * self.cos2k[1] - cos_o * self.sin2k[1], cos_o * self.cos2k[1] + sin_o * self.sin2k[1]),
            1 => (sin_o * self.cos2k[0] + cos_o * self.sin2k[0], cos_o * self.cos2k[0] - sin_o * self.sin2k[0]),
            2 => (sin_o * self.cos2k[2] + cos_o * self.sin2k[2], cos_o * self.cos2k[2] - sin_o * self.sin2k[2]),
            _ => (sin_o, cos_o),
        };
        (s, c.abs())
    }

    fn attenuation(&self, cos_o: f32, sin_o: f32) -> ([Vec3; P_MAX + 1], f32) {
        let sin_t = sin_o / self.eta;
        let cos_t = safe_sqrt(1.0 - sqr(sin_t));
        let etap = (sqr(self.eta) - sqr(sin_o)).max(1e-8).sqrt() / cos_o.max(1e-4);
        let sin_gt = self.h / etap;
        let cos_gt = safe_sqrt(1.0 - sqr(sin_gt));
        let gamma_t = safe_asin(sin_gt);
        let tr = (-self.sigma_a * (2.0 * cos_gt / cos_t.max(1e-4))).exp();
        let cos_go = safe_sqrt(1.0 - sqr(self.h));
        let f = crate::bsdf::fresnel_dielectric(cos_o * cos_go, self.eta);
        let mut ap = [Vec3::ZERO; P_MAX + 1];
        ap[0] = Vec3::splat(f);
        ap[1] = tr * sqr(1.0 - f);
        for p in 2..P_MAX {
            ap[p] = ap[p - 1] * tr * f;
        }
        let denom = Vec3::ONE - tr * f;
        ap[P_MAX] = ap[P_MAX - 1] * f * tr / denom.max(Vec3::splat(1e-4));
        ap[0] *= self.r_tint;
        ap[1] *= self.tt_tint;
        ap[2] *= self.trt_tint;
        (ap, gamma_t)
    }

    /// f * |cos| (the integrator's convention) and the density of sample() choosing wi.
    pub fn eval(&self, wo: Vec3, wi: Vec3) -> (Vec3, f32) {
        let sin_o = wo.x.clamp(-1.0, 1.0);
        let cos_o = safe_sqrt(1.0 - sqr(sin_o));
        let phi_o = wo.z.atan2(wo.y);
        let sin_i = wi.x.clamp(-1.0, 1.0);
        let cos_i = safe_sqrt(1.0 - sqr(sin_i));
        let phi_i = wi.z.atan2(wi.y);
        let (ap, gamma_t) = self.attenuation(cos_o, sin_o);
        let pdf_p = ap_pdf(&ap);
        let phi = phi_i - phi_o;
        let mut f = Vec3::ZERO;
        let mut pdf = 0.0;
        for p in 0..P_MAX {
            let (so, co) = self.rotated(p, sin_o, cos_o);
            let m = mp(cos_i, co, sin_i, so, self.v[p]);
            let n = np(phi, p, self.s, self.gamma_o, gamma_t);
            f += ap[p] * (m * n);
            pdf += m * n * pdf_p[p];
        }
        let m = mp(cos_i, cos_o, sin_i, sin_o, self.v[P_MAX]);
        f += ap[P_MAX] * (m / (2.0 * PI));
        pdf += m * pdf_p[P_MAX] / (2.0 * PI);
        (f, pdf)
    }

    /// A direction in the local frame, from four uniform numbers.
    pub fn sample(&self, wo: Vec3, u0: Vec2, u1: Vec2) -> Option<Vec3> {
        let sin_o = wo.x.clamp(-1.0, 1.0);
        let cos_o = safe_sqrt(1.0 - sqr(sin_o));
        let phi_o = wo.z.atan2(wo.y);
        let (ap, gamma_t) = self.attenuation(cos_o, sin_o);
        let pdf_p = ap_pdf(&ap);
        let mut x = u0.x;
        let mut p = 0;
        while p < P_MAX {
            if x < pdf_p[p] {
                break;
            }
            x -= pdf_p[p];
            p += 1;
        }
        let (so, co) = self.rotated(p, sin_o, cos_o);
        let uu = u1.x.max(1e-5);
        let cos_theta = 1.0 + self.v[p] * (uu + (1.0 - uu) * (-2.0 / self.v[p]).exp()).ln();
        let sin_theta = safe_sqrt(1.0 - sqr(cos_theta));
        let cos_phi = (2.0 * PI * u1.y).cos();
        let sin_i = (-cos_theta * so + sin_theta * cos_phi * co).clamp(-1.0, 1.0);
        let cos_i = safe_sqrt(1.0 - sqr(sin_i));
        let dphi = if p < P_MAX { phi_of(p, self.gamma_o, gamma_t) + sample_trimmed_logistic(u0.y, self.s, -PI, PI) } else { 2.0 * PI * u0.y };
        let phi_i = phi_o + dphi;
        let wi = Vec3::new(sin_i, cos_i * phi_i.cos(), cos_i * phi_i.sin());
        wi.is_finite().then_some(wi)
    }
}

fn ap_pdf(ap: &[Vec3; P_MAX + 1]) -> [f32; P_MAX + 1] {
    let y = |c: Vec3| 0.2126 * c.x + 0.7152 * c.y + 0.0722 * c.z;
    let sum: f32 = ap.iter().map(|&c| y(c)).sum();
    let mut out = [0.0; P_MAX + 1];
    for i in 0..=P_MAX {
        out[i] = if sum > 0.0 { y(ap[i]) / sum } else { 1.0 / (P_MAX + 1) as f32 };
    }
    out
}

fn i0(x: f32) -> f32 {
    let mut val = 0.0;
    let mut x2i = 1.0;
    let mut ifact = 1.0f32;
    let mut i4 = 1.0;
    for i in 0..10 {
        if i > 1 {
            ifact *= i as f32;
        }
        val += x2i / (i4 * sqr(ifact));
        x2i *= x * x;
        i4 *= 4.0;
    }
    val
}

fn log_i0(x: f32) -> f32 {
    if x > 12.0 {
        x + 0.5 * (-(2.0 * PI).ln() + (1.0 / x).ln() + 1.0 / (8.0 * x))
    } else {
        i0(x).ln()
    }
}

fn mp(cos_i: f32, cos_o: f32, sin_i: f32, sin_o: f32, v: f32) -> f32 {
    let a = cos_i * cos_o / v;
    let b = sin_i * sin_o / v;
    let m = if v <= 0.1 { (log_i0(a) - b - 1.0 / v + 0.6931 + (1.0 / (2.0 * v)).ln()).exp() } else { ((-b).exp() * i0(a)) / ((1.0 / v).sinh() * 2.0 * v) };
    if m.is_finite() {
        m
    } else {
        0.0
    }
}

fn phi_of(p: usize, gamma_o: f32, gamma_t: f32) -> f32 {
    2.0 * p as f32 * gamma_t - 2.0 * gamma_o + p as f32 * PI
}

fn logistic(x: f32, s: f32) -> f32 {
    let x = x.abs();
    (-x / s).exp() / (s * sqr(1.0 + (-x / s).exp()))
}

fn logistic_cdf(x: f32, s: f32) -> f32 {
    1.0 / (1.0 + (-x / s).exp())
}

fn trimmed_logistic(x: f32, s: f32, a: f32, b: f32) -> f32 {
    logistic(x, s) / (logistic_cdf(b, s) - logistic_cdf(a, s))
}

fn np(phi: f32, p: usize, s: f32, gamma_o: f32, gamma_t: f32) -> f32 {
    let mut dphi = phi - phi_of(p, gamma_o, gamma_t);
    while dphi > PI {
        dphi -= 2.0 * PI;
    }
    while dphi < -PI {
        dphi += 2.0 * PI;
    }
    trimmed_logistic(dphi, s, -PI, PI)
}

fn sample_trimmed_logistic(u: f32, s: f32, a: f32, b: f32) -> f32 {
    let k = logistic_cdf(b, s) - logistic_cdf(a, s);
    let x = -s * (1.0 / (u * k + logistic_cdf(a, s)) - 1.0).ln();
    x.clamp(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hair_conserves_and_samples_where_it_scatters() {
        let lobe = HairLobe::new(&HairParams { melanin: 0.0, base_color: Vec3::ONE, ..Default::default() }, 0.3);
        let wo = Vec3::new(0.3, 0.2, 0.93).normalize();
        // Integrate f*|cos| over the sphere by sampling it uniformly: white hair loses little and gains nothing.
        let mut sum = Vec3::ZERO;
        let n = 200_000;
        let mut rng = crate::math::Pcg::new(7);
        for _ in 0..n {
            let z = 1.0 - 2.0 * rng.next_f32();
            let r = safe_sqrt(1.0 - z * z);
            let ph = 2.0 * PI * rng.next_f32();
            let wi = Vec3::new(r * ph.cos(), r * ph.sin(), z);
            sum += lobe.eval(wo, wi).0 * (4.0 * PI);
        }
        let avg = sum / n as f32;
        assert!(avg.max_element() < 1.05 && avg.min_element() > 0.6, "{avg}");
        // Sampled directions come where eval has density.
        let mut ok = 0;
        for i in 0..2000 {
            let u = |k: u32| ((i * 7919 + k * 104729) % 1000) as f32 / 1000.0;
            if let Some(wi) = lobe.sample(wo, Vec2::new(u(1), u(2)), Vec2::new(u(3), u(4))) {
                if lobe.eval(wo, wi).1 > 0.0 {
                    ok += 1;
                }
            }
        }
        assert!(ok > 1900, "{ok}");
    }

    #[test]
    fn melanin_darkens() {
        let wo = Vec3::new(0.1, 0.0, 1.0).normalize();
        let wi = Vec3::new(-0.1, 0.0, -1.0).normalize();
        let light = HairLobe::new(&HairParams { melanin: 0.1, ..Default::default() }, 0.0).eval(wo, wi).0;
        let dark = HairLobe::new(&HairParams { melanin: 1.0, ..Default::default() }, 0.0).eval(wo, wi).0;
        assert!(light.x > dark.x * 2.0, "{light} {dark}");
    }
}
