/** 
 * This module defines a component trackings positions on a map.
 */

import L from "leaflet";

import React from "react";
import { useEffect } from "react";

const INITIAL_MAP_ZOOM = 2;
const INITIAL_MAP_POSITION: L.LatLngExpression = [0.0, 0.0];

let map: L.Map;
let icon: L.Icon;
let markers: Map<String, L.Marker> = new Map();

function removeMarkersFromMap(map: L.Map, markers: Map<String, L.Marker>): void {
  for (const key of markers.keys()) {
    markers.get(key).removeFrom(map);
  }
}

function setMarkersToPositions(positions: any, markers: Map<String, L.Marker>): void {
  for (const key in positions) {
    let lat = positions[key].latitude;
    let lng = positions[key].longitude;
    markers.set(key, L.marker([lat, lng], { icon: icon, title: key }));
  }
}

function addMarkersToMap(map: L.Map, markers: Map<String, L.Marker>): void {
  for (const key of markers.keys()) {
    markers.get(key).addTo(map);
  }
}

type TrackerMapParameters = {
  positions: any
}

/**
 * Renders a map and marks positions on it.
 * @param {any} `positions` - The positions marked on the map.
 *
 * The positions should be a hash map from strings to pairs of numbers.
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
      iconAnchor: [25, 49]
    });
  }, []);

  useEffect(() => {
    removeMarkersFromMap(map, markers);
    setMarkersToPositions(positions, markers)
    addMarkersToMap(map, markers);
  }, [positions]);

  return (
    <div id="map" className="vh-100">
    </div>
  )
}
