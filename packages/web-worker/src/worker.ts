/**
 * Web Worker for TypeScript compilation
 * Runs the Rust-based TypeScript compiler in a separate thread
 */

import init, { WasmCompiler } from '../pkg/ts_wasm';

// Message types for communication with main thread
interface InitMessage {
    id: string;
    type: 'init';
}

interface CompileMessage {
    id: string;
    type: 'compile';
    payload: {
        source: string;
        options: any;
    };
}

type WorkerMessage = InitMessage | CompileMessage;

interface WorkerResponse {
    id: string;
    type: 'init-success' | 'compile-success' | 'error';
    result?: any;
    error?: string;
}

let compiler: WasmCompiler | null = null;
let isInitialized = false;

/**
 * Handle messages from the main thread
 */
self.onmessage = async (event: MessageEvent<WorkerMessage>) => {
    const { id, type } = event.data;
    
    try {
        switch (type) {
            case 'init':
                await initializeCompiler();
                postResponse({ id, type: 'init-success' });
                break;
                
            case 'compile':
                if (!isInitialized || !compiler) {
                    throw new Error('Compiler not initialized. Call init first.');
                }
                
                const { source, options } = event.data.payload;
                const result = compiler.compile(source, options);
                postResponse({ id, type: 'compile-success', result });
                break;
                
            default:
                throw new Error(`Unknown message type: ${type}`);
        }
    } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        postResponse({ 
            id, 
            type: 'error', 
            error: errorMessage 
        });
    }
};

/**
 * Initialize the WASM compiler
 */
async function initializeCompiler(): Promise<void> {
    if (isInitialized) {
        return;
    }
    
    try {
        // Initialize the WASM module
        await init();
        
        // Create compiler instance
        compiler = new WasmCompiler();
        isInitialized = true;
        
        console.log('TypeScript Rust compiler initialized in Web Worker');
    } catch (error) {
        console.error('Failed to initialize compiler:', error);
        throw error;
    }
}

/**
 * Send response back to main thread
 */
function postResponse(response: WorkerResponse): void {
    self.postMessage(response);
}

/**
 * Handle worker errors
 */
self.onerror = (error) => {
    console.error('Worker error:', error);
};

self.onunhandledrejection = (event) => {
    console.error('Unhandled promise rejection in worker:', event.reason);
};