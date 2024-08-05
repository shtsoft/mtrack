import React from "react";

export function PostposForm() {
  return (
    <form action="/postpos" method="post" className="dark">
      <ul className="nobullets">
        <li>
          <label htmlFor="key">Key</label>
          <input type="password" id="key" name="key" required></input>
        </li>
        <li><button type="submit" className="highlight">Start tracking</button></li>
      </ul>
    </form>
  );
}
