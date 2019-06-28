import datetime
import json
import pathlib
import typing as ty

import bs4


class TidePrediction(ty.NamedTuple):
    date: datetime.datetime
    tide: float


MONTHS = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
]

TZINFO = datetime.timezone(offset=datetime.timedelta(hours=-8), name="PST")


def parse_row(rw, month_src: str, year: int) -> TidePrediction:
    day_src, time_src, tide_src = [c.get_text() for c in rw.find_all("td")]
    tide = float(tide_src)
    time = datetime.datetime.strptime(time_src, "%I:%M %p").time()
    day = int(day_src)
    month = MONTHS.index(month_src) + 1
    date = datetime.datetime(
        year=year,
        month=month,
        day=day,
        hour=time.hour,
        minute=time.minute,
        tzinfo=TZINFO,
    )
    return TidePrediction(date=date, tide=tide)


def parse_table(tbl) -> ty.Iterable[TidePrediction]:
    caption = tbl.caption.get_text()
    month, year = caption.split(" ")
    assert int(year) == 2019
    assert month in MONTHS

    body = tbl.tbody
    rows = body.find_all("tr")
    return [parse_row(rw, month, int(year)) for rw in rows]


def parse_tides(src: str) -> ty.Iterable[TidePrediction]:
    parsed = bs4.BeautifulSoup(src, "html.parser")
    tables = parsed.find_all("table")
    for month in map(parse_table, tables):
        yield from month


def format_prediction(p: TidePrediction) -> str:
    return json.dumps({"tide": p.tide, "time": p.date.isoformat()})


if __name__ == "__main__":
    src = pathlib.Path("point_atkinson_2019.html").open().read()
    predictions = list(parse_tides(src))
    with pathlib.Path("atkinson_predictions.json").open("w") as outf:
        outf.write("[\n")
        outf.write(",\n".join([format_prediction(p) for p in predictions]))
        outf.write("]")
