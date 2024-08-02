/**
 * Describe what the app is about ...
 *
 * Describe the design of the app ...
 */

import L from "leaflet";

import React from "react";
import { useEffect, useState } from "react";

type TrackerNumbersParameters = {
  positions: any
}

function TrackerNumbers({ positions }: TrackerNumbersParameters) {
  const list_items = [];

  for (const key in positions) {
    let lat = positions[key].latitude;
    let lng = positions[key].longitude;
    list_items.push(<li>{key}:{lat}:{lng}</li>)
  }

  return (
    <div id="numbers">
      <ul>
        {list_items}
      </ul>
    </div>
  )
}

let map: L.Map;
let icon: L.Icon;
let markers: Map<String, L.Marker> = new Map();

type TrackerMapParameters = {
  positions: any
}

function TrackerMap({ positions }: TrackerMapParameters) {
  useEffect(() => {
    map = L.map('map').setView([0.0, 0.0], 2);

    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 19,
      attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);

    icon = L.icon({
      iconUrl: '/assets/vendor/images/marker-icon.png',
      iconSize: [50, 50],
    });
  }, []);

  useEffect(() => {
    for (const key of markers.keys()) {
      markers.get(key).removeFrom(map);
    }

    for (const key in positions) {
      let lat = positions[key].latitude;
      let lng = positions[key].longitude;
      markers.set(key, L.marker([lat, lng], { icon: icon, title: key }));
    }

    for (const key of markers.keys()) {
      markers.get(key).addTo(map);
    }
  }, [positions]);

  return (
    <div id="map">
    </div>
  )
}

export function App() {
  const [positions, setPositions] = useState({});

  useEffect(() => {
    let id: NodeJS.Timeout;

    function getPosition(): void {
      clearInterval(id);

      const response = {
        "foo": { "latitude": 0.0 + Math.random(), "longitude": 0.0 },
        "bar": { "latitude": 0.0 + Math.random(), "longitude": -1.0 }
      };
      setPositions(response);
    };

    id = setInterval(getPosition, 1000);
  });

  return (
    <div className="app">
      <TrackerNumbers positions={positions} />
      <TrackerMap positions={positions} />
    </div>
  );
}
