import datetime
import json
import pathlib
import time
import typing as ty

import bs4
import pytz


class TidePrediction(ty.NamedTuple):
    date: datetime.datetime
    tide: float


def parse_item(item) -> ty.Iterable[TidePrediction]:
    date_src = item["date"]
    date = datetime.datetime.strptime(date_src, "%m/%d/%Y").date()
    for data in item.find_all("data"):
        time_src = data.time.contents[0]
        pred_src = data.pred.contents[0]
        predtime = datetime.datetime.strptime(time_src, "%H:%M").time()
        tide = float(pred_src)
        yield TidePrediction(
            date=datetime.datetime.combine(date, predtime, pytz.UTC), tide=tide
        )


def parse_tides(src) -> ty.Iterable[TidePrediction]:
    parsed = bs4.BeautifulSoup(src, "lxml")
    items = parsed.find_all("item")
    for item in items:
        yield from parse_item(item)


def format_prediction(p: TidePrediction) -> str:
    return json.dumps({"tide": p.tide, "time": p.date.isoformat()})


if __name__ == "__main__":
    src = pathlib.Path("port_lavaca.xml").open().read()
    predictions = list(parse_tides(src))
    with pathlib.Path("lavaca_predictions.json").open("w") as outf:
        outf.write("[\n")
        outf.write(",\n".join([format_prediction(p) for p in predictions]))
        outf.write("]")
