/**
 * The tracker app enables users to track positions via numerical data and on a map.
 *
 * ## Design
 *
 * This application fetches location data from a server and utilizes two subcomponents to visualize the positions: one for numerical display and another for map-based tracking.
 */

import { Logout } from "./components/logout";
import { TrackerMap } from "./components/tracker-map";
import { TrackerNumbers } from "./components/tracker-numbers";

import React from "react";
import { useEffect, useState } from "react";

const GET_POSITION_INTERVAL = 1000;

/**
 * Orchestrates the `TrackerNumbers`, `TrackerMap`, and `Logout` components.
 *
 * This component handles the side effect of fetching positions from the server and provides the data to its subcomponents.
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
