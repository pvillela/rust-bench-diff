/// Altnernative statistical hypothesis to the null hypothesis that there is no difference between the two distributions.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AltHyp {
    Lt,
    Gt,
    Ne,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hyp {
    Null,
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

/// Statistical hypothesis test result
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HypTestResult {
    p: f64,
    alpha: f64,
    alt_hyp: AltHyp,
    accepted: Hyp,
}

impl HypTestResult {
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

    pub fn p(&self) -> f64 {
        self.p
    }

    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    pub fn alt_hyp(&self) -> AltHyp {
        self.alt_hyp
    }

    pub fn accepted(&self) -> Hyp {
        self.accepted
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionWrtCi {
    Below,
    In,
    Above,
}

impl PositionWrtCi {
    pub fn position_of_value(value: f64, low: f64, high: f64) -> Self {
        match value {
            _ if value <= low => PositionWrtCi::Below,
            _ if low < value && value < high => PositionWrtCi::In,
            _ => PositionWrtCi::Above,
        }
    }
}
