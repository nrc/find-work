module.exports = {
  entry: "./src/index.js",
  output: {
    filename: "../static/work.out.js",
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
  devServer: {
    contentBase: '.',
    historyApiFallback: true
  },
  devtool: 'source-map'
}
