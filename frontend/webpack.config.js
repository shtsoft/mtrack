const path = require("path");

module.exports = {
  mode: "production",
  entry: {
    mtrack: "./src/dynamic/index.ts",
    postpos: "./src/dynamic/postpos/index.ts",
    tracker: "./src/dynamic/tracker/index.tsx",
  },
  devtool: "source-map",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: [".tsx", ".ts", ".jsx", ".js", ".json", ".wasm"]
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "dist/assets/app/js")
  }
};
