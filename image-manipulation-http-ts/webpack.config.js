const path = require('path');
const SpinSdkPlugin = require("@fermyon/spin-sdk/plugins/webpack")

module.exports = {
    entry: './src/spin.ts',
    experiments: {
        outputModule: true,
    },
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
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        path: path.resolve(__dirname, './'),
        filename: 'dist.js',
        module: true,
        library: {
            type: "module",
        }
    },
    // Need to add this one because component imports cannot be resolved by
    // webpack at build time as it is satisfied by Spin at runtime
    externals: {
        "component:image-manipulation-lib/image-manipulation": "component:image-manipulation-lib/image-manipulation"
    },
    plugins: [
        new SpinSdkPlugin()
    ],
    optimization: {
        minimize: false
    },
};