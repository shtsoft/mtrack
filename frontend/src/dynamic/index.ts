const m: HTMLElement = document.querySelector("header > h1 > span");

function toggle_highlight_color() {
  if (m.getAttribute("class") === "highlight-color") {
    m.setAttribute("class", "");
  } else {
    m.setAttribute("class", "highlight-color");
  }
}

setInterval(toggle_highlight_color, 1000);
