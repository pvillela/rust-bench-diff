use hdrhistogram::{
    Histogram,
    iterators::{HistogramIterator, IterationValue, recorded::Iter},
};
use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Debug)]
struct RankedItem {
    #[allow(unused)] // for debugging only
    #[cfg(test)]
    value: u64,
    count: u64,
    rank: f64,
}

fn wilcoxon_ranked_items(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> Vec<RankedItem> {
    fn rank_item(
        #[cfg(test)] value: u64,
        count_i: u64,
        count_other: u64,
        iter_i: &mut HistogramIterator<u64, Iter>,
        item_opt_i: &mut Option<IterationValue<u64>>,
        prev_rank: f64,
    ) -> (RankedItem, f64) {
        let count = count_i + count_other;
        let rank = prev_rank + (count as f64 + 1.0) / 2.0;
        let item = RankedItem {
            #[cfg(test)]
            value,
            count: count_i,
            rank,
        };
        let new_prev_rank = prev_rank + count as f64;
        *item_opt_i = iter_i.next();
        (item, new_prev_rank)
    }

    let ranked_items_b: Vec<RankedItem> = {
        let mut items_b = Vec::<RankedItem>::with_capacity(hist_b.distinct_values());
        let mut iter_a = hist_a.iter_recorded();
        let mut iter_b = hist_b.iter_recorded();
        let (mut item_a_opt, mut item_b_opt) = (iter_a.next(), iter_b.next());
        let mut prev_rank = 0.0;

        loop {
            match (&mut item_a_opt, &mut item_b_opt) {
                (Some(item_a), None) => {
                    let (_, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item_a.value_iterated_to(),
                        item_a.count_at_value(),
                        0,
                        &mut iter_a,
                        &mut item_a_opt,
                        prev_rank,
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
                    );
                    let (ranked_item, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item_b.value_iterated_to(),
                        count_b,
                        count_a,
                        &mut iter_b,
                        &mut item_b_opt,
                        prev_rank,
                    );
                    items_b.push(ranked_item);
                    prev_rank = new_prev_rank;
                }

                (None, None) => break,
            }
        }

        items_b
    };

    ranked_items_b
}

fn wilcoxon_rank_sum(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let ranked_items_b = wilcoxon_ranked_items(hist_a, hist_b);
    ranked_items_b.iter().map(|y| y.count as f64 * y.rank).sum()
}

#[cfg(test)]
fn mann_whitney_a_lt_b_u(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let w = wilcoxon_rank_sum(hist_a, hist_b);
    let n_b = hist_b.len() as f64;
    w - n_b * (n_b + 1.0) / 2.0
}

#[cfg(test)]
fn mann_whitney_a_gt_b_u(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let n_a = hist_a.len() as f64;
    let n_b = hist_b.len() as f64;
    (n_a * n_b) - mann_whitney_a_lt_b_u(hist_a, hist_b)
}

fn wilcoxon_rank_sum_a_lt_b_z(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let n_a = hist_a.len() as f64;
    let n_b = hist_b.len() as f64;
    let w: f64 = wilcoxon_rank_sum(hist_a, hist_b);
    let e0_w = n_b * (n_a + n_b + 1.0) / 2.0;
    let var0_w = n_a * n_b * (n_a + n_b + 1.0) / 12.0;
    let w_star = (w - e0_w) / var0_w.sqrt();

    -w_star
}

pub fn wilcoxon_rank_sum_a_lt_b_p(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let z = wilcoxon_rank_sum_a_lt_b_z(hist_a, hist_b);
    let normal = Normal::standard();
    normal.cdf(z)
}

fn wilcoxon_rank_sum_a_gt_b_z(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    -wilcoxon_rank_sum_a_lt_b_z(hist_a, hist_b)
}

pub fn wilcoxon_rank_sum_a_gt_b_p(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let z = wilcoxon_rank_sum_a_gt_b_z(hist_a, hist_b);
    let normal = Normal::standard();
    normal.cdf(z)
}

fn wilcoxon_rank_sum_a_ne_b_z(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    -wilcoxon_rank_sum_a_lt_b_z(hist_a, hist_b).abs()
}

pub fn wilcoxon_rank_sum_a_ne_b_p(hist_a: &Histogram<u64>, hist_b: &Histogram<u64>) -> f64 {
    let z = wilcoxon_rank_sum_a_ne_b_z(hist_a, hist_b);
    let normal = Normal::standard();
    normal.cdf(z) * 2.0
}

#[cfg(test)]
mod test {
    use crate::{
        statistics::wilcoxon::{mann_whitney_a_gt_b_u, mann_whitney_a_lt_b_u},
        wilcoxon_rank_sum_a_gt_b_p, wilcoxon_rank_sum_a_lt_b_p, wilcoxon_rank_sum_a_ne_b_p,
    };

    use super::wilcoxon_ranked_items;
    use hdrhistogram::Histogram;
    use hypors::{common::TailType, mann_whitney::u_test};
    use polars::prelude::*;

    const ALPHA: f64 = 0.05;

    fn round_to_sig_decimals(x: f64, n: u32) -> f64 {
        (x * n as f64).round() / (n as f64)
    }

    fn process_samples(sample_a: Vec<u64>, sample_b: Vec<u64>) {
        let mut sorted_a = sample_a.clone();
        sorted_a.sort();

        let mut sorted_b = sample_b.clone();
        sorted_b.sort();

        let mut combined = sample_a.iter().chain(sample_b.iter()).collect::<Vec<_>>();
        combined.sort();

        let exp_ranks_b = [1.0, 2.0, 5.0, 6.0, 7.0, 9.5, 11.0, 12.5, 15.5, 18.5];
        let exp_rank_sum_b = exp_ranks_b.iter().sum::<f64>();

        println!("sorted_a={sorted_a:?}");
        println!("sorted_b={sorted_b:?}");
        println!("combined={combined:?}");
        println!("exp_ranks_b={exp_ranks_b:?}");
        println!("exp_rank_sum_b={exp_rank_sum_b:?}");

        let mut hist_a = Histogram::new_with_max(300, 3).unwrap();
        let mut hist_b = Histogram::new_with_max(300, 3).unwrap();

        for i in 0..sample_a.len() {
            hist_a.record(sample_a[i]).unwrap();
            hist_b.record(sample_b[i]).unwrap();
        }

        let ranked_items = wilcoxon_ranked_items(&mut hist_a, &mut hist_b);
        println!("{ranked_items:?}");

        let rank_sum_b = ranked_items.iter().map(|y| y.rank).sum::<f64>();
        println!("rank_sum_b={rank_sum_b}");
        // assert_eq!(exp_rank_sum_b, rank_sum_b);

        let n_a = sample_a.len() as f64;
        let n_b = sample_b.len() as f64;
        let rank_sum_a = (1.0 + n_a + n_b) * (n_a + n_b) / 2.0 - rank_sum_b;
        println!("rank_sum_a={rank_sum_a}");

        let wilcoxon_rank_sum_a_lt_b_p = wilcoxon_rank_sum_a_lt_b_p(&mut hist_a, &mut hist_b);
        println!("wilcoxon_rank_sum_a_lt_b_p={wilcoxon_rank_sum_a_lt_b_p}");
        let wilcoxon_rank_sum_a_gt_b_p = wilcoxon_rank_sum_a_gt_b_p(&mut hist_a, &mut hist_b);
        println!("wilcoxon_rank_sum_a_gt_b_p={wilcoxon_rank_sum_a_gt_b_p}");
        let wilcoxon_rank_sum_a_ne_b_p: f64 = wilcoxon_rank_sum_a_ne_b_p(&mut hist_a, &mut hist_b);
        println!("wilcoxon_rank_sum_a_ne_b_p={wilcoxon_rank_sum_a_ne_b_p}");

        let mann_whitney_a_lt_b_u = mann_whitney_a_lt_b_u(&hist_a, &hist_b);
        println!("mann_whitney_a_lt_b_u={mann_whitney_a_lt_b_u}");
        let mann_whitney_a_gt_b_u = mann_whitney_a_gt_b_u(&hist_a, &hist_b);
        println!("mann_whitney_a_gt_b_u={mann_whitney_a_gt_b_u}");

        {
            let series_a = Series::new(
                "a".into(),
                sample_a.iter().map(|x| *x as f64).collect::<Vec<_>>(),
            );
            let series_b = Series::new(
                "b".into(),
                sample_b.iter().map(|x| *x as f64).collect::<Vec<_>>(),
            );

            let result = u_test(&series_a, &series_b, ALPHA, TailType::Two);
            println!("result={result:?}");

            let result = result.unwrap();

            println!("U Statistic: {}", result.test_statistic);
            println!("P-value: {}", result.p_value);
            println!("Reject Null: {}", result.reject_null);

            assert_eq!(
                result.test_statistic,
                mann_whitney_a_lt_b_u.min(mann_whitney_a_gt_b_u),
                "comparison of U statistics"
            );

            assert_eq!(
                round_to_sig_decimals(result.p_value, 5),
                round_to_sig_decimals(wilcoxon_rank_sum_a_ne_b_p, 5),
                "comparison of p values for non-equality"
            );
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
    fn test() {
        let sample_a0 = vec![85, 90, 78, 92, 88, 76, 95, 89, 91, 82];
        let sample_b0 = vec![70, 85, 80, 90, 75, 88, 92, 79, 86, 81];

        println!("***** Original samples *****");
        {
            let sample_a = sample_a0.clone();
            let sample_b = sample_b0.clone();
            process_samples(sample_a, sample_b);
        }

        println!();
        println!("***** Magnified samples *****");
        {
            let delta = 30;
            let nrepeats = 5;
            let sample_a = expand_sample(&sample_a0, delta, nrepeats);
            let sample_b = expand_sample(&sample_b0, delta, nrepeats);
            process_samples(sample_a, sample_b);
        }

        println!();
        println!("***** sample_a < sample_b *****");
        {
            let sample_a = sample_a0.clone();
            let delta = 2;
            let sample_b = sample_a.iter().map(|x| x + delta).collect::<Vec<_>>();
            process_samples(sample_a, sample_b);
        }
    }
}
