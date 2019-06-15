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
}
