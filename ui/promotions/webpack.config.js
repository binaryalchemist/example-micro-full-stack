const HtmlWebpackPlugin = require('html-webpack-plugin');
const ModuleFederationPlugin = require('webpack/lib/container/ModuleFederationPlugin');
const port = 8082;

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
      name: 'promotions',
      library: {type: 'var', name: 'promotions'},
      filename: 'promotionsApp.js',
      exposes: {
        PromotionsComponent: './src/Promotions.tsx'
      }
    }),
    new HtmlWebpackPlugin({
      template: './public/index.html'
    }),
  ]
};
  