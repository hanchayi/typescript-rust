/**
 * VSCode extension entry point for TypeScript Rust compiler
 */

import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';
import { WorkerCompiler } from './worker';

let client: LanguageClient;
let workerCompiler: WorkerCompiler;

/**
 * Extension activation function
 */
export async function activate(context: vscode.ExtensionContext) {
    console.log('TypeScript Rust extension is being activated');
    
    try {
        // Initialize Web Worker compiler
        const useWebWorker = vscode.workspace.getConfiguration('typescriptRust').get<boolean>('useWebWorker', true);
        
        if (useWebWorker) {
            workerCompiler = new WorkerCompiler(context.extensionUri);
            await workerCompiler.init();
            console.log('Web Worker compiler initialized');
        }
        
        // Start Language Server
        await startLanguageServer(context);
        
        // Register commands
        registerCommands(context);
        
        console.log('TypeScript Rust extension activated successfully');
    } catch (error) {
        console.error('Failed to activate TypeScript Rust extension:', error);
        vscode.window.showErrorMessage(`Failed to activate TypeScript Rust extension: ${error}`);
    }
}

/**
 * Extension deactivation function
 */
export function deactivate(): Thenable<void> | undefined {
    if (client) {
        return client.stop();
    }
    
    if (workerCompiler) {
        workerCompiler.dispose();
    }
    
    return undefined;
}

/**
 * Start the Language Server
 */
async function startLanguageServer(context: vscode.ExtensionContext) {
    // Language server executable path
    const serverCommand = 'ts-lsp';
    
    const serverOptions: ServerOptions = {
        command: serverCommand,
        args: ['--stdio']
    };
    
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'typescript' },
            { scheme: 'file', language: 'typescriptreact' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{ts,tsx}')
        },
        outputChannelName: 'TypeScript Rust Language Server'
    };
    
    client = new LanguageClient(
        'typescriptRust',
        'TypeScript Rust Language Server',
        serverOptions,
        clientOptions
    );
    
    try {
        await client.start();
        console.log('Language Server started successfully');
    } catch (error) {
        console.error('Failed to start Language Server:', error);
        vscode.window.showWarningMessage('TypeScript Rust Language Server failed to start. Some features may not be available.');
    }
}

/**
 * Register extension commands
 */
function registerCommands(context: vscode.ExtensionContext) {
    // Compile command
    const compileCommand = vscode.commands.registerCommand(
        'typescriptRust.compile',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('No active TypeScript file');
                return;
            }
            
            if (!editor.document.fileName.endsWith('.ts') && !editor.document.fileName.endsWith('.tsx')) {
                vscode.window.showWarningMessage('Current file is not a TypeScript file');
                return;
            }
            
            try {
                await compileCurrentFile(editor);
            } catch (error) {
                vscode.window.showErrorMessage(`Compilation failed: ${error}`);
            }
        }
    );
    
    // Type check command
    const checkCommand = vscode.commands.registerCommand(
        'typescriptRust.check',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('No active TypeScript file');
                return;
            }
            
            try {
                await typeCheckCurrentFile(editor);
            } catch (error) {
                vscode.window.showErrorMessage(`Type checking failed: ${error}`);
            }
        }
    );
    
    context.subscriptions.push(compileCommand, checkCommand);
}

/**
 * Compile the current file
 */
async function compileCurrentFile(editor: vscode.TextEditor) {
    const source = editor.document.getText();
    const fileName = editor.document.fileName;
    
    // Get configuration
    const config = vscode.workspace.getConfiguration('typescriptRust');
    const options = {
        fileName,
        config: {
            target: config.get<string>('target', 'es2020'),
            module: config.get<string>('module', 'es2020'),
            strict: config.get<boolean>('strict', true),
            source_map: false
        }
    };
    
    // Show progress
    await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: 'Compiling TypeScript...',
        cancellable: false
    }, async (progress) => {
        progress.report({ increment: 0 });
        
        if (workerCompiler) {
            const result = await workerCompiler.compile(source, options);
            
            progress.report({ increment: 100 });
            
            // Show compilation result
            if (result.diagnostics && result.diagnostics.length > 0) {
                const diagnosticsText = result.diagnostics.map((d: any) => d.message || d).join('\n');
                vscode.window.showWarningMessage(`Compilation completed with warnings:\n${diagnosticsText}`);
            } else {
                vscode.window.showInformationMessage('Compilation completed successfully!');
            }
            
            // Optionally show the generated JavaScript
            if (result.code) {
                const doc = await vscode.workspace.openTextDocument({
                    content: result.code,
                    language: 'javascript'
                });
                await vscode.window.showTextDocument(doc);
            }
        } else {
            throw new Error('Web Worker compiler not available');
        }
    });
}

/**
 * Type check the current file
 */
async function typeCheckCurrentFile(editor: vscode.TextEditor) {
    // Implementation for type checking
    vscode.window.showInformationMessage('Type checking feature coming soon!');
}