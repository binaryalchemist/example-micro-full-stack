const HtmlWebpackPlugin = require('html-webpack-plugin');
const port = 8082;

module.exports = {
    mode: 'development',
    output: {
      publicPath: `http://localhost:${port}/`,
    },
    devServer: {
      port: port,
      historyApiFallback: true,
    },
    module: {
      rules: [
        {
          test: /\.m?js$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: {
              presets: ['@babel/preset-react', '@babel/preset-env'],
              plugins: ['@babel/plugin-transform-runtime'],
            },
          },
        },
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: './public/index.html',
      }),
    ],
};
