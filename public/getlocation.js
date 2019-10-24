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

function showMap(lat, lon) {
  var map = L.map('map', {
      center: [lat, lon],
      zoom: 13
  });
  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: "&copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a> contributors"
  }).addTo(map);
}
