use chrono::prelude::*;
use crate::model::{TidePrediction, TidePredictionPair};

pub fn find_nearest_prediction(
    tides: &[TidePrediction],
    time: DateTime<FixedOffset>,
) -> Option<TidePrediction> {
    let mut deltas: Vec<_> = tides.into();
    deltas.sort_by_key(|x| (x.time.timestamp() - time.timestamp()).abs());
    deltas.first().cloned()
}

pub fn find_nearest_pair(
    tides: &[TidePrediction],
    time: DateTime<FixedOffset>,
) -> TidePredictionPair {
    let mut deltas: Vec<_> = tides.into();
    deltas.sort_by_key(|x| x.time.timestamp() - time.timestamp());
    let (after, before): (Vec<_>, Vec<_>) = deltas
        .into_iter()
        .partition(|x| x.time.timestamp() - time.timestamp() >= 0);

    TidePredictionPair {
        next: after.first().cloned(),
        prev: before.last().cloned(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_a_matching_prediction() {
        let time = FixedOffset::west(8 * 3600)
            .ymd(2019, 05, 14)
            .and_hms(0, 0, 0);

        let tide = TidePrediction { tide: 0.5, time };

        let tides = vec![tide];

        let found = find_nearest_prediction(&tides, time);
        assert_eq!(found, Some(tide));
    }

    #[test]
    fn it_returns_the_nearest_prediction() {
        let time1 = FixedOffset::west(8 * 3600)
            .ymd(2019, 05, 14)
            .and_hms(0, 0, 0);

        let time2 = FixedOffset::west(8 * 3600)
            .ymd(2019, 05, 14)
            .and_hms(1, 0, 0);

        let time3 = FixedOffset::west(8 * 3600)
            .ymd(2019, 05, 18)
            .and_hms(0, 0, 0);

        let tide1 = TidePrediction {
            tide: 1.0,
            time: time1,
        };
        let tide2 = TidePrediction {
            tide: 2.0,
            time: time2,
        };
        let tide3 = TidePrediction {
            tide: 3.0,
            time: time3,
        };

        let tides = vec![tide1, tide2, tide3];

        let test_time = FixedOffset::west(8 * 3600)
            .ymd(2019, 05, 14)
            .and_hms(0, 59, 0);

        let found = find_nearest_prediction(&tides, test_time);
        assert_eq!(found, Some(tide2));
    }

    #[test]
    fn it_finds_the_nearest_pair() {
        let pst = FixedOffset::west(8 * 3600);
        let time1 = pst.ymd(2019, 05, 14).and_hms(0, 0, 0);

        let time2 = pst.ymd(2019, 05, 14).and_hms(1, 0, 0);

        let time3 = pst.ymd(2019, 05, 18).and_hms(0, 0, 0);

        let time4 = pst.ymd(2019, 05, 18).and_hms(1, 0, 0);

        let tide1 = TidePrediction {
            tide: 1.0,
            time: time1,
        };
        let tide2 = TidePrediction {
            tide: 2.0,
            time: time2,
        };
        let tide3 = TidePrediction {
            tide: 3.0,
            time: time3,
        };
        let tide4 = TidePrediction {
            tide: 4.0,
            time: time4,
        };

        let tides = vec![tide1, tide2, tide3, tide4];

        let test_time = pst.ymd(2019, 05, 15).and_hms(0, 59, 0);

        let found = find_nearest_pair(&tides, test_time);
        assert_eq!(
            found,
            TidePredictionPair {
                next: Some(tide3),
                prev: Some(tide2)
            }
        );
    }
}
