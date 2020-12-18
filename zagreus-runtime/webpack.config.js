const path = require('path');

module.exports = {
    entry: './src/runtime.ts',
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
        filename: 'zagreus-runtime.js',
        path: path.resolve(__dirname, 'dist'),
    },
    devServer: {
        open: true,
        openPage: 'api/template/test-template',
        publicPath: '/api/template/test-template',
        contentBase: path.join(__dirname, 'dist'),
        contentBasePublicPath: '/api/template/test-template',
        compress: true,
        port: 9000,
        proxy: {
            '/api/template/test-template/ws': {
                target: 'http://localhost:80/api/template/test-template/ws',
                ws: true,
                changeOrigin: true,
            }
        }
    }
};
