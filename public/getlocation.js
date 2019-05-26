function reloadWithLocation(pos) {
  let coords = pos.coords;
  let latlon =
    "lat=" + coords.latitude.toString() + "&lon=" + coords.longitude.toString();
  let newUrl = "/?" + latlon;
  window.location.replace(newUrl);
}

function getLocationAndReload() {
  if ("geolocation" in navigator) {
    let geo = navigator.geolocation;
    geo.getCurrentPosition(reloadWithLocation, console.error);
  }
}

window.onload = function() {
  if (!window.location.search) {
    getLocationAndReload();
  }
};
