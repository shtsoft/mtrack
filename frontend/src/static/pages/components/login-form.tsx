import React from "react";

export function LoginForm() {
  return (
    <form action="/login" method="post" className="login dark">
      <ul className="nobullets">
        <li>
          <label htmlFor="name">Name</label>
          <input type="text" id="name" name="name" required></input>
        </li>
        <li>
          <label htmlFor="password">Password</label>
          <input type="password" id="password" name="password" required></input>
        </li>
        <li>
          <button type="submit" className="highlight">Log in</button>
        </li>
      </ul>
    </form>
  );
}
