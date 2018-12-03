module.exports = {
    extends: ["eslint:recommended", "google"],
    rules: {
        'no-console': 'off',
        'max-len': 'off',
        'no-tabs': 'off'    
    },
    env: {
        "node": true,
        "es6": true
    }
};
