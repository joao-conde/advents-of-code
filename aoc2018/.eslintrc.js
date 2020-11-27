module.exports = {
    extends: ["eslint:recommended", "google"],
    rules: {
        'no-console': 'off',
        'max-len': 'off',
        'no-tabs': 'off',
        'indent': [2, "tab"],
        'one-var': 'off',
        'guard-for-in': 'off'
    },
    env: {
        "node": true,
        "es6": true
    }
};
