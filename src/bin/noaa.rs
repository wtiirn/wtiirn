use serde::Deserialize;
use serde_xml_rs;

use wtiirn::noaa_api;

// Taken from the examples section of the NOAA website
// https://opendap.co-ops.nos.noaa.gov/axis/webservices/highlowtidepred/samples/response.xml
const RESPONSE: &str = include_str!("../../scraping/noaa_response.xml");

#[derive(Debug, Deserialize)]
struct Envelope {
    #[serde(rename = "Body")]
    body: Body,
}

#[derive(Debug, Deserialize)]
struct Body {
    #[serde(rename = "HighLowAndMetadata")]
    predictions: noaa_api::HighLowAndMetadata,
}

fn main() {
    let s: Envelope = serde_xml_rs::from_str(RESPONSE).unwrap();
    dbg!(&s);
    let extracted = noaa_api::transform::extract_predictions(&s.body.predictions);
    dbg!(&extracted);
}
