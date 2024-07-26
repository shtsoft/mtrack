module.exports = {
  moduleFileExtensions: ["tsx", "ts", "jsx", "js", "json"],
  modulePaths: ["<rootDir>"],
  transform: {
    "^.+\\.(ts|tsx)$": [
      "ts-jest",
      {
        tsconfig: "tsconfig.json",
      }
    ]
  },
  testMatch: ["**/tests/**/*.test.tsx"]
};
