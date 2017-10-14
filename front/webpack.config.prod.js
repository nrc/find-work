const UglifyJSPlugin = require('uglifyjs-webpack-plugin');
const webpack = require('webpack');

module.exports = {
  entry: "./src/index.js",
  output: {
    path: __dirname + '/../static',
    filename: 'work.out.js',
    libraryTarget: 'var',
    library: 'Work'
  },
  module: {
    loaders: [
    {
      test: /\.js$/,
      exclude: /node_modules/,
      loader: 'babel-loader'
    }]
  },
  plugins: [
    new UglifyJSPlugin(),
    new webpack.DefinePlugin({
      'process.env': {
        'FINDWORK_API': JSON.stringify('https://www.rustaceans.org/findwork/data/'),
        'NODE_ENV': JSON.stringify('production')
      }
    })
  ]
}
