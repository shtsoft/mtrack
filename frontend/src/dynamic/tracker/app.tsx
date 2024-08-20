/**
 * The tracker app allows to track positions as numbers as well as on a map.
 *
 * ## Design
 *
 * The idea is to fetch the positions from a server and have two subcomponents to visualize them as numbers and on a map, respectively.
 */

import { Logout } from "./components/logout";
import { TrackerMap } from "./components/tracker-map";
import { TrackerNumbers } from "./components/tracker-numbers";

import React from "react";
import { useEffect, useState } from "react";

const GET_POSITION_INTERVAL = 1000;

/**
 * Combines the `TrackerNumbers`-component with the `TrackerMap`-component and the  `Logout`-component.
 *
 * As a side effect the function fetches positions from the server and makes them available to its tracker-subcomponents.
 */
export function App() {
  const [positions, setPositions] = useState({});

  useEffect(() => {
    let id: NodeJS.Timeout;

    const getPosition = () => {
      clearInterval(id);

      const request = new Request(`/positions`, { method: "GET" });
      fetch(request)
        .then((response) => response.json())
        .then((data) => {
          setPositions(data);
        })
        .catch(console.error)
    };

    id = setInterval(getPosition, GET_POSITION_INTERVAL);
  });

  return (
    <div className="app">
      <TrackerNumbers positions={positions} />
      <TrackerMap positions={positions} />
      <Logout />
    </div>
  );
}
