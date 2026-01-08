//! Aether Compiler - World-Class Production Compiler
//! 
//! Compiles Aether source code to native binaries for all platforms.
//! Includes LSP server and package manager.

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod typechecker;
pub mod borrowck;
pub mod codegen;
pub mod binary;
pub mod runtime;
pub mod stdlib;
pub mod tooling;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "aetherc")]
#[command(about = "Aether Compiler - The fastest, most secure programming language")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Input source file (for direct compilation without subcommand)
    #[arg(global = true)]
    input: Option<PathBuf>,

    /// Output binary path
    #[arg(short, long, default_value = "a.out", global = true)]
    output: PathBuf,

    /// Target triple (e.g., aarch64-apple-darwin, x86_64-unknown-linux-gnu)
    #[arg(short, long, global = true)]
    target: Option<String>,

    /// Optimization level (0-3)
    #[arg(short = 'O', long, default_value = "2", global = true)]
    opt_level: u8,

    /// Emit assembly instead of binary (LLVM IR)
    #[arg(long, global = true)]
    emit_asm: bool,

    /// Enable debug info
    #[arg(short = 'g', long, global = true)]
    debug: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile source file to binary
    Build {
        /// Input source file
        input: PathBuf,
    },
    /// Start Language Server Protocol (LSP) server
    Lsp,
    /// Aether Package Manager
    Apm {
        #[command(subcommand)]
        action: ApmAction,
    },
}

#[derive(Subcommand, Debug)]
enum ApmAction {
    /// Install a package from git URL
    Install {
        /// Git repository URL
        url: String,
    },
    /// Initialize new Aether project
    Init {
        /// Project name
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Lsp) => {
            // Start LSP server
            let mut server = tooling::lsp::LspServer::new();
            server.start();
            Ok(())
        }
        Some(Commands::Apm { action }) => {
            match action {
                ApmAction::Install { url } => {
                    tooling::apm::Apm::install(url);
                }
                ApmAction::Init { name } => {
                    tooling::apm::Apm::init(name);
                }
            }
            Ok(())
        }
        Some(Commands::Build { input }) => {
            compile_file(input, &cli)
        }
        None => {
            // Legacy mode: direct file argument
            if let Some(input) = &cli.input {
                compile_file(input, &cli)
            } else {
                println!("AETHERC v1.0.0 - World-Class Compiler");
                println!("=====================================");
                println!("Usage: aetherc <FILE> or aetherc build <FILE>");
                println!("       aetherc lsp     - Start language server");
                println!("       aetherc apm     - Package manager");
                Ok(())
            }
        }
    }
}

fn compile_file(input: &PathBuf, cli: &Cli) -> anyhow::Result<()> {
    println!("AETHERC v1.0.0 - World-Class Compiler");
    println!("=====================================");
    
    // Read source
    let source = std::fs::read_to_string(input)?;
    if cli.verbose {
        println!("[1/5] Lexing {} ({} bytes)...", input.display(), source.len());
    }
    
    // Lex
    let tokens = lexer::tokenize(&source);
    if cli.verbose {
        println!("      {} tokens", tokens.len());
    }
    
    // Parse
    if cli.verbose {
        println!("[2/5] Parsing...");
    }
    let ast = parser::parse(&tokens)?;
    
    // Type check
    if cli.verbose {
        println!("[3/5] Type checking...");
    }
    let typed_ast = typechecker::check(&ast)?;
    
    // Borrow check
    if cli.verbose {
        println!("[4/5] Borrow checking...");
    }
    borrowck::check(&typed_ast)?;
    
    // LLVM Code Generation
    if cli.verbose {
        println!("[5/5] Generating LLVM IR...");
    }
    
    let mut llvm_gen = codegen::llvm::LLVMCodeGen::new();
    llvm_gen.emit_header();
    
    for typed_decl in &typed_ast.decls {
        llvm_gen.gen_function(&typed_decl.decl);
    }
    
    let ir = llvm_gen.get_ir();
    
    // Output handling
    if cli.emit_asm {
        // Just write .ll file if emit_asm requested (treating IR as assembly for now)
        let ll_path = cli.output.with_extension("ll");
        std::fs::write(&ll_path, ir)?;
        println!("✓ Generated LLVM IR: {}", ll_path.display());
    } else {
        // Compile using system clang
        // 1. Write temp .ll
        let ll_path = cli.output.with_extension("ll");
        std::fs::write(&ll_path, ir)?;
        
        if cli.verbose {
            println!("      Compiling with system clang -O3...");
        }
        
        // 2. Invoke clang
        let mut cmd = std::process::Command::new("clang");
        cmd.arg("-O3")
           .arg(&ll_path)
           .arg("-o")
           .arg(&cli.output);
           
        // Target settings if needed, but clang usually auto-detects host
        if let Some(target) = &cli.target {
            cmd.arg("--target").arg(target);
        }
        
        let output = cmd.output();
        
        match output {
            Ok(out) => {
                if !out.status.success() {
                    let err = String::from_utf8_lossy(&out.stderr);
                    anyhow::bail!("Clang compilation failed:\n{}", err);
                }
                println!("✓ Binary written to: {}", cli.output.display());
                
                // Cleanup intermediate file unless verbose
                if !cli.verbose && !cli.debug {
                    let _ = std::fs::remove_file(ll_path);
                }
            }
            Err(e) => {
                println!("! Error invoking clang: {}", e);
                println!("  Ensure clang is installed and in PATH.");
                println!("  Check generated IR at: {}", ll_path.display());
                return Err(e.into());
            }
        }
    }
    
    Ok(())
}
