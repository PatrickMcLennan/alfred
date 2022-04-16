'use strict';
// eslint-disable-next-line
const common = require('./webpack.common');
// eslint-disable-next-line
const path = require('path');
// eslint-disable-next-line
const { merge } = require('webpack-merge');
// eslint-disable-next-line
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
// eslint-disable-next-line
const HtmlWebpackPlugin = require('html-webpack-plugin');
// eslint-disable-next-line
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
// eslint-disable-next-line
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
// eslint-disable-next-line
const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin');
// eslint-disable-next-line
const { BundleAnalyzerPlugin } = require('webpack-bundle-analyzer');
// eslint-disable-next-line
const TerserPlugin = require('terser-webpack-plugin');

module.exports = merge(common, {
  mode: `production`,
  module: {
    rules: [
      {
        test: /\.(sa|sc|c)ss/,
        use: [
          { loader: MiniCssExtractPlugin.loader },
          { loader: `css-loader` },
          {
            loader: `postcss-loader`,
          },
          { loader: `sass-loader` },
        ],
      },
    ],
  },
  optimization: {
    minimizer: [
      new OptimizeCSSAssetsPlugin({}),
      new TerserPlugin({
        extractComments: false,
      }),
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: `index.html`,
      template: path.resolve(__dirname, `../../ui/template.html`),
      inject: `head`,
      minify: {
        collapseWhitespace: true,
        removeComments: true,
      },
    }),
    new ScriptExtHtmlWebpackPlugin({
      defaultAttribute: 'defer',
    }),
    new MiniCssExtractPlugin({
      filename: `[name].[contenthash].css`,
      chunkFilename: `[name].[contenthash].css`,
    }),
    new CleanWebpackPlugin(),
    new BundleAnalyzerPlugin(),
  ],
});
