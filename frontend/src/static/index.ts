import { Home } from "./pages/home";
import { Login } from "./pages/login";
import { Map } from "./pages/map";

import { writeFileSync } from "fs";

import * as prettier from "prettier";

import { renderToStaticMarkup } from "react-dom/server";

const outdir = "./public";

const pages = [
  { route: "", element: Home },
  { route: "/login", element: Login },
  { route: "/map", element: Map }];

for (const page of pages) {
  prettier
    .format(
       "<!DOCTYPE html />" + renderToStaticMarkup(page.element()), 
      { semi: false, parser: "html" })
    .then(html => {
      try {
        writeFileSync(outdir + page.route + "/index.html", html)
      } catch (err) {
        console.error(err)
      }
    })
}
