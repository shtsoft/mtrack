import React from "react";

type HeadProps = {
  description: string,
  title: string,
  canonical: string,
  css: () => React.JSX.Element,
  favicon: () => React.JSX.Element
}

function Head(props: HeadProps) {
  const CSS = props.css;
  const Favicon = props.favicon;
  return (
    <head>
      <meta httpEquiv="Content-Type" content="text/html; charset=UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <meta name="description" content={props.description} />

      <title>
        {props.title}
      </title>

      <CSS />
      <Favicon />
      <link rel="canonical" href={props.canonical} />
    </head>
  );
}

type BodyProps = {
  content: () => React.JSX.Element,
  js: () => React.JSX.Element
}

function Body(props: BodyProps) {
  const Content = props.content;
  const JS = props.js;
  return (
    <body>
      <Content />
      <JS />
    </body>
  );
}

type PageProps = {
  head: HeadProps,
  body: BodyProps
}

export function Page(props: PageProps) {
  return (
    <html lang="en">
      <Head
        description={props.head.description}
        title={props.head.title}
        canonical={props.head.canonical}
        css={props.head.css}
        favicon={props.head.favicon} />
      <Body content={props.body.content} js={props.body.js} />
    </html>
  );
}
