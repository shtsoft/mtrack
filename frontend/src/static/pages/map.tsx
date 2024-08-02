import { Favicon } from "./components/favicon";
import { Page } from "./_page";

import React from "react";

function CSS() {
  return (
    <>
      <link rel="stylesheet" href="/assets/vendor/css/leaflet.css" />
      <link rel="stylesheet" href="/assets/app/css/map.css" />
      <link rel="stylesheet" href="/assets/app/css/mtrack.css" />
    </>
  );
}

function Content() {
  return (
    <>
      <noscript>
        This is a React app.
        Enable JavaScript to make it work.
      </noscript>
      <div id="root"></div>
    </>
  );
}

function JS() {
  return (
    <script src="/assets/app/js/map.js"></script> 
  );
}

export function Map() {
  const hprops = {
    description: "",
    title: "mtrack - Map",
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
