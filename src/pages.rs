use chrono::prelude::*;

use crate::compute;
use crate::model;

pub fn home_page(predictions: &[model::TidePrediction]) -> String {
    let time = now_in_pst();
    let pair = compute::find::nearest_pair(&predictions, time);
    let (headline, detail) = match pair {
        Some(p) => (p.headline(), p.detail()),
        _ => ("No Tide Information".into(), "".into())
    };
    format!(
        "<html>
            <head>
                <title>What Tide Is It Right Now?!</title>
                <link REL=stylesheet href='style.css' />
            </head>
            <body>
                <div class='container'>
                    <div class='content'>
                        <div class='title'>
                            <h1>What Tide Is It Right Now?!</h1>
                        </div>
                        <div class='headline'>
                            <h2>{}</h2>
                        </div>
                        <div class='detail'>
                            <p>{}</p>
                        </div>
                    </div>
                </div>
                <script src='getlocation.js'></script>
            </body>
        </html>",
        headline,
        detail,
    )
}

fn now_in_pst() -> DateTime<FixedOffset> {
    let pst = FixedOffset::west(8 * 3600);
    Local::now().with_timezone(&pst)
}

pub fn not_found_page() -> String {
    "<html><h1>404</h1><p>Not found!<p></html>".to_string()
}