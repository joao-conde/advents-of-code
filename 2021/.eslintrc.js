module.exports = {
    root: true,
    parser: "@typescript-eslint/parser",
    plugins: ["@typescript-eslint"],
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended",
        "hive"
    ],
    rules: {
        "no-unused-vars": ["off"]
    },
    ignorePatterns: ["node_modules/", "dist/"]
};
