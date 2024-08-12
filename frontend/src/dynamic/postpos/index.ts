function postPosition(
  geostatus: HTMLParagraphElement,
  key: String,
  position: any
): void {
  const request = new Request(
    `/positions/${key}`,
    { method: "POST", body: `${JSON.stringify(position)}` }
  );
  fetch(request)
    .then((response) => {
      if (response.ok) {
        geostatus.textContent = "Posting positions ...";
      } else {
        response.text().then((responseText) => {
          geostatus.textContent = `Error posting positions: ${responseText}`;
        })
      }
    })
    .catch((error) => {
      console.error(`Could not make request: ${error}`);
    })
}

function geoSuccess(
  geostatus: HTMLParagraphElement,
  key: String,
  position: GeolocationPosition
) {
  const latitude = position.coords.latitude;
  const longitude = position.coords.longitude;

  const positionObject = { "latitude": latitude, "longitude": longitude };

  postPosition(geostatus, key, positionObject);
}

function geoError(geostatus: HTMLParagraphElement): void {
  geostatus.textContent = "Unable to retrieve your location";
}

const layout = document.querySelector("#layout");
const keyForm = document.querySelector("form");
const keyInput: any = document.querySelector("#key");

const geoStatus = document.createElement("p");
geoStatus.style.textAlign = "center";
geoStatus.textContent = "";
layout.appendChild(geoStatus);

keyForm.addEventListener("submit", (event) => {
  event.preventDefault();

  keyForm.remove();

  if (!navigator.geolocation) {
    geoStatus.textContent = "Geolocation is not supported by your browser";
  } else {
    geoStatus.textContent = "Locating ...";
    navigator.geolocation.watchPosition(
      (position) => geoSuccess(geoStatus, keyInput.value, position),
      () => geoError(geoStatus)
    );
  }
});
