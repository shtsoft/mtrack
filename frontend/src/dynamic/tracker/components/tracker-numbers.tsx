/** 
 * Describe the module ...
 */

import React from "react";

function private_placeholder_function(): number {
  return 0
}

type TrackerNumbersParameters = {
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
export function TrackerNumbers({ positions }: TrackerNumbersParameters) {
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
