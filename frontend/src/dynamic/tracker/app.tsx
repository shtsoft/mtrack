/**
 * Describe what the app is about ...
 *
 * Describe the design of the app ...
 */

import { TrackerMap } from "./components/tracker-map";
import { TrackerNumbers } from "./components/tracker-numbers";

import React from "react";
import { useEffect, useState } from "react";

const GET_POSITION_INTERVAL = 1000;

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
