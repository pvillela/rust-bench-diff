use super::core::{AltHyp, Ci, HypTestResult, SampleMoments};
use statrs::distribution::{ContinuousCDF, Normal, StudentsT};

/// Returns the p-value for a z-value from the standard normal.
/// `alt_hyp` is the alternative hypothesis. The null hypothesis is that the sample distribution's mean is 0.
pub fn z_to_p(z: f64, alt_hyp: AltHyp) -> f64 {
    let normal = Normal::standard();

    match alt_hyp {
        AltHyp::Lt => normal.cdf(z),
        AltHyp::Gt => normal.cdf(-z),
        AltHyp::Ne => normal.cdf(-z.abs()) * 2.,
    }
}

/// Returns the p-value for a t-value from the Student distribution with location 0, scale 1, and `df` degrees of freedom.
/// `alt_hyp` is the alternative hypothesis. The null hypothesis is that the sample distribution's mean is 0.
pub fn t_to_p(t: f64, df: f64, alt_hyp: AltHyp) -> f64 {
    let stud = StudentsT::new(0., 1., df).expect("degrees of freedom must be > 0");

    match alt_hyp {
        AltHyp::Lt => stud.cdf(t),
        AltHyp::Gt => stud.cdf(-t),
        AltHyp::Ne => stud.cdf(-t.abs()) * 2.,
    }
}

/// Returns the probability that the standard normal distribution produces a value greater than `alpha`.
pub fn z_alpha(alpha: f64) -> f64 {
    let normal = Normal::standard();
    normal.cdf(-alpha)
}

/// Returns the probability that the Student distribution with location 0, scale 1, and `df` degrees of freedom
/// produces a value greater than `alpha`.
pub fn t_alpha(df: f64, alpha: f64) -> f64 {
    let stud = StudentsT::new(0., 1., df).expect("degrees of freedom must be > 0");
    stud.cdf(-alpha)
}

pub fn welch_t(moments_a: &SampleMoments, moments_b: &SampleMoments) -> f64 {
    let n_a = moments_a.nf();
    let n_b = moments_b.nf();
    let dx = moments_a.mean() - moments_b.mean();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let s_dx = (s2_mean_a + s2_mean_b).sqrt();
    dx / s_dx
}

pub fn welch_df(moments_a: &SampleMoments, moments_b: &SampleMoments) -> f64 {
    let n_a = moments_a.nf();
    let n_b = moments_b.nf();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let numerator = (s2_mean_a + s2_mean_b).powi(2);
    let denominator = s2_mean_a.powi(2) / (n_a - 1.) + s2_mean_b.powi(2) / (n_b - 1.);
    numerator / denominator
}

pub fn welch_p(moments_a: &SampleMoments, moments_b: &SampleMoments, alt_hyp: AltHyp) -> f64 {
    let t = welch_t(moments_a, moments_b);
    let df = welch_df(moments_a, moments_b);
    t_to_p(t, df, alt_hyp)
}

pub fn welch_alt_hyp_ci(
    moments_a: &SampleMoments,
    moments_b: &SampleMoments,
    alt_hyp: AltHyp,
    alpha: f64,
) -> Ci {
    let n_a = moments_a.nf();
    let n_b = moments_b.nf();
    let dx = moments_a.mean() - moments_b.mean();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let df = welch_df(moments_a, moments_b);

    let stud = StudentsT::new(0., 1., df).expect("Welch degrees of freedom must be > 0");
    let t0 = match alt_hyp {
        AltHyp::Ne => -stud.inverse_cdf(alpha / 2.),
        _ => -stud.inverse_cdf(alpha),
    };

    let mid = dx;
    let delta = (s2_mean_a + s2_mean_b).sqrt() * t0;

    match alt_hyp {
        AltHyp::Lt => Ci(-f64::INFINITY, mid + delta),
        AltHyp::Ne => Ci(mid - delta, mid + delta),
        AltHyp::Gt => Ci(mid - delta, f64::INFINITY),
    }
}

pub fn welch_ci(moments_a: &SampleMoments, moments_b: &SampleMoments, alpha: f64) -> Ci {
    welch_alt_hyp_ci(moments_a, moments_b, AltHyp::Ne, alpha)
}

pub fn welch_test(
    moments_a: &SampleMoments,
    moments_b: &SampleMoments,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = welch_p(moments_a, moments_b, alt_hyp);
    HypTestResult::new(p, alpha, alt_hyp)
}

pub fn student_one_sample_t(moments: &SampleMoments, mu0: f64) -> f64 {
    let n = moments.nf();
    let mean = moments.mean();
    let s = moments.stdev();
    (mean - mu0) / s * n.sqrt()
}

pub fn student_one_sample_df(moments: &SampleMoments) -> f64 {
    moments.nf() - 1.
}

pub fn student_one_sample_p(moments: &SampleMoments, mu0: f64, alt_hyp: AltHyp) -> f64 {
    let t = student_one_sample_t(moments, mu0);
    let df = student_one_sample_df(moments);
    t_to_p(t, df, alt_hyp)
}

pub fn student_one_sample_alt_hyp_ci(moments: &SampleMoments, alt_hyp: AltHyp, alpha: f64) -> Ci {
    let df = student_one_sample_df(moments);

    let stud = StudentsT::new(0., 1., df)
        .expect("can't happen: degrees of freedom is always >= 3 by construction");
    let t0 = match alt_hyp {
        AltHyp::Ne => -stud.inverse_cdf(alpha / 2.),
        _ => -stud.inverse_cdf(alpha),
    };

    let mid = moments.mean();
    let delta = (moments.stdev() / moments.nf().sqrt()) * t0;

    match alt_hyp {
        AltHyp::Lt => Ci(-f64::INFINITY, mid + delta),
        AltHyp::Ne => Ci(mid - delta, mid + delta),
        AltHyp::Gt => Ci(mid - delta, f64::INFINITY),
    }
}

pub fn student_one_sample_ci(moments: &SampleMoments, alpha: f64) -> Ci {
    student_one_sample_alt_hyp_ci(moments, AltHyp::Ne, alpha)
}

pub fn student_one_sample_test(
    moments: &SampleMoments,
    mu0: f64,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = student_one_sample_p(moments, mu0, alt_hyp);
    HypTestResult::new(p, alpha, alt_hyp)
}

#[cfg(test)]
#[cfg(feature = "test_support")]
mod test {
    //! Used R's t.test function to generate expected values.
    //! https://www.rdocumentation.org/packages/stats/versions/3.6.2/topics/t.test

    use super::*;
    use crate::{
        basic_stats::core::{AltHyp, Hyp},
        dev_utils::ApproxEq,
    };

    const ALPHA: f64 = 0.05;
    const EPSILON: f64 = 0.0005;

    fn check_welch(
        dataset_a: &[f64],
        dataset_b: &[f64],
        alt_hyp: AltHyp,
        exp_t: f64,
        exp_df: f64,
        exp_p: f64,
        exp_ci: Ci,
        exp_accept_hyp: Hyp,
    ) {
        let moments_a = SampleMoments::from_slice(dataset_a);
        let moments_b = SampleMoments::from_slice(dataset_b);

        let t = welch_t(&moments_a, &moments_b);
        let df = welch_df(&moments_a, &moments_b);
        let p = t_to_p(t, df, alt_hyp);
        let ci = welch_alt_hyp_ci(&moments_a, &moments_b, alt_hyp, ALPHA);
        let res = welch_test(&moments_a, &moments_b, alt_hyp, ALPHA);

        assert!(
            exp_t.approx_eq(t, EPSILON),
            "alt_hyp={alt_hyp:?} -- exp_t={exp_t}, t={t}"
        );
        assert!(
            exp_df.approx_eq(df, EPSILON),
            "alt_hyp={alt_hyp:?} -- exp_df={exp_df}, df={df}"
        );
        assert!(
            exp_p.approx_eq(p, EPSILON),
            "alt_hyp={alt_hyp:?} -- exp_p={exp_p}, p={p}"
        );
        assert!(
            exp_ci.0.approx_eq(ci.0, EPSILON) || exp_ci.0.is_infinite() && ci.0.is_infinite(),
            "alt_hyp={alt_hyp:?} -- exp_ci.0={}, ci.0={}",
            exp_ci.0,
            ci.0
        );
        assert!(
            exp_ci.1.approx_eq(ci.1, EPSILON) || exp_ci.1.is_infinite() && ci.1.is_infinite(),
            "alt_hyp={alt_hyp:?} -- exp_ci.1={}, ci.1={}",
            exp_ci.1,
            ci.1
        );

        assert_eq!(p, res.p(), "alt_hyp={alt_hyp:?} -- res.p");
        assert_eq!(ALPHA, res.alpha(), "alt_hyp={alt_hyp:?} -- res.alpha");
        assert_eq!(alt_hyp, res.alt_hyp(), "alt_hyp={alt_hyp:?} -- res.alt_hyp");
        assert_eq!(
            exp_accept_hyp,
            res.accepted(),
            "alt_hyp={alt_hyp:?} -- res.accepted"
        );
    }

    fn check_student(
        dataset: &[f64],
        mu0: f64,
        alt_hyp: AltHyp,
        exp_t: f64,
        exp_df: f64,
        exp_p: f64,
        exp_ci: Ci,
        exp_accept_hyp: Hyp,
    ) {
        let moments = SampleMoments::from_slice(dataset);

        let t = student_one_sample_t(&moments, mu0);
        let df = student_one_sample_df(&moments);
        let p = t_to_p(t, df, alt_hyp);
        let ci = student_one_sample_alt_hyp_ci(&moments, alt_hyp, ALPHA);
        let res = student_one_sample_test(&moments, mu0, alt_hyp, ALPHA);

        assert!(
            exp_t.approx_eq(t, EPSILON),
            "alt_hyp={alt_hyp:?} -- exp_t={exp_t}, t={t}"
        );
        assert!(
            exp_df.approx_eq(df, EPSILON),
            "alt_hyp={alt_hyp:?} -- exp_df={exp_df}, df={df}"
        );
        assert!(
            exp_p.approx_eq(p, EPSILON),
            "alt_hyp={alt_hyp:?} -- exp_p={exp_p}, p={p}"
        );
        assert!(
            exp_ci.0.approx_eq(ci.0, EPSILON) || exp_ci.0.is_infinite() && ci.0.is_infinite(),
            "alt_hyp={alt_hyp:?} -- exp_ci.0={}, ci.0={}",
            exp_ci.0,
            ci.0
        );
        assert!(
            exp_ci.1.approx_eq(ci.1, EPSILON) || exp_ci.1.is_infinite() && ci.1.is_infinite(),
            "alt_hyp={alt_hyp:?} -- exp_ci.1={}, ci.1={}",
            exp_ci.1,
            ci.1
        );

        assert_eq!(p, res.p(), "alt_hyp={alt_hyp:?} -- res.p");
        assert_eq!(ALPHA, res.alpha(), "alt_hyp={alt_hyp:?} -- res.alpha");
        assert_eq!(alt_hyp, res.alt_hyp(), "alt_hyp={alt_hyp:?} -- res.alt_hyp");
        assert_eq!(
            exp_accept_hyp,
            res.accepted(),
            "alt_hyp={alt_hyp:?} -- res.accepted"
        );
    }

    #[test]
    fn test_welch_eq() {
        let a = [14., 15., 15., 15., 16., 18., 22., 23., 24., 25., 25.];
        let b = [
            10., 12., 14., 15., 18., 22., 24., 27., 31., 33., 34., 34., 34.,
        ];

        let exp_t = -1.5379;
        let exp_df = 18.137;

        {
            let alt_hyp = AltHyp::Lt;
            let exp_p = 0.07067;
            let exp_ci = Ci(-f64::INFINITY, 0.5616789);
            check_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci, Hyp::Null);
        }

        {
            let alt_hyp = AltHyp::Ne;
            let exp_p = 0.1413;
            let exp_ci = Ci(-10.453875, 1.614714);
            check_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci, Hyp::Null);
        }

        {
            let alt_hyp = AltHyp::Gt;
            let exp_p = 0.9293;
            let exp_ci = Ci(-9.40084, f64::INFINITY);
            check_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci, Hyp::Null);
        }
    }

    #[test]
    fn test_welch_gt() {
        let a = [24., 28., 32., 29., 35., 36., 30., 32., 25., 31.];
        let b = [5., 10., 25., 15., 16., 20.];

        let exp_t = 4.7857;
        let exp_df = 6.8409;

        {
            let alt_hyp = AltHyp::Lt;
            let exp_accept_hyp = Hyp::Null;
            let exp_p = 0.9989;
            let exp_ci = Ci(-f64::INFINITY, 21.00566);
            check_welch(
                &a,
                &b,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Ne;
            let exp_accept_hyp = Hyp::Alt(AltHyp::Ne);
            let exp_p = 0.00213;
            let exp_ci = Ci(7.57018, 22.49649);
            check_welch(
                &a,
                &b,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Gt;
            let exp_accept_hyp = Hyp::Alt(AltHyp::Gt);
            let exp_p = 0.001065;
            let exp_ci = Ci(9.061005, f64::INFINITY);
            check_welch(
                &a,
                &b,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }
    }

    fn student_data() -> Vec<f64> {
        vec![
            20.70, 27.46, 22.15, 19.85, 21.29, 24.75, 20.75, 22.91, 25.34, 20.33, 21.54, 21.08,
            22.14, 19.56, 21.10, 18.04, 24.12, 19.95, 19.72, 18.28, 16.26, 17.46, 20.53, 22.12,
            25.06, 22.44, 19.08, 19.88, 21.39, 22.33, 25.79,
        ]
    }
    #[test]
    fn test_student_lt() {
        let data = student_data();

        let mu0 = 23.;
        let exp_t = -3.505;
        let exp_df = 30.;

        {
            let alt_hyp = AltHyp::Lt;
            let exp_accept_hyp = Hyp::Alt(AltHyp::Lt);
            let exp_p = 0.0007288;
            let exp_ci = Ci(-f64::INFINITY, 22.17479);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Ne;
            let exp_accept_hyp = Hyp::Alt(AltHyp::Ne);
            let exp_p = 0.001458;
            let exp_ci = Ci(20.46771, 22.33229);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Gt;
            let exp_accept_hyp = Hyp::Null;
            let exp_p = 0.9993;
            let exp_ci = Ci(20.62521, f64::INFINITY);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }
    }
    #[test]
    fn test_student_eq() {
        let data = student_data();

        let mu0 = 21.;
        let exp_t = 0.87624;
        let exp_df = 30.;

        {
            let alt_hyp = AltHyp::Lt;
            let exp_accept_hyp = Hyp::Null;
            let exp_p = 0.8061;
            let exp_ci = Ci(-f64::INFINITY, 22.17479);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Ne;
            let exp_accept_hyp = Hyp::Null;
            let exp_p = 0.3879;
            let exp_ci = Ci(20.46771, 22.33229);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Gt;
            let exp_accept_hyp = Hyp::Null;
            let exp_p = 0.1939;
            let exp_ci = Ci(20.62521, f64::INFINITY);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }
    }

    #[test]
    fn test_student_gt() {
        let data = student_data();

        let mu0 = 20.;
        let exp_t = 3.0668;
        let exp_df = 30.;

        {
            let alt_hyp = AltHyp::Lt;
            let exp_accept_hyp = Hyp::Null;
            let exp_p = 0.9977;
            let exp_ci = Ci(-f64::INFINITY, 22.17479);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Ne;
            let exp_accept_hyp = Hyp::Alt(AltHyp::Ne);
            let exp_p = 0.004553;
            let exp_ci = Ci(20.46771, 22.33229);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }

        {
            let alt_hyp = AltHyp::Gt;
            let exp_accept_hyp = Hyp::Alt(AltHyp::Gt);
            let exp_p = 0.002276;
            let exp_ci = Ci(20.62521, f64::INFINITY);
            check_student(
                &data,
                mu0,
                alt_hyp,
                exp_t,
                exp_df,
                exp_p,
                exp_ci,
                exp_accept_hyp,
            );
        }
    }
}
