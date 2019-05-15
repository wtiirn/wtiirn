import datetime
import pathlib
import typing as ty

import bs4


class TidePrediction(ty.NamedTuple):
    date: datetime.date
    time: datetime.time
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


def parse_row(rw, month: str, year: int) -> TidePrediction:
    day_src, time_src, tide_src = [c.get_text() for c in rw.find_all("td")]
    tide = float(tide_src)
    time = datetime.datetime.strptime(time_src, "%I:%M %p")
    day = int(day_src)
    return TidePrediction(
        date=datetime.date(year=year, month=MONTHS.index(month) + 1, day=day),
        time=time,
        tide=tide,
    )


def parse_table(tbl) -> ty.Tuple[str, ty.Iterable[TidePrediction]]:
    caption = tbl.caption.get_text()
    month, year = caption.split(" ")
    assert int(year) == 2019
    assert month in MONTHS

    body = tbl.tbody
    rows = body.find_all("tr")
    return month, [parse_row(rw, month, int(year)) for rw in rows]


def parse_tides(src: str):
    parsed = bs4.BeautifulSoup(src, "html.parser")
    tables = parsed.find_all("table")
    return {month: points for month, points in map(parse_table, tables)}


if __name__ == "__main__":
    import pprint

    src = pathlib.Path("point_atkinson_2019.html").open().read()
    pprint.pprint(parse_tides(src))
