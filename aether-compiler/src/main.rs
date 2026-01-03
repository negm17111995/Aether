//! Aether Compiler - World-Class Production Compiler
//! 
//! Compiles Aether source code to native binaries for all platforms.

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod typechecker;
pub mod borrowck;
pub mod codegen;
pub mod binary;
pub mod runtime;
pub mod stdlib;

use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aetherc")]
#[command(about = "Aether Compiler - The fastest, most secure programming language")]
#[command(version = "1.0.0")]
struct Args {
    /// Input source file
    #[arg(required = true)]
    input: PathBuf,

    /// Output binary path
    #[arg(short, long, default_value = "a.out")]
    output: PathBuf,

    /// Target triple (e.g., aarch64-apple-darwin, x86_64-unknown-linux-gnu)
    #[arg(short, long)]
    target: Option<String>,

    /// Optimization level (0-3)
    #[arg(short = 'O', long, default_value = "2")]
    opt_level: u8,

    /// Emit assembly instead of binary
    #[arg(long)]
    emit_asm: bool,

    /// Enable debug info
    #[arg(short = 'g', long)]
    debug: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    println!("AETHERC v1.0.0 - World-Class Compiler");
    println!("=====================================");
    
    // Read source
    let source = std::fs::read_to_string(&args.input)?;
    if args.verbose {
        println!("[1/6] Lexing {} ({} bytes)...", args.input.display(), source.len());
    }
    
    // Lex
    let tokens = lexer::tokenize(&source);
    if args.verbose {
        println!("      {} tokens", tokens.len());
    }
    
    // Parse
    if args.verbose {
        println!("[2/6] Parsing...");
    }
    let ast = parser::parse(&tokens)?;
    if args.verbose {
        println!("      {} declarations", ast.decls.len());
    }
    
    // Type check
    if args.verbose {
        println!("[3/6] Type checking...");
    }
    let typed_ast = typechecker::check(&ast)?;
    
    // Borrow check
    if args.verbose {
        println!("[4/6] Borrow checking...");
    }
    borrowck::check(&typed_ast)?;
    
    // Determine target
    let target = args.target.unwrap_or_else(|| {
        #[cfg(target_os = "macos")]
        #[cfg(target_arch = "aarch64")]
        return "aarch64-apple-darwin".to_string();
        
        #[cfg(target_os = "macos")]
        #[cfg(target_arch = "x86_64")]
        return "x86_64-apple-darwin".to_string();
        
        #[cfg(target_os = "linux")]
        return "x86_64-unknown-linux-gnu".to_string();
        
        #[cfg(target_os = "windows")]
        return "x86_64-pc-windows-msvc".to_string();
        
        #[allow(unreachable_code)]
        "x86_64-unknown-linux-gnu".to_string()
    });
    
    // Code generation
    if args.verbose {
        println!("[5/6] Generating code for {}...", target);
    }
    let code = codegen::generate(&typed_ast, &target, args.opt_level)?;
    if args.verbose {
        println!("      {} bytes of machine code", code.len());
    }
    
    // Emit
    if args.emit_asm {
        let asm_path = args.output.with_extension("s");
        std::fs::write(&asm_path, codegen::disassemble(&code, &target))?;
        println!("Wrote assembly to {}", asm_path.display());
    } else {
        if args.verbose {
            println!("[6/6] Writing binary...");
        }
        binary::write(&args.output, &code, &target)?;
        
        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&args.output)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&args.output, perms)?;
        }
        
        println!("✓ Compiled {} → {}", args.input.display(), args.output.display());
    }
    
    Ok(())
}
