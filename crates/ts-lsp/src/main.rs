//! TypeScript Language Server main entry point

use clap::Parser;
use tower_lsp::{LspService, Server};
use tracing_subscriber;

mod server;
mod handlers;

use server::TypeScriptLanguageServer;

/// Command line arguments for the language server
#[derive(Parser, Debug)]
#[command(name = "ts-lsp")]
#[command(about = "TypeScript Language Server implemented in Rust")]
struct Args {
    /// Use stdio for communication
    #[arg(long)]
    stdio: bool,
    
    /// TCP port to listen on
    #[arg(long)]
    port: Option<u16>,
    
    /// Enable debug logging
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // Initialize logging
    if args.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }
    
    // Create the language server
    let (service, socket) = LspService::new(|client| {
        TypeScriptLanguageServer::new(client)
    });
    
    // Start the server
    if args.stdio {
        tracing::info!("Starting TypeScript Language Server on stdio");
        Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
            .serve(service)
            .await;
    } else if let Some(port) = args.port {
        tracing::info!("Starting TypeScript Language Server on port {}", port);
        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .expect("Failed to bind to port");
            
        let (stream, _) = listener.accept().await.expect("Failed to accept connection");
        let (read, write) = tokio::io::split(stream);
        
        Server::new(read, write, socket)
            .serve(service)
            .await;
    } else {
        eprintln!("Must specify either --stdio or --port");
        std::process::exit(1);
    }
}