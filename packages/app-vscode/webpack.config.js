/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

//@ts-check
'use strict';

//@ts-check
/** @typedef {import('webpack').Configuration} WebpackConfig **/

const path = require('path');
const webpack = require('webpack');
const wasmPlugin = require("vscode-web-wasm-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
/** @type WebpackConfig */
const webExtensionConfig = {
	mode: 'none', // this leaves the source code as close as possible to the original (when packaging we set this to 'production')
	target: 'webworker', // extensions run in a webworker context
	experiments: {
	    asyncWebAssembly: true,
	},
	entry: {
		'extension': './src/web/extension.ts',
		'test/suite/index': './src/web/test/suite/index.ts'
	},
	output: {
		filename: '[name].js',
		path: path.join(__dirname, './dist/web'),
		libraryTarget: 'commonjs',
		devtoolModuleFilenameTemplate: '../../[resource-path]',
		enabledWasmLoadingTypes: ["async-vscode"],
		wasmLoading: "async-vscode",
	},
	resolve: {
		mainFields: ['browser', 'module', 'main'], // look for `browser` entry point in imported node modules
		extensions: ['.ts', '.js'], // support ts-files and js-files
		alias: {
			// provides alternate implementation for node module and source files
		},
		fallback: {
			fs: false,
			child_process: false,
			net: false,
			crypto: false,
			path: require.resolve("path-browserify"),
			'assert': require.resolve('assert')
		  },
	},
	module: {
		rules: [{
			test: /\.ts$/,
			exclude: /node_modules/,
			use: [{
				loader: 'ts-loader'
			}]
		}]
	},
	plugins: [
		new webpack.optimize.LimitChunkCountPlugin({
			maxChunks: 1, // disable chunks by default since web extensions must be a single bundle
		}),
		new webpack.ProvidePlugin({
			process: 'process/browser', // provide a shim for the global `process` variable
		}),
		new wasmPlugin.ReadFileVsCodeWebCompileAsyncWasmPlugin(),
		new WasmPackPlugin({
		  crateDirectory: path.resolve(__dirname, "../../crates/browser"),
		  forceMode: "production",
		}),
		new webpack.ProgressPlugin(),
		new CleanWebpackPlugin(),
		new CopyWebpackPlugin({
		  patterns: [
			{
			  from: "../../node_modules/web-tree-sitter/tree-sitter.wasm",
			},
		  ],
		}),
		new HtmlWebpackPlugin({
		  template: "../app/assets/index.html",
		  scriptLoading: "module",
		  title: "tower-lsp web demo",
		}),
	],
	externals: {
		'vscode': 'commonjs vscode', // ignored because it doesn't exist
	},
	performance: {
		hints: false
	},
	devtool: 'nosources-source-map', // create a source map that points to the original source file
	infrastructureLogging: {
		level: "log", // enables logging required for problem matchers
	},
};

module.exports = [ webExtensionConfig ];
