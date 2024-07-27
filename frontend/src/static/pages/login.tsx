import { Favicon } from "./components/favicon";
import { Page } from "./_page";

import React from "react";

function CSS() {
  return (
    <>
      <link rel="stylesheet" href="/assets/app/css/login.css" />
      <link rel="stylesheet" href="/assets/app/css/mtrack.css" />
    </>
  );
}

function Content() {
  return (
    <div id="layout">
      <div className="vh-50 dark">
      </div>
      <div className="login-form">
      </div>
      <div className="vh-50 map">
        <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>
      </div>
    </div>
  );
}

function JS() {
  return (
    <>
    </>
  );
}

export function Login() {
  const hprops = {
    description: "",
    title: "mtrack - Login",
    canonical: "",
    css: CSS,
    favicon: Favicon
  };
  const bprops = {
    content: Content,
    js: JS
  };
  return (
    <Page head={hprops} body={bprops} />
  );
}
