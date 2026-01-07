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

    /// Emit assembly instead of binary
    #[arg(long, global = true)]
    emit_asm: bool,

    /// Emit LLVM IR and use LLVM opt/llc for maximum optimization
    #[arg(long, global = true)]
    emit_llvm: bool,

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
        println!("[1/6] Lexing {} ({} bytes)...", input.display(), source.len());
    }
    
    // Lex
    let tokens = lexer::tokenize(&source);
    if cli.verbose {
        println!("      {} tokens", tokens.len());
    }
    
    // Parse
    if cli.verbose {
        println!("[2/6] Parsing...");
    }
    let ast = parser::parse(&tokens)?;
    if cli.verbose {
        println!("      {} declarations", ast.decls.len());
    }
    
    // Type check
    if cli.verbose {
        println!("[3/6] Type checking...");
    }
    let typed_ast = typechecker::check(&ast)?;
    
    // Borrow check
    if cli.verbose {
        println!("[4/6] Borrow checking...");
    }
    borrowck::check(&typed_ast)?;
    
    // Determine target
    let target = cli.target.clone().unwrap_or_else(|| {
        #[cfg(target_os = "macos")]
        #[cfg(target_arch = "aarch64")]
        return "aarch64-apple-darwin".to_string();
        
        #[cfg(target_os = "macos")]
        #[cfg(target_arch = "x86_64")]
        return "x86_64-apple-darwin".to_string();
        
        #[cfg(target_os = "linux")]
        #[cfg(target_arch = "aarch64")]
        return "aarch64-unknown-linux-gnu".to_string();
        
        #[cfg(target_os = "linux")]
        #[cfg(target_arch = "x86_64")]
        return "x86_64-unknown-linux-gnu".to_string();
        
        #[cfg(target_os = "windows")]
        return "x86_64-pc-windows-msvc".to_string();
        
        #[allow(unreachable_code)]
        "x86_64-unknown-linux-gnu".to_string()
    });
    
    // LLVM Mode
    if cli.emit_llvm {
        let ll_path = cli.output.with_extension("ll");
        let mut llvm_gen = codegen::llvm::LLVMCodeGen::new();
        llvm_gen.emit_header();
        
        // Generate IR for each function using full codegen
        for typed_decl in &typed_ast.decls {
            llvm_gen.gen_function(&typed_decl.decl);
        }
        
        std::fs::write(&ll_path, llvm_gen.get_ir())?;
        println!("✓ Generated LLVM IR: {}", ll_path.display());
        
        return Ok(());
    }
    
    // Code generation
    if cli.verbose {
        println!("[5/6] Generating code for {}...", target);
    }
    let mut codegen = codegen::CodeGen::new(&target, cli.opt_level);
    codegen.generate(&typed_ast);
    
    // Write binary
    if cli.verbose {
        println!("[6/6] Writing binary...");
    }
    
    if cli.emit_asm {
        let asm = codegen.get_asm();
        let asm_path = cli.output.with_extension("s");
        std::fs::write(&asm_path, asm)?;
        println!("✓ Assembly written to: {}", asm_path.display());
    } else {
        binary::write(&cli.output, codegen.get_asm().as_bytes(), &target)?;
        println!("✓ Binary written to: {}", cli.output.display());
    }
    
    Ok(())
}
