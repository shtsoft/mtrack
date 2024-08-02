import { Favicon } from "./components/favicon";
import { Nav } from "./components/nav";
import { Page } from "./_page";

import React from "react";

function CSS() {
  return (
    <link rel="stylesheet" href="/assets/app/css/mtrack.css" />
  );
}

function Content() {
  return (
    <>
      <div className="vh-10 dark">
        <Nav />
      </div>
      <div id="layout" className="vh-90">
        <header className="vh-40 dark">
          <h1>
            <span className="highlight-color">m</span>track
          </h1>
        </header>
        <div id="content">
          <div className="skewleft dark">
          </div>
          <div className="skewright dark">
          </div>
          <div className="vh-50 map">
            <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>
          </div>
        </div>
      </div>
    </>
  );
}

function JS() {
  return (
    <script src="/assets/app/js/mtrack.js"></script> 
  );
}

export function Home() {
  const hprops = {
    description: "",
    title: "mtrack - Home",
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
