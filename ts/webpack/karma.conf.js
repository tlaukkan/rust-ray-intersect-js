const path = require('path');

module.exports = (config) => {
    config.set({
        frameworks: ['mocha', 'chai'],

        files: [
            { pattern: 'test/**/*_test.ts', watched: false },
        ],

        preprocessors: {
            'test/**/*_test.ts': ['webpack'],
        },

        browsers: ['ChromeHeadless', 'Firefox'],

        webpack: {
            mode: 'development',
            entry: './src/index.ts',
            devtool: 'inline-source-map',
            module: {
                rules: [
                    {
                        test: /\.tsx?$/,
                        use: 'ts-loader',
                        exclude: /node_modules/,
                    },
                ],
            },
            resolve: {
                extensions: [ '.ts', '.js' ],
            },
            output: {
                filename: 'bundle.js',
                path: path.resolve(__dirname, 'dist'),
            },
        },

        webpackMiddleware: {
            stats: 'errors-only',
        },
    });
};