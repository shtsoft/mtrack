import { Favicon } from "./components/favicon";
import { Footer } from "./components/footer";
import { Header } from "./components/header";
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
      <Nav />
      <Header />
      <div id="content">
      </div>
      <hr />
      <Footer />
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
