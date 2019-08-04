use chrono::prelude::*;
use uom::si::f64::*;
use uom::si::length::foot;

use crate::model::TidePrediction;
use crate::noaa_api::HighLowAndMetadata;

pub fn extract_predictions(m: &HighLowAndMetadata) -> Vec<TidePrediction> {
    m.values
        .values
        .iter()
        .flat_map(|item| {
            item.data.iter().map(move |data| TidePrediction {
                tide: Length::new::<foot>(data.pred.into()),
                time: parse_date_time(&item.date, &data.time),
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
}
