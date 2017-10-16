const webpack = require('webpack');

module.exports = {
  entry: "./src/index.js",
  output: {
    path: __dirname + '/../static',
    filename: 'work.out.js',
    libraryTarget: 'var',
    library: 'Work',
    pathinfo: true
  },
  module: {
    loaders: [
    {
      test: /\.js$/,
      exclude: /node_modules/,
      loader: 'babel-loader'
    }]
  },
  devServer: {
    contentBase: '.',
    historyApiFallback: true
  },
  devtool: 'source-map',
  plugins: [
    new webpack.DefinePlugin({
      'process.env': {
        'FINDWORK_API': JSON.stringify('/data/')
      }
    })
  ]
}
