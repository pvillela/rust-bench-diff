use hdrhistogram::{
    iterators::{recorded::Iter, HistogramIterator, IterationValue},
    Histogram,
};
use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Debug)]
pub enum PositionInCi {
    Below,
    In,
    Above,
}

impl PositionInCi {
    pub fn position_of_value(value: f64, low: f64, high: f64) -> Self {
        match value {
            _ if value <= low => PositionInCi::Below,
            _ if low < value && value < high => PositionInCi::In,
            _ => PositionInCi::Above,
        }
    }
}

#[inline(always)]
pub fn sample_mean(n: f64, sum: f64) -> f64 {
    sum / n
}

#[inline(always)]
pub fn sample_sum2_deviations(n: f64, sum: f64, sum2: f64) -> f64 {
    sum2 - sum.powi(2) / n
}

#[inline(always)]
pub fn sample_stdev(n: f64, sum: f64, sum2: f64) -> f64 {
    (sample_sum2_deviations(n, sum, sum2) / (n - 1.0)).sqrt()
}

pub fn welch_t(n1: u64, n2: u64, mean1: f64, mean2: f64, stdev1: f64, stdev2: f64) -> f64 {
    todo!()
}

pub fn welch_deg_freedom(n1: u64, n2: u64, stdev1: f64, stdev2: f64) -> f64 {
    todo!()
}

pub fn welch_ci(
    n1: u64,
    n2: u64,
    mean1: f64,
    mean2: f64,
    stdev1: f64,
    stdev2: f64,
    alpha: f64,
) -> (f64, f64) {
    todo!()
}

#[derive(Debug)]
struct RankedItem {
    #[allow(unused)] // for debugging only
    #[cfg(test)]
    value: u64,
    count: u64,
    rank: f64,
}

fn wilcoxon_ranked_items(hist1: &Histogram<u64>, hist2: &Histogram<u64>) -> Vec<RankedItem> {
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

    let ranked_items2: Vec<RankedItem> = {
        let mut items2 = Vec::<RankedItem>::with_capacity(hist2.distinct_values());
        let mut iter1 = hist1.iter_recorded();
        let mut iter2 = hist2.iter_recorded();
        let (mut item1_opt, mut item2_opt) = (iter1.next(), iter2.next());
        let mut prev_rank = 0.0;

        loop {
            match (&mut item1_opt, &mut item2_opt) {
                (Some(item1), None) => {
                    let (_, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item1.value_iterated_to(),
                        item1.count_at_value(),
                        0,
                        &mut iter1,
                        &mut item1_opt,
                        prev_rank,
                    );
                    prev_rank = new_prev_rank;
                }

                (Some(item1), Some(item2))
                    if item1.value_iterated_to() < item2.value_iterated_to() =>
                {
                    let (_, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item1.value_iterated_to(),
                        item1.count_at_value(),
                        0,
                        &mut iter1,
                        &mut item1_opt,
                        prev_rank,
                    );
                    prev_rank = new_prev_rank;
                }

                (None, Some(item2)) => {
                    let (ranked_item, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item2.value_iterated_to(),
                        item2.count_at_value(),
                        0,
                        &mut iter2,
                        &mut item2_opt,
                        prev_rank,
                    );
                    items2.push(ranked_item);
                    prev_rank = new_prev_rank;
                }

                (Some(item1), Some(item2))
                    if item1.value_iterated_to() > item2.value_iterated_to() =>
                {
                    let (ranked_item, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item2.value_iterated_to(),
                        item2.count_at_value(),
                        0,
                        &mut iter2,
                        &mut item2_opt,
                        prev_rank,
                    );
                    items2.push(ranked_item);
                    prev_rank = new_prev_rank;
                }

                // if item1.value_iterated_to() == item2.value_iterated_to()
                (Some(item1), Some(item2)) => {
                    let count_1 = item1.count_at_value();
                    let count_2 = item2.count_at_value();
                    #[cfg(test)]
                    rank_item(
                        item1.value_iterated_to(),
                        count_1,
                        count_2,
                        &mut iter1,
                        &mut item1_opt,
                        prev_rank,
                    );
                    let (ranked_item, new_prev_rank) = rank_item(
                        #[cfg(test)]
                        item2.value_iterated_to(),
                        count_2,
                        count_1,
                        &mut iter2,
                        &mut item2_opt,
                        prev_rank,
                    );
                    items2.push(ranked_item);
                    prev_rank = new_prev_rank;
                }

                (None, None) => break,
            }
        }

        items2
    };

    ranked_items2
}

pub fn wilcoxon_rank_sum_z(hist1: &Histogram<u64>, hist2: &Histogram<u64>) -> f64 {
    let ranked_items2 = wilcoxon_ranked_items(hist1, hist2);
    let n1 = hist1.len() as f64;
    let n2 = hist2.len() as f64;

    let w: f64 = ranked_items2.iter().map(|y| y.count as f64 * y.rank).sum();
    let e0_w = n2 * (n1 + n2 + 1.0) / 2.0;
    let var0_w = n1 * n2 * (n1 + n2 + 1.0) / 12.0;
    let w_star = (w - e0_w) / var0_w.sqrt();

    w_star
}

pub fn wilcoxon_rank_sum_p(hist1: &Histogram<u64>, hist2: &Histogram<u64>) -> f64 {
    let z = wilcoxon_rank_sum_z(hist1, hist2);
    let normal = Normal::standard();
    normal.cdf(z)
}

#[cfg(test)]
mod test {
    use crate::wilcoxon_rank_sum_z;

    use super::wilcoxon_ranked_items;
    use hdrhistogram::Histogram;

    #[test]
    fn test() {
        let sample_a = [85, 90, 78, 92, 88, 76, 95, 89, 91, 82];
        let sample_b = [70, 85, 80, 90, 75, 88, 92, 79, 86, 81];

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

        let mut hist_a = Histogram::new_with_max(100, 2).unwrap();
        let mut hist_b = Histogram::new_with_max(100, 2).unwrap();

        for i in 0..sample_a.len() {
            hist_a.record(sample_a[i]).unwrap();
            hist_b.record(sample_b[i]).unwrap();
        }

        let ranked_items = wilcoxon_ranked_items(&mut hist_a, &mut hist_b);
        println!("{ranked_items:?}");

        let rank_sum_b = ranked_items.iter().map(|y| y.rank).sum::<f64>();
        println!("rank_sum_b={rank_sum_b}");
        assert_eq!(exp_rank_sum_b, rank_sum_b);

        let n1 = sample_a.len() as f64;
        let n2 = sample_b.len() as f64;
        let rank_sum_a = (1.0 + n1 + n2) * (n1 + n2) / 2.0 - rank_sum_b;
        println!("rank_sum_a={rank_sum_a}");

        let z = wilcoxon_rank_sum_z(&mut hist_a, &mut hist_b);
        println!("z={z}");
    }
}
