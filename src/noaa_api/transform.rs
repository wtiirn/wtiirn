use chrono::prelude::*;
use uom::si::f64::*;
use uom::si::length::foot;

use crate::model::TidePrediction;
use crate::noaa_api::HighLowAndMetadata;
use crate::stations::{PredictionWithId, Station};

pub fn extract_predictions(m: &HighLowAndMetadata) -> Vec<PredictionWithId> {
    let station_id = Station::generate_id(&m.station_name, m.station_id);
    m.values
        .values
        .iter()
        .flat_map(|item| {
            item.data.iter().map(move |data| PredictionWithId {
                station_id,
                prediction: TidePrediction {
                    tide: Length::new::<foot>(data.pred.into()),
                    time: parse_date_time(&item.date, &data.time),
                },
            })
        })
        .collect()
}

fn parse_date_time(date: &str, time: &str) -> DateTime<FixedOffset> {
    let time_str = format!("{}/{}:00+0000", date, time);

    DateTime::parse_from_str(&time_str, "%m/%d/%Y/%H:%M:%S%z").expect("couldn't parse date")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_should_parse_the_date() {
        let time = parse_date_time("01/27/2009", "01:23");
        let utc = FixedOffset::west(0);
        assert_eq!(utc.ymd(2009, 01, 27).and_hms(01, 23, 0), time);
    }

    #[test]
    fn it_should_extract_the_predictions() {
        use crate::noaa_api::{HighLowValues, Item, TideData};
        let m = HighLowAndMetadata {
            station_id: 1000,
            station_name: "fake station".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            time_zone: "UTC".to_string(),
            unit_name: "foot".to_string(),
            values: HighLowValues {
                values: vec![Item {
                    date: "01/01/2019".to_string(),
                    data: vec![TideData {
                        time: "12:00".to_string(),
                        pred: 1.0,
                    }],
                }],
            },
        };

        let extracted = extract_predictions(&m);

        assert_eq!(extracted.len(), 1);
        assert_eq!(extracted[0].prediction.tide, Length::new::<foot>(1.0));
        let utc = FixedOffset::west(0);
        assert_eq!(
            extracted[0].prediction.time,
            utc.ymd(2019, 01, 01).and_hms(12, 00, 00)
        );
    }
}
