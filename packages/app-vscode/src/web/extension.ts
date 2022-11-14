// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
	__webpack_public_path__ =
    context.extensionUri.toString().replace("file:///", "") + "/dist/web/";
	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "app-vscode" is now active in the web extension host!');

	let disposable = vscode.commands.registerCommand(
		"app-vscode.helloWorld",
		() => {
		  require("browser").then((browser: any) => {
			vscode.window.showInformationMessage(browser.greet());
		  });
		}
	);
	
	context.subscriptions.push(disposable);
}

// this method is called when your extension is deactivated
export function deactivate() {}
