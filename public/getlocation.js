function reloadWithLocationAndOffset(pos) {
  let coords = pos.coords;
  let latlon =
    "lat=" + coords.latitude.toString() + "&lon=" + coords.longitude.toString();
  let newUrl = "/?" + latlon + "&offset=" + getLocalTimezoneOffset();
  window.location.replace(newUrl);
}

function reloadWithOffset() {
  let offset = getLocalTimezoneOffset();
  window.location.replace("/?offset=" + offset);
}

function getLocationAndReload() {
  if ("geolocation" in navigator) {
    let geo = navigator.geolocation;
    geo.getCurrentPosition(reloadWithLocationAndOffset, reloadWithOffset);
  } else {
    reloadWithOffset();
  }
}

function getLocalTimezoneOffset() {
  let date = new Date();
  return date.getTimezoneOffset();
}

window.onload = function() {
  if (!window.location.search) {
    getLocationAndReload();
  }
};
