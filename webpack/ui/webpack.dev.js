'use strict';

// eslint-disable-next-line
const common = require('./webpack.common');
// eslint-disable-next-line
const { merge } = require('webpack-merge');
// eslint-disable-next-line
const HtmlWebpackPlugin = require('html-webpack-plugin');
// eslint-disable-next-line
const path = require('path');
// eslint-disable-next-line
const fs = require('fs');
// eslint-disable-next-line
const { config } = require('dotenv');

config({ path: path.resolve(__dirname, `../../.env`) });

module.exports = merge(common, {
  mode: `development`,
  devServer: {
    client: {
      overlay: {
        errors: true,
        warnings: false,
      },
    },
    compress: true,
    historyApiFallback: true,
    hot: true,
    open: true,
    port: 3000,
    proxy: {
      '/api/**': {
        target: process.env.API_GATEWAY_URL,
        secure: false,
        changeOrigin: true,
        pathRewrite: {
          '^/api': '',
        },
      },
    },
    server: {
      type: 'https',
      options: {
        cert: fs.readFileSync(path.resolve(__dirname, `../../private.crt`)),
        key: fs.readFileSync(path.resolve(__dirname, `../../private.key`)),
        ca: fs.readFileSync(path.resolve(__dirname, `../../private.pem`)),
      },
    },
  },
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss/,
        use: [
          { loader: `style-loader` },
          {
            loader: `css-loader`,
            options: {
              sourceMap: true,
            },
          },
          {
            loader: `sass-loader`,
            options: {
              sourceMap: true,
            },
          },
        ],
      },
    ],
  },
  devtool: `source-map`,
  plugins: [
    new HtmlWebpackPlugin({
      filename: `index.html`,
      template: path.resolve(__dirname, `../../ui/template.html`),
      inject: `body`,
    }),
  ],
});
