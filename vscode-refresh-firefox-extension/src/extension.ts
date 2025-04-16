import * as vscode from 'vscode';
import { exec } from 'child_process';

export function activate(context: vscode.ExtensionContext) {
    let disposable = vscode.workspace.onDidSaveTextDocument((document) => {
        // Check if the saved file is a CSS file
        if (!document.fileName.endsWith('.css')) {
            return;
        }

        const scriptPath = '../refresh_firefox.sh';
        exec(scriptPath, (error, stdout, stderr) => {
            if (error) {
                console.error(`Error executing script: ${error.message}`);
                return;
            }
            if (stderr) {
                console.error(`Script error: ${stderr}`);
                return;
            }
            console.log(`Script output: ${stdout}`);
        });
    });

    context.subscriptions.push(disposable);
}

export function deactivate() {}