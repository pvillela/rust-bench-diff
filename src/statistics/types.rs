/// Alternative statistical hypothesis to the null hypothesis of equality.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AltHyp {
    /// Less than
    Lt,
    /// Greater than
    Gt,
    /// Not equal
    Ne,
}

/// Statistical test hypothesis.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hyp {
    /// Null hypothesis of equality.
    Null,
    /// Alternative hypothesis.
    Alt(AltHyp),
}

impl Hyp {
    pub fn alt_hyp(&self) -> AltHyp {
        match self {
            Self::Null => AltHyp::Ne,
            Self::Alt(h) => *h,
        }
    }
}

/// Result of a statistical hypothesis test with a null hypothesis of equality.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HypTestResult {
    p: f64,
    alpha: f64,
    alt_hyp: AltHyp,
    accepted: Hyp,
}

impl HypTestResult {
    /// Creates a new instance of `Self`.
    ///
    /// Arguments:
    /// - `p` - the 'p' value for the test result.
    /// - `alpha` - determines the confidence level `(1-alpha)`.
    /// - `alt_hyp` - the alternative hypothesis.
    /// - `accepted` - the accepted hypothesis (null or alternative).
    pub fn new(p: f64, alpha: f64, alt_hyp: AltHyp) -> HypTestResult {
        Self {
            p,
            alpha,
            alt_hyp,
            accepted: if p >= alpha {
                Hyp::Null
            } else {
                Hyp::Alt(alt_hyp)
            },
        }
    }

    /// The 'p' value of the test result.
    pub fn p(&self) -> f64 {
        self.p
    }

    /// The 'alpha' for the test; determines the confidence level `(1-alpha)`.
    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    /// The alternative hypothesis for the test.
    pub fn alt_hyp(&self) -> AltHyp {
        self.alt_hyp
    }

    /// The hypothesis accepted by the test.
    pub fn accepted(&self) -> Hyp {
        self.accepted
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents the position of a value with respect to a confidence interval.
pub enum PositionWrtCi {
    /// The value is lower than the low end of the confidence interval.
    Below,
    /// The value is inside the confidence interval.
    In,
    /// The value is higher than the high end of the confidence interval.
    Above,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Confidence interval.
pub struct Ci(
    /// Low end of interval.
    pub f64,
    /// High end of interval.
    pub f64,
);

impl Ci {
    /// Returns the position of `value` with respect to `self`.
    pub fn position_of(&self, value: f64) -> PositionWrtCi {
        match value {
            _ if value <= self.0 => PositionWrtCi::Below,
            _ if self.0 < value && value < self.1 => PositionWrtCi::In,
            _ => PositionWrtCi::Above,
        }
    }
}
