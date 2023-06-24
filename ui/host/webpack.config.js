const HtmlWebpackPlugin = require('html-webpack-plugin');
const ModuleFederationPlugin = require('webpack/lib/container/ModuleFederationPlugin');
const port = 8081;

module.exports = {
  mode: 'development',
  output: {
    publicPath: `http://localhost:${port}/`,
  },
  devServer: {
    port: port,
    historyApiFallback: true,
    open: true
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/,
        exclude: /node_modules/,
        use: 'ts-loader',
      },
      {
          test: /\.(css)$/,
          exclude: /node_modules/,
          use: ['style-loader', 'css-loader']
        },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.css'],
  },
  plugins: [
    new ModuleFederationPlugin({
      name: 'host',
      remotes: {
        PromotionsComponent: 'PromotionsComponent@http://localhost:8082/promotionsApp.js',
      },
    }),
    new HtmlWebpackPlugin({
      template: './public/index.html'
    }),
  ]
};
