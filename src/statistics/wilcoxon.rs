use super::{AltHyp, HypTestResult, z_to_p};
use hdrhistogram::{
    Histogram,
    iterators::{HistogramIterator, IterationValue, recorded::Iter},
};

#[cfg(test)]
#[derive(Debug)]
#[allow(unused)] // for debugging only
struct RankedItem {
    value: u64,
    count: u64,
    rank: f64,
}

#[cfg(test)]
#[allow(unused)]
fn wilcoxon_ranked_items_ties_sum_prod(
    hist_a: &Histogram<u64>,
    hist_b: &Histogram<u64>,
) -> (Vec<RankedItem>, u64) {
    fn rank_item(
        #[cfg(test)] value: u64,
        count_i: u64,
        count_other: u64,
        iter_i: &mut HistogramIterator<u64, Iter>,
        item_opt_i: &mut Option<IterationValue<u64>>,
        prev_rank: f64,
        ties_sum_prod: &mut u64,
    ) -> (RankedItem, f64) {
        let count = count_i + count_other;
        let rank = prev_rank + (count as f64 + 1.) / 2.;
        let item = RankedItem {
            #[cfg(test)]
            value,
            count: count_i,
            rank,
        };
        let new_prev_rank = prev_rank + count as f64;
        *item_opt_i = iter_i.next();
        *ties_sum_prod += (count - 1) * count * (count + 1);
        (item, new_prev_rank)
    }

    let mut ties_sum_prod = 0;

    let ranked_items_b: Vec<RankedItem> = {
        let mut items_b = Vec::<RankedItem>::with_capacity(hist_b.distinct_values());
        let mut iter_a = hist_a.iter_recorded();
        let mut iter_b = hist_b.iter_recorded();
        let (mut item_a_opt, mut item_b_opt) = (iter_a.next(), iter_b.next());
        let mut prev_rank = 0.;

        loop {
            match (&mut item_a_opt, &mut item_b_opt) {
                (Some(item_a), None) => {
                    let count = item_a.count_at_value();
                    let (_, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item_a.value_iterated_to(),
                        count,
                        0,
                        &mut iter_a,
                        &mut item_a_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    prev_rank = new_prev_rank;
                }

                (Some(item_a), Some(item_b))
                    if item_a.value_iterated_to() < item_b.value_iterated_to() =>
                {
                    let (_, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item_a.value_iterated_to(),
                        item_a.count_at_value(),
                        0,
                        &mut iter_a,
                        &mut item_a_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    prev_rank = new_prev_rank;
                }

                (None, Some(item_b)) => {
                    let (ranked_item, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item_b.value_iterated_to(),
                        item_b.count_at_value(),
                        0,
                        &mut iter_b,
                        &mut item_b_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    items_b.push(ranked_item);
                    prev_rank = new_prev_rank;
                }

                (Some(item_a), Some(item_b))
                    if item_a.value_iterated_to() > item_b.value_iterated_to() =>
                {
                    let (ranked_item, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item_b.value_iterated_to(),
                        item_b.count_at_value(),
                        0,
                        &mut iter_b,
                        &mut item_b_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    items_b.push(ranked_item);
                    prev_rank = new_prev_rank;
                }

                // if item_a.value_iterated_to() == item_b.value_iterated_to()
                (Some(item_a), Some(item_b)) => {
                    let count_a = item_a.count_at_value();
                    let count_b = item_b.count_at_value();
                    #[cfg(test)]
                    rank_item(
                        item_a.value_iterated_to(),
                        count_a,
                        count_b,
                        &mut iter_a,
                        &mut item_a_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    let (ranked_item, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item_b.value_iterated_to(),
                        count_b,
                        count_a,
                        &mut iter_b,
                        &mut item_b_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    items_b.push(ranked_item);
                    prev_rank = new_prev_rank;
                }

                (None, None) => break,
            }
        }

        items_b
    };

    (ranked_items_b, ties_sum_prod)
}

fn wilcoxon_rank_sum_ties_sum_prod(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> (f64, f64) {
    fn rank_item(
        count_i: u64,
        count_other: u64,
        iter_i: &mut HistogramIterator<u64, Iter>,
        item_opt_i: &mut Option<IterationValue<u64>>,
        prev_rank: f64,
        ties_sum_prod: &mut u64,
    ) -> (f64, f64) {
        let count = count_i + count_other;
        let rank = prev_rank + (count as f64 + 1.) / 2.;
        let rank_sum = count_i as f64 * rank;
        let new_prev_rank = prev_rank + count as f64;
        *item_opt_i = iter_i.next();
        *ties_sum_prod += (count - 1) * count * (count + 1);
        (rank_sum, new_prev_rank)
    }

    let mut ties_sum_prod = 0;

    let rank_sum_b: f64 = {
        let mut rank_sum_a = 0.;
        let mut rank_sum_b = 0.;
        let mut iter_a = hist_a.iter_recorded();
        let mut iter_b = hist_b.iter_recorded();
        let (mut item_a_opt, mut item_b_opt) = (iter_a.next(), iter_b.next());
        let mut prev_rank = 0.;

        loop {
            match (&mut item_a_opt, &mut item_b_opt) {
                (Some(item_a), None) => {
                    let count = item_a.count_at_value();
                    let (rank_sum, new_prev_rank) = rank_item(
                        count,
                        0,
                        &mut iter_a,
                        &mut item_a_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    rank_sum_a += rank_sum;
                    prev_rank = new_prev_rank;
                }

                (Some(item_a), Some(item_b))
                    if item_a.value_iterated_to() < item_b.value_iterated_to() =>
                {
                    let (rank_sum, new_prev_rank) = rank_item(
                        item_a.count_at_value(),
                        0,
                        &mut iter_a,
                        &mut item_a_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    rank_sum_a += rank_sum;
                    prev_rank = new_prev_rank;
                }

                (None, Some(item_b)) => {
                    let (rank_sum, new_prev_rank) = rank_item(
                        item_b.count_at_value(),
                        0,
                        &mut iter_b,
                        &mut item_b_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    rank_sum_b += rank_sum;
                    prev_rank = new_prev_rank;
                }

                (Some(item_a), Some(item_b))
                    if item_a.value_iterated_to() > item_b.value_iterated_to() =>
                {
                    let (rank_sum, new_prev_rank) = rank_item(
                        item_b.count_at_value(),
                        0,
                        &mut iter_b,
                        &mut item_b_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    rank_sum_b += rank_sum;
                    prev_rank = new_prev_rank;
                }

                // if item_a.value_iterated_to() == item_b.value_iterated_to()
                (Some(item_a), Some(item_b)) => {
                    let count_a = item_a.count_at_value();
                    let count_b = item_b.count_at_value();

                    let (rank_sum, _) = rank_item(
                        count_a,
                        count_b,
                        &mut iter_a,
                        &mut item_a_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    rank_sum_a += rank_sum;

                    let (rank_sum, new_prev_rank) = rank_item(
                        count_b,
                        count_a,
                        &mut iter_b,
                        &mut item_b_opt,
                        prev_rank,
                        &mut ties_sum_prod,
                    );
                    rank_sum_b += rank_sum;
                    prev_rank = new_prev_rank;
                }

                (None, None) => break,
            }
        }

        // Check rank-sum calculation.
        {
            let n_a = hist_a.len() as f64;
            let n_b = hist_b.len() as f64;
            let expected_rank_sum_a = (1. + n_a + n_b) * (n_a + n_b) / 2. - rank_sum_b;
            debug_assert_eq!(expected_rank_sum_a, rank_sum_a, "rank_sum_a check");
        }

        rank_sum_b
    };

    (rank_sum_b, ties_sum_prod as f64)
}

#[cfg(test)]
#[cfg(feature = "hypors")]
fn mann_whitney_u_b(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let (w, _) = wilcoxon_rank_sum_ties_sum_prod(hist_a, hist_b);
    let n_b = hist_b.len() as f64;
    w - n_b * (n_b + 1.) / 2.
}

#[cfg(test)]
#[cfg(feature = "hypors")]
fn mann_whitney_u_a(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let n_a = hist_a.len() as f64;
    let n_b = hist_b.len() as f64;
    (n_a * n_b) - mann_whitney_u_b(hist_a, hist_b)
}

#[cfg(test)]
#[cfg(feature = "hypors")]
fn mann_whitney_u(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    mann_whitney_u_b(hist_a, hist_b).min(mann_whitney_u_a(hist_a, hist_b))
}

pub fn wilcoxon_rank_sum_z(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let n_a = hist_a.len() as f64;
    let n_b = hist_b.len() as f64;
    let (w, ties_sum_prod) = wilcoxon_rank_sum_ties_sum_prod(hist_a, hist_b);
    let e0_w = n_b * (n_a + n_b + 1.) / 2.;
    let var0_w_base = n_a * n_b * (n_a + n_b + 1.) / 12.;
    let var0_w_ties_adjust = n_a * n_b / (12. * (n_a + n_b) * (n_a + n_b - 1.)) * ties_sum_prod;
    let var0_w = var0_w_base - var0_w_ties_adjust;
    let w_star = (w - e0_w) / var0_w.sqrt();

    -w_star
}

// #[cfg(test)]
pub fn wilcoxon_rank_sum_z_no_ties_adjust(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let n_a = hist_a.len() as f64;
    let n_b = hist_b.len() as f64;
    let (w, _) = wilcoxon_rank_sum_ties_sum_prod(hist_a, hist_b);
    let e0_w = n_b * (n_a + n_b + 1.) / 2.;
    let var0_w_base = n_a * n_b * (n_a + n_b + 1.) / 12.;
    let var0_w_ties_adjust = 0.;
    let var0_w = var0_w_base - var0_w_ties_adjust;
    let w_star = (w - e0_w) / var0_w.sqrt();

    -w_star
}

pub fn wilcoxon_rank_sum_p(
    hist_a: &Histogram<u64>,
    hist_b: &Histogram<u64>,
    alt_hyp: AltHyp,
) -> f64 {
    let z = wilcoxon_rank_sum_z(hist_a, hist_b);
    z_to_p(z, alt_hyp)
}

#[cfg(test)]
fn wilcoxon_rank_sum_p_no_ties_adjust(
    hist_a: &Histogram<u64>,
    hist_b: &Histogram<u64>,
    alt_hyp: AltHyp,
) -> f64 {
    let z = wilcoxon_rank_sum_z_no_ties_adjust(hist_a, hist_b);
    z_to_p(z, alt_hyp)
}

pub fn wilcoxon_rank_sum_test(
    hist_a: &Histogram<u64>,
    hist_b: &Histogram<u64>,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = wilcoxon_rank_sum_p(hist_a, hist_b, alt_hyp);
    HypTestResult::new(p, alpha, alt_hyp)
}

#[cfg(test)]
pub fn wilcoxon_rank_sum_test_no_ties_adjust(
    hist_a: &Histogram<u64>,
    hist_b: &Histogram<u64>,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = wilcoxon_rank_sum_p_no_ties_adjust(hist_a, hist_b, alt_hyp);
    HypTestResult::new(p, alpha, alt_hyp)
}

#[cfg(test)]
mod base_test {
    use crate::statistics::{
        AltHyp,
        wilcoxon::{wilcoxon_rank_sum_p, wilcoxon_rank_sum_ties_sum_prod},
    };
    use hdrhistogram::Histogram;

    #[test]
    fn test_w() {
        // Based on https://learning.oreilly.com/library/view/nonparametric-statistical-methods/9781118553299/9781118553299c04.xhtml#c04_level1_2
        // Nonparametric Statistical Methods, 3rd Edition, by Myles Hollander, Douglas A. Wolfe, Eric Chicken
        // Example 4.1.

        let sample_a0 = vec![0.73, 0.80, 0.83, 1.04, 1.38, 1.45, 1.46, 1.64, 1.89, 1.91];
        let sample_b0 = vec![0.74, 0.88, 0.90, 1.15, 1.21];

        let sample_a = sample_a0
            .into_iter()
            .map(|x| (x * 100.) as u64)
            .collect::<Vec<_>>();
        let sample_b = sample_b0
            .into_iter()
            .map(|x| (x * 100.) as u64)
            .collect::<Vec<_>>();

        let mut hist_a = Histogram::new_with_max(200, 3).unwrap();
        let mut hist_b = Histogram::new_with_max(200, 3).unwrap();

        for v in &sample_a {
            hist_a.record(*v).unwrap();
        }

        for v in &sample_b {
            hist_b.record(*v).unwrap();
        }

        let expected_w = 30.;
        let (actual_w, _) = wilcoxon_rank_sum_ties_sum_prod(&hist_a, &hist_b);
        assert_eq!(expected_w, actual_w, "w comparison");

        let expected_p = 0.2544;
        let actual_p = wilcoxon_rank_sum_p(&hist_a, &hist_b, AltHyp::Ne);
        // assert_eq!(expected_p, actual_p, "p comparison"); // this fails because I'm using normal approximation
        println!("expected_p={expected_p}, actual_p={actual_p}");
    }
}

#[cfg(test)]
#[cfg(feature = "hypors")]
mod test_with_hypors {
    use super::*;
    use crate::{dev_utils::ApproxEq, statistics::AltHyp};
    use hdrhistogram::Histogram;
    use hypors::{common::TailType, mann_whitney::u_test};
    use polars::prelude::*;

    const ALPHA: f64 = 0.05;

    fn process_samples(sample_a: Vec<u64>, sample_b: Vec<u64>, hist_max: u64, hist_sigfig: u8) {
        let mut hist_a = Histogram::new_with_max(hist_max, hist_sigfig).unwrap();
        let mut hist_b = Histogram::new_from(&hist_a);

        for v in &sample_a {
            hist_a.record(*v).unwrap();
        }

        for v in &sample_b {
            hist_b.record(*v).unwrap();
        }

        let (ranked_items, _) = wilcoxon_ranked_items_ties_sum_prod(&mut hist_a, &mut hist_b);
        println!("{ranked_items:?}");

        let (rank_sum_b, _) = wilcoxon_rank_sum_ties_sum_prod(&mut hist_a, &mut hist_b);
        println!("rank_sum_b={rank_sum_b}");

        let n_a = sample_a.len() as f64;
        let n_b = sample_b.len() as f64;
        let rank_sum_a = (1. + n_a + n_b) * (n_a + n_b) / 2. - rank_sum_b;
        println!("rank_sum_a={rank_sum_a}");

        let wilcoxon_rank_sum_a_lt_b_p = wilcoxon_rank_sum_p(&mut hist_a, &mut hist_b, AltHyp::Lt);
        println!("wilcoxon_rank_sum_a_lt_b_p={wilcoxon_rank_sum_a_lt_b_p}");
        let wilcoxon_rank_sum_a_lt_b_p_no_ties_adjust: f64 =
            wilcoxon_rank_sum_p_no_ties_adjust(&mut hist_a, &mut hist_b, AltHyp::Lt);
        println!(
            "wilcoxon_rank_sum_a_lt_b_p_no_ties_adjust={wilcoxon_rank_sum_a_lt_b_p_no_ties_adjust}"
        );
        let wilcoxon_rank_sum_a_gt_b_p = wilcoxon_rank_sum_p(&mut hist_a, &mut hist_b, AltHyp::Gt);
        println!("wilcoxon_rank_sum_a_gt_b_p={wilcoxon_rank_sum_a_gt_b_p}");
        let wilcoxon_rank_sum_a_ne_b_p: f64 =
            wilcoxon_rank_sum_p(&mut hist_a, &mut hist_b, AltHyp::Ne);
        println!("wilcoxon_rank_sum_a_ne_b_p={wilcoxon_rank_sum_a_ne_b_p}");
        let wilcoxon_rank_sum_a_ne_b_p_no_ties_adjust: f64 =
            wilcoxon_rank_sum_p_no_ties_adjust(&mut hist_a, &mut hist_b, AltHyp::Ne);
        println!(
            "wilcoxon_rank_sum_a_ne_b_p_no_ties_adjust={wilcoxon_rank_sum_a_ne_b_p_no_ties_adjust}"
        );

        let mann_whitney_a_lt_b_u = mann_whitney_u_b(&hist_a, &hist_b);
        println!("mann_whitney_a_lt_b_u={mann_whitney_a_lt_b_u}");
        let mann_whitney_a_gt_b_u = mann_whitney_u_a(&hist_a, &hist_b);
        println!("mann_whitney_a_gt_b_u={mann_whitney_a_gt_b_u}");
        let mann_whitney_a_ne_b_u = mann_whitney_u(&hist_a, &hist_b);
        println!("mann_whitney_a_ne_b_u={mann_whitney_a_ne_b_u}");

        {
            let series_a = Series::new(
                "a".into(),
                sample_a.iter().map(|x| *x as f64).collect::<Vec<_>>(),
            );
            let series_b = Series::new(
                "b".into(),
                sample_b.iter().map(|x| *x as f64).collect::<Vec<_>>(),
            );

            {
                let result = u_test(&series_a, &series_b, ALPHA, TailType::Two);
                println!("result(two tail)={result:?}");

                let result = result.unwrap();

                println!("U Statistic: {}", result.test_statistic);
                println!("P-value: {}", result.p_value);
                println!("Reject Null: {}", result.reject_null);

                assert_eq!(
                    result.test_statistic, mann_whitney_a_ne_b_u,
                    "comparison of U statistics"
                );

                assert_eq!(
                    result.p_value.round_to_sig_decimals(5),
                    wilcoxon_rank_sum_a_ne_b_p_no_ties_adjust.round_to_sig_decimals(5),
                    "comparison of p values for non-equality (no ties adjustment)"
                );
                // Below fails because `hypors` does not compute ties adjustment.
                // assert_eq!(
                //     result.p_value.round_to_sig_decimals(5),
                //     wilcoxon_rank_sum_a_ne_b_p.round_to_sig_decimals(5),
                //     "comparison of p values for non-equality"
                // );
            }
        }
    }

    fn expand_sample(sample: &[u64], delta: u64, nrepeats: usize) -> Vec<u64> {
        let mut cumulative = Vec::with_capacity(sample.len() * nrepeats);
        let mut curr = Vec::from(sample);
        for _ in 0..nrepeats {
            let next = curr.iter().map(|x| x + delta).collect::<Vec<_>>();
            cumulative.append(&mut next.clone());
            curr = next;
        }
        cumulative
    }

    #[test]
    fn test_book_data() {
        println!(
            "***** Samples from Nonparametric Statistical Methods, 3rd Edition, Example 4.1. *****"
        );
        {
            let sample_a0 = vec![0.73, 0.80, 0.83, 1.04, 1.38, 1.45, 1.46, 1.64, 1.89, 1.91];
            let sample_b0 = vec![0.74, 0.88, 0.90, 1.15, 1.21];

            let sample_a = sample_a0
                .into_iter()
                .map(|x| (x * 100.) as u64)
                .collect::<Vec<_>>();
            let sample_b = sample_b0
                .into_iter()
                .map(|x| (x * 100.) as u64)
                .collect::<Vec<_>>();

            process_samples(sample_a, sample_b, 200, 3);
        }
    }

    #[test]
    fn test_contrived_data() {
        let sample_a0 = vec![85, 90, 78, 92, 88, 76, 95, 89, 91, 82];
        let sample_b0 = vec![70, 85, 80, 90, 75, 88, 92, 79, 86, 81];

        println!("***** Original samples *****");
        {
            let sample_a = sample_a0.clone();
            let sample_b = sample_b0.clone();

            {
                let mut sorted_a = sample_a.clone();
                sorted_a.sort();

                let mut sorted_b = sample_b.clone();
                sorted_b.sort();

                let mut combined = sample_a.iter().chain(sample_b.iter()).collect::<Vec<_>>();
                combined.sort();

                let exp_ranks_b = [1., 2., 5., 6., 7., 9.5, 11., 12.5, 15.5, 18.5];
                let exp_rank_sum_b = exp_ranks_b.iter().sum::<f64>();

                println!("sorted_a={sorted_a:?}");
                println!("sorted_b={sorted_b:?}");
                println!("combined={combined:?}");
                println!("exp_ranks_b={exp_ranks_b:?}");
                println!("exp_rank_sum_b={exp_rank_sum_b:?}");
            }
            process_samples(sample_a, sample_b, 300, 3);
        }

        println!();
        println!("***** Magnified samples *****");
        {
            let delta = 30;
            let nrepeats = 5;
            let sample_a = expand_sample(&sample_a0, delta, nrepeats);
            let sample_b = expand_sample(&sample_b0, delta, nrepeats);
            process_samples(sample_a, sample_b, 300, 3);
        }

        println!();
        println!("***** sample_a < sample_b *****");
        {
            let sample_a = sample_a0.clone();
            let delta = 2;
            let sample_b = sample_a.iter().map(|x| x + delta).collect::<Vec<_>>();
            process_samples(sample_a, sample_b, 300, 3);
        }
    }
}
