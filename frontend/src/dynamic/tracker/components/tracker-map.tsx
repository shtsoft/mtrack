/** 
 * Describe the module ...
 */

import L from "leaflet";

import React from "react";
import { useEffect, useState } from "react";


function private_placeholder_function(): number {
  return 0
}

const INITIAL_MAP_ZOOM = 2;
const INITIAL_MAP_POSITION: L.LatLngExpression = [0.0, 0.0];

let map: L.Map;
let icon: L.Icon;
let markers: Map<String, L.Marker> = new Map();

type TrackerMapParameters = {
  positions: any
}

/**
 * Describe what the function does ...
 * @param {any} `arg1` - Describe the meaning ...
 * @param {any} `arg2` - Describe the meaning ...
 *
 * Describe the side effects of the function ...
 *
 * Describe the preconditions, postconditions and invariants ...
 *
 * Provide additional information interesting to callers ...
 */
export function TrackerMap({ positions }: TrackerMapParameters) {
  useEffect(() => {
    map = L.map('map').setView(INITIAL_MAP_POSITION, INITIAL_MAP_ZOOM);

    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 19,
      attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);

    icon = L.icon({
      iconUrl: '/assets/app/images/marker-icon.svg',
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

