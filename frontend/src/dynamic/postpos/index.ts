function postPosition(
  geostatus: HTMLParagraphElement,
  key: String,
  serverURL: URL,
  position: any
): void {
  const requestURL = new URL(`${serverURL}positions/${key}`);
  const request = new Request(requestURL, { method: "POST", body: `${JSON.stringify(position)}` });
  fetch(request)
    .then((response) => {
      if (response.ok) {
        geostatus.textContent = "Posting positions ...";
      } else {
        geostatus.textContent = "Error posting positions";
      }
    })
    .catch((error) => {
      console.error(`Could not make request: ${error}`);
    })
}

function geoSuccess(
  geostatus: HTMLParagraphElement,
  key: String,
  serverURL: URL,
  position: GeolocationPosition
) {
  const latitude = position.coords.latitude;
  const longitude = position.coords.longitude;

  const positionObject = { "latitude": latitude, "longitude": longitude };

  postPosition(geostatus, key, serverURL, positionObject);
}

function geoError(status: HTMLParagraphElement): void {
  status.textContent = "Unable to retrieve your location";
}

const serverURL: URL = new URL("https://127.0.0.1:10443");

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
      (position) => geoSuccess(geoStatus, keyInput.value, serverURL, position),
      () => geoError(geoStatus)
    );
  }
});
