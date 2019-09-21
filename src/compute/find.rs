use crate::model::{TidePrediction, TidePredictionPair};
use chrono::prelude::*;
use itertools::Itertools;
use uom::si::f64::*;
use uom::si::length::meter;

pub fn nearest_pair(
    tides: &[TidePrediction],
    time: DateTime<FixedOffset>,
) -> Option<TidePredictionPair> {
    let (before, after): (Vec<_>, Vec<_>) = tides
        .iter()
        .sorted_by_key(|x| x.time)
        .partition(|x| x.is_before(time));

    match (after.first().cloned(), before.last().cloned()) {
        (None, _) | (_, None) => None,
        (Some(next), Some(prev)) => Some(TidePredictionPair { next, prev }),
    }
}

/// Given two predictions and a time between them, use a sinusoidal
/// function to approximate the current tide level.
///
/// It's assumed that `last_prediction` comes before `next_prediction`
/// and that `last_prediction.time <= current_time <= next_prediction.time`.
/// If this assumption doesn't hold, the result probably won't be meaningful.
pub fn approximate_current_level(
    predictions: &TidePredictionPair,
    current_time: &DateTime<FixedOffset>,
) -> Length {
    let last_prediction = predictions.prev;
    let next_prediction = predictions.next;

    let f0: f64 = last_prediction.tide.get::<meter>();
    let f1: f64 = next_prediction.tide.get::<meter>();
    let t0 = last_prediction.time.timestamp() as f64;
    let t1 = next_prediction.time.timestamp() as f64;
    let t = current_time.timestamp() as f64;

    let phase = std::f64::consts::PI * (t - t0) / (t1 - t0);
    let l = 0.5 * (f0 - f1) * (1.0 + phase.cos()) + f1;
    Length::new::<meter>(l)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_finds_the_nearest_pair() {
        let pst = FixedOffset::west(8 * 3600);
        let time1 = pst.ymd(2019, 05, 14).and_hms(0, 0, 0);
        let time2 = pst.ymd(2019, 05, 14).and_hms(1, 0, 0);
        let time3 = pst.ymd(2019, 05, 18).and_hms(0, 0, 0);
        let time4 = pst.ymd(2019, 05, 18).and_hms(1, 0, 0);
        let tide1 = TidePrediction {
            tide: Length::new::<meter>(1.0),
            time: time1,
        };
        let tide2 = TidePrediction {
            tide: Length::new::<meter>(2.0),
            time: time2,
        };
        let tide3 = TidePrediction {
            tide: Length::new::<meter>(3.0),
            time: time3,
        };
        let tide4 = TidePrediction {
            tide: Length::new::<meter>(4.0),
            time: time4,
        };

        let tides = vec![tide1, tide2, tide3, tide4];

        let test_time = pst.ymd(2019, 05, 15).and_hms(0, 59, 0);

        let found = nearest_pair(&tides, test_time);
        assert_eq!(
            found,
            Some(TidePredictionPair {
                next: tide3,
                prev: tide2,
            })
        );
    }

    #[test]
    fn it_can_interpolate_times() {
        let pst = FixedOffset::west(8 * 3600);

        fn acceptably_close(l1: &Length, l2: &Length) {
            const ACCEPTABLE_ERROR: f64 = 0.001;
            let a = l1.get::<meter>();
            let b = l2.get::<meter>();
            let diff = (b - a).abs();
            assert!(diff < ACCEPTABLE_ERROR);
        }

        fn check_cases(p: &TidePredictionPair, cases: &Vec<(i64, f64)>) {
            use chrono::Duration;
            // Interpolation matches predictions at their times.
            acceptably_close(&approximate_current_level(p, &p.prev.time), &p.prev.tide);
            acceptably_close(&approximate_current_level(p, &p.next.time), &p.next.tide);
            for (mins, lvl) in cases {
                let t = p.prev.time + Duration::minutes(*mins);
                let l = Length::new::<meter>(*lvl);
                acceptably_close(&approximate_current_level(p, &t), &l);
            }
        }

        {
            let time1 = pst.ymd(2019, 05, 14).and_hms(0, 0, 0);
            let time2 = pst.ymd(2019, 05, 14).and_hms(1, 0, 0);
            let tide1 = TidePrediction {
                tide: Length::new::<meter>(1.0),
                time: time1,
            };
            let tide2 = TidePrediction {
                tide: Length::new::<meter>(2.0),
                time: time2,
            };
            let pair = TidePredictionPair {
                prev: tide1,
                next: tide2,
            };
            let cases = vec![
                (15, 1.1464466094067262),
                (30, 1.5),
                (45, 1.8535533905932737),
            ];
            check_cases(&pair, &cases);
        }
        {
            let time1 = pst.ymd(2015, 11, 30).and_hms(22, 0, 0);
            let time2 = pst.ymd(2015, 12, 1).and_hms(02, 0, 0);
            let tide1 = TidePrediction {
                tide: Length::new::<meter>(10.0),
                time: time1,
            };
            let tide2 = TidePrediction {
                tide: Length::new::<meter>(0.0),
                time: time2,
            };
            let pair = TidePredictionPair {
                prev: tide1,
                next: tide2,
            };
            let cases = vec![(60, 8.5355), (120, 5.0), (180, 1.4644999999999992)];
            check_cases(&pair, &cases);
        }
    }
}
