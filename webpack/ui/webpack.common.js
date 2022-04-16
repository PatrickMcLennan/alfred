'use strict';
// eslint-disable-next-line
const path = require('path');

module.exports = {
  entry: {
    app: path.resolve(__dirname, `../../ui/index.ts`),
  },
  output: {
    path: path.resolve(__dirname, `../../ui/dist`),
    filename: `app.[contenthash].js`,
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx|js|jsx)$/,
        exclude: /(node_modules)/,
        use: `swc-loader`,
      },
      {
        test: /\.(png|svg|jpg|jpeg|gif)$/i,
        type: 'asset/resource',
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: 'asset/resource',
      },
    ],
  },
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx'],
  },
};
