/**
 * The app allows to track positions as numbers as well as on a map.
 *
 * The idea is to fetch the positions from a remote server and have two subcomponents to visualize them as numbers and on a map, respectively.
 */

import { TrackerMap } from "./components/tracker-map";
import { TrackerNumbers } from "./components/tracker-numbers";

import React from "react";
import { useEffect, useState } from "react";

const GET_POSITION_INTERVAL = 1000;

/**
 * Combines the `TrackerNumbers`-component with the `TrackerMap`-component.
 *
 * As a side effect the function fetches positions from the server and makes them available to its subcomponents.
 */
export function App() {
  const [positions, setPositions] = useState({});

  useEffect(() => {
    let id: NodeJS.Timeout;

    const getPosition = () => {
      clearInterval(id);

      const response = {
        "foo": { "latitude": 0.0 + Math.random(), "longitude": 0.0 },
        "bar": { "latitude": 0.0 + Math.random(), "longitude": -1.0 }
      };
      setPositions(response);
    };

    id = setInterval(getPosition, GET_POSITION_INTERVAL);
  });

  return (
    <div className="app">
      <TrackerNumbers positions={positions} />
      <TrackerMap positions={positions} />
    </div>
  );
}
