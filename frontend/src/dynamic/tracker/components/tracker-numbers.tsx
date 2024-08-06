/** 
 * This module defines a component tracking positions as numbers.
 */

import React from "react";

type TrackerNumbersParameters = {
  positions: any
}

/**
 * Renders a list of positions.
 * @param {any} `positions` - The positions to render.
 *
 * The positions should be a hash map from strings to pairs of numbers.
 */
export function TrackerNumbers({ positions }: TrackerNumbersParameters) {
  const list_items = [];

  for (const key in positions) {
    let lat = positions[key].latitude;
    let lng = positions[key].longitude;
    list_items.push(
      <>
        <dt key={key}>{key}</dt>
        <dd>{lat} (Latitude)</dd>
        <dd>{lng} (Longitude)</dd>
      </>
    )
  }

  return (
    <div id="numbers" data-testid="numbers" className="dark">
      <dl>
        {list_items}
      </dl>
    </div>
  )
}
