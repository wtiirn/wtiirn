use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct TidePrediction {
    pub tide: f32,
    pub time: DateTime<FixedOffset>,
}

pub fn find_nearest_prediction(
    tides: &[TidePrediction],
    time: DateTime<FixedOffset>,
) -> Option<TidePrediction> {
    let mut deltas: Vec<_> = tides.into();
    deltas.sort_by_key(|x| (x.time.timestamp() - time.timestamp()).abs());
    deltas.first().cloned()
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
}
