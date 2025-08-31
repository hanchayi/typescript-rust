//! TypeScript compiler command line interface

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use ts_core::{compile, CompileOptions};
use ts_core::baseline_test::BaselineTestRunner;

/// TypeScript compiler implemented in Rust
#[derive(Parser)]
#[command(name = "ts-cli")]
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
    /// Run baseline tests
    Test {
        /// Test name pattern to filter tests
        #[arg(short, long)]
        pattern: Option<String>,
        
        /// Directory containing test cases
        #[arg(long, default_value = "tests/cases/compiler")]
        test_dir: PathBuf,
        
        /// Directory containing baseline files
        #[arg(long, default_value = "tests/baselines")]
        baseline_dir: PathBuf,
        
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { files, outdir, target, module, strict, sourcemap } => {
            compile_files(files, outdir, target, module, strict, sourcemap).await;
        }
        Commands::Check { files } => {
            check_files(files).await;
        }
        Commands::Test { pattern, test_dir, baseline_dir, verbose } => {
            run_baseline_tests(pattern, test_dir, baseline_dir, verbose).await;
        }
        Commands::Version => {
            println!("ts-cli version {}", env!("CARGO_PKG_VERSION"));
        }
    }
}

async fn compile_files(
    files: Vec<PathBuf>,
    _outdir: Option<PathBuf>,
    target: String,
    module: String,
    _strict: bool,
    sourcemap: bool,
) {
    for file in files {
        match std::fs::read_to_string(&file) {
            Ok(_source) => {
                let options = CompileOptions {
                    target: target.clone(),
                    module: module.clone(),
                    source_map: sourcemap,
                    file_name: file.to_string_lossy().to_string(),
                };
                
                match compile(&file, &options) {
                    Ok(result) => {
                        if result.diagnostics.is_empty() {
                            println!("{}: Compiled successfully", file.display().to_string().green());
                        } else {
                            for diagnostic in &result.diagnostics {
                                println!("{}: {}", "warning".yellow(), diagnostic.message);
                            }
                        }
                    }
                    Err(diagnostics) => {
                        for diagnostic in &diagnostics {
                            eprintln!("{}: {}", "error".red(), diagnostic.message);
                        }
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

async fn run_baseline_tests(
    pattern: Option<String>,
    test_dir: PathBuf,
    baseline_dir: PathBuf,
    verbose: bool,
) {
    println!("{}", "Running baseline tests...".blue().bold());
    
    if verbose {
        println!("Test directory: {}", test_dir.display());
        println!("Baseline directory: {}", baseline_dir.display());
        if let Some(ref p) = pattern {
            println!("Pattern filter: {}", p);
        }
    }
    
    let runner = BaselineTestRunner::new(test_dir, baseline_dir);
    let results = runner.run_tests(pattern.as_deref()).await;
    
    // 统计结果
    let total_tests = results.len();
    let passed_tests = results.iter().filter(|r| r.passed).count();
    let failed_tests = total_tests - passed_tests;
    
    if failed_tests > 0 {
        println!("{}", format!("❌ {} tests failed", failed_tests).red().bold());
        std::process::exit(1);
    } else {
        println!("{}", format!("✅ All {} tests passed!", total_tests).green().bold());
    }
}