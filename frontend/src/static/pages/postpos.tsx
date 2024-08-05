import { Favicon } from "./components/favicon";
import { PostposForm } from "./components/postpos-form";
import { Page } from "./_page";

import React from "react";

function CSS() {
  return (
    <>
      <link rel="stylesheet" href="/assets/app/css/mtrack.css" />
    </>
  );
}

function Content() {
  return (
    <div id="layout">
      <img className="logoblock" src="/assets/app/images/mtrackpath.svg" alt="mtrackpath" />
      <PostposForm />
    </div>
  );
}

function JS() {
  return (
    <script src="/assets/app/js/postpos.js"></script>
  );
}

export function Postpos() {
  const hprops = {
    description: "",
    title: "mtrack - Postpos",
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
