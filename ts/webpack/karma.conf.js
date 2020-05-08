const path = require('path');
const webpackConfig = require('./webpack.config.js');

module.exports = (config) => {
    config.set({
        frameworks: ['mocha', 'chai'],

        files: [
            { pattern: 'test/**/*_test.ts', watched: false },
        ],

        preprocessors: {
            'test/**/*_test.ts': [ 'webpack'],
        },

        browsers: [],

        webpack: webpackConfig,

        webpackMiddleware: {
            stats: 'errors-only',
        },
    });
};