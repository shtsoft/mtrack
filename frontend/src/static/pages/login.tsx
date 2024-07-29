import { Favicon } from "./components/favicon";
import { LoginForm } from "./components/login-form";
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
      <LoginForm />
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
