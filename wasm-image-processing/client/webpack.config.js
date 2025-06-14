const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
	entry: "./bootstrap.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "bootstrap.js",
	},
	mode: "development",
	experiments: {
		asyncWebAssembly: true,
	},
	module: {
		rules: [
			{
				test: /\.wasm$/,
				type: "webassembly/async",
			},
		],
	},
	devServer: {
		port: 8080,
	},
	plugins: [new CopyWebpackPlugin(["index.html"])],
};
