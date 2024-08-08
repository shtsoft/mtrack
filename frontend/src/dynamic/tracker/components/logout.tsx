/**
 * This module defines a component for logging out.
 */

import React from "react";

/**
 * Renders a logout form.
 */
export function Logout() {
  return (
    <form action="/logout" method="post" className="dark">
      <ul className="nobullets">
        <li>
          <button type="submit" className="highlight">Log out</button>
        </li>
      </ul>
    </form>
  )
}
