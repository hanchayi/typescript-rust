//! TypeScript compiler command line interface

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use ts_core::{Compiler, CompileOptions, CompilerConfig, TargetVersion, ModuleKind};

/// TypeScript compiler implemented in Rust
#[derive(Parser)]
#[command(name = "tsc-rust")]
#[command(about = "A TypeScript compiler implemented in Rust")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile TypeScript files
    Compile {
        /// Input TypeScript files
        #[arg(required = true)]
        files: Vec<PathBuf>,
        
        /// Output directory
        #[arg(short, long)]
        outdir: Option<PathBuf>,
        
        /// Target JavaScript version
        #[arg(long, default_value = "es2020")]
        target: String,
        
        /// Module system
        #[arg(long, default_value = "es2020")]
        module: String,
        
        /// Enable strict mode
        #[arg(long)]
        strict: bool,
        
        /// Generate source maps
        #[arg(long)]
        sourcemap: bool,
    },
    /// Check TypeScript files for errors
    Check {
        /// Input TypeScript files
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Compile { 
            files, 
            outdir, 
            target, 
            module, 
            strict, 
            sourcemap 
        } => {
            compile_files(files, outdir, target, module, strict, sourcemap).await;
        }
        Commands::Check { files } => {
            check_files(files).await;
        }
        Commands::Version => {
            println!("tsc-rust {}", env!("CARGO_PKG_VERSION"));
        }
    }
}

async fn compile_files(
    files: Vec<PathBuf>,
    _outdir: Option<PathBuf>,
    target: String,
    module: String,
    strict: bool,
    sourcemap: bool,
) {
    let target_version = match target.as_str() {
        "es5" => TargetVersion::ES5,
        "es2015" => TargetVersion::ES2015,
        "es2017" => TargetVersion::ES2017,
        "es2018" => TargetVersion::ES2018,
        "es2019" => TargetVersion::ES2019,
        "es2020" => TargetVersion::ES2020,
        "es2021" => TargetVersion::ES2021,
        "es2022" => TargetVersion::ES2022,
        "esnext" => TargetVersion::ESNext,
        _ => {
            eprintln!("{}: Unknown target '{}'", "error".red(), target);
            return;
        }
    };
    
    let module_kind = match module.as_str() {
        "none" => ModuleKind::None,
        "commonjs" => ModuleKind::CommonJS,
        "amd" => ModuleKind::AMD,
        "umd" => ModuleKind::UMD,
        "system" => ModuleKind::System,
        "es2015" => ModuleKind::ES2015,
        "es2020" => ModuleKind::ES2020,
        "es2022" => ModuleKind::ES2022,
        "esnext" => ModuleKind::ESNext,
        _ => {
            eprintln!("{}: Unknown module system '{}'", "error".red(), module);
            return;
        }
    };
    
    let config = CompilerConfig {
        target: target_version,
        module: module_kind,
        strict,
        source_map: sourcemap,
    };
    
    let mut compiler = Compiler::with_config(config);
    
    for file in files {
        match std::fs::read_to_string(&file) {
            Ok(source) => {
                let options = CompileOptions {
                    file_name: file.to_string_lossy().to_string(),
                    config: compiler.config.clone(),
                };
                
                match compiler.compile(&source, options) {
                    Ok(result) => {
                        if result.diagnostics.is_empty() {
                            println!("{}: Compiled successfully", file.display().to_string().green());
                        } else {
                            for diagnostic in &result.diagnostics {
                                println!("{}: {}", "warning".yellow(), diagnostic);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: Failed to compile {}: {}", "error".red(), file.display(), e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: Failed to read {}: {}", "error".red(), file.display(), e);
            }
        }
    }
}

async fn check_files(files: Vec<PathBuf>) {
    println!("{}: Type checking {} files...", "info".blue(), files.len());
    // Implementation for type checking without compilation
}