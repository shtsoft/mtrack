const path = require("path");

module.exports = {
  mode: "production",
  entry: {
    mtrack: "./src/dynamic/index.ts",
    map: "./src/dynamic/map/index.tsx",
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
