module.exports = {
    parserOptions: {
        ecmaVersion: 2020,
        sourceType: "module",
    },
    rules: {
        camelcase: ["error", { properties: "never" }],
    },
};
