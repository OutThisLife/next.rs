const fs = require('fs')
const path = require('path')
const webpack = require('webpack')
const HtmlWebpackPlugin = require('html-webpack-plugin')

const prod = process.env.NODE_ENV === 'production'

module.exports = {
  devServer: {
    contentBase: path.join(__dirname, 'src'),
    compress: true,
    port: 3000
  },
  bail: true,
  devtool: 'source-map',
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
      {
        oneOf: [
          {
            test: /\.js$/,
            exclude: /(node_modules)/,
            use: {
              loader: 'babel-loader',
              options: {
                compact: true,
                ...JSON.parse(fs.readFileSync(path.resolve(__dirname, './.babelrc')))
              }
            }
          },
          {
            test: /\.rs$/,
            use: [
              {
                loader: 'babel-loader',
                options: {
                  compact: true,
                  ...JSON.parse(fs.readFileSync(path.resolve(__dirname, './.babelrc')))
                }
              },
              {
                loader: 'rust-native-wasm-loader',
                options: {
                  release: true,
                  // release: prod,
                  // gc: prod,
                  wasmBindgen: {
                    wasm2es6js: true
                  }
                }
              }
            ]
          }
        ]
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      inject: true,
      template: './src/index.html',
      minify: {
        removeComments: true,
        collapseWhitespace: true,
        removeRedundantAttributes: true,
        useShortDoctype: true,
        removeEmptyAttributes: true,
        removeStyleLinkTypeAttributes: true,
        keepClosingSlash: true,
        minifyJS: true,
        minifyCSS: true,
        minifyURLs: true
      }
    })
  ],
  node: {
    dgram: 'empty',
    fs: 'empty',
    net: 'empty',
    tls: 'empty',
    child_process: 'empty'
  }
}
