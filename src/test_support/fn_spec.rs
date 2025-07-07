use bench_utils::busy_work;
use rand::{SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, LogNormal};
use std::fmt::{Debug, Display};

/// Variance selector
#[derive(Debug, Clone, Copy)]
pub enum VarSelector {
    No,
    Lo,
    Hi,
}

impl VarSelector {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "novar" => Some(Self::No),
            "lovar" => Some(Self::Lo),
            "hivar" => Some(Self::Hi),
            _ => None,
        }
    }

    pub fn default_variance(&self) -> f64 {
        match self {
            Self::No => 0.,
            Self::Lo => 1.2_f64.ln() / 2.,
            Self::Hi => 2.4_f64.ln() / 2.,
        }
    }
}

impl Display for VarSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => f.write_str("novar"),
            Self::Lo => f.write_str("lovar"),
            Self::Hi => f.write_str("hivar"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FnSpec {
    pub base_median_factor: f64,
    pub var_selector: VarSelector,
}

impl Display for FnSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}@{}",
            self.base_median_factor, self.var_selector
        ))
    }
}

impl Debug for FnSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl PartialEq for FnSpec {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for FnSpec {}

impl PartialOrd for FnSpec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl Ord for FnSpec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl FnSpec {
    pub fn parse(s: &str) -> FnSpec {
        let err_msg = || {
            format!(
                "fn spec string must be of the form <positive number>@<novar | lovar | hivar>, but was \"{s}\""
            )
        };
        let spec_vec = s.split("@").collect::<Vec<_>>();
        assert!(spec_vec.len() == 2, "{}", err_msg());
        let base_median_factor = spec_vec[0]
            .parse::<f64>()
            .map_err(|_| s.parse::<u64>())
            .unwrap_or_else(|_| panic!("{}", err_msg()));
        assert!(base_median_factor > 0., "{}", err_msg());
        let var_selector =
            VarSelector::parse(spec_vec[1]).unwrap_or_else(|| panic!("{}", err_msg()));
        FnSpec {
            base_median_factor,
            var_selector,
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum MyFnMut {
    Det {
        median_effort: u32,
    },

    NonDet {
        median_effort: u32,
        lognormal: LogNormal<f64>,
        rng: StdRng,
    },
}

impl MyFnMut {
    fn new_deterministic(base_median_effort: u32, base_median_factor: f64) -> Self {
        Self::Det {
            median_effort: (base_median_effort as f64 * base_median_factor) as u32,
        }
    }

    fn new_non_deterministic(
        base_median_effort: u32,
        base_median_factor: f64,
        stdev_ln: f64,
    ) -> Self {
        let mu = 0.0_f64;
        let sigma = stdev_ln;
        Self::NonDet {
            median_effort: (base_median_effort as f64 * base_median_factor) as u32,
            lognormal: LogNormal::new(mu, sigma).expect("stdev_ln must be > 0"),
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    pub fn new(base_median_effort: u32, fn_spec: FnSpec) -> Self {
        match fn_spec.var_selector {
            VarSelector::No => {
                Self::new_deterministic(base_median_effort, fn_spec.base_median_factor)
            }

            _ => Self::new_non_deterministic(
                base_median_effort,
                fn_spec.base_median_factor,
                fn_spec.var_selector.default_variance(),
            ),
        }
    }

    pub fn invoke(&mut self) {
        match self {
            Self::Det { median_effort } => {
                busy_work(*median_effort);
            }

            Self::NonDet {
                median_effort,
                lognormal,
                rng,
            } => {
                let factor = lognormal.sample(rng);
                let effort = (*median_effort as f64) * factor;
                busy_work(effort as u32);
            }
        }
    }
}
