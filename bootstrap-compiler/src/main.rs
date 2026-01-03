// AETHER BOOTSTRAP COMPILER - STAGE 0
// A Rust compiler that compiles Aether source to native or LLVM IR
// This is the "host" compiler used to bootstrap the self-hosted Aether compiler

mod lexer;
mod parser;
mod codegen;
mod llvm_codegen;
mod binary;

use std::env;
use std::fs;
use std::process;
use std::process::Command;

fn print_usage() {
    eprintln!("Aether Bootstrap Compiler (Stage 0)");
    eprintln!("");
    eprintln!("Usage: aether_bootstrap [OPTIONS] <source.aether> [output]");
    eprintln!("");
    eprintln!("Options:");
    eprintln!("  --emit-llvm    Output LLVM IR instead of native binary");
    eprintln!("  --version      Show version information");
    eprintln!("  --help         Show this help message");
    eprintln!("");
    eprintln!("Examples:");
    eprintln!("  aether_bootstrap hello.aether hello       # Compile to native");
    eprintln!("  aether_bootstrap --emit-llvm hello.aether hello.ll  # Output LLVM IR");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    if args[1] == "--version" || args[1] == "-v" {
        println!("Aether Bootstrap Compiler (Stage 0)");
        println!("Version: 1.0.0");
        println!("Targets: ARM64 (native), LLVM IR");
        println!("Written in Rust for bootstrapping purposes only");
        return;
    }
    
    if args[1] == "--help" || args[1] == "-h" {
        print_usage();
        return;
    }
    
    // Parse options
    let mut emit_llvm = false;
    let mut source_idx = 1;
    
    if args[1] == "--emit-llvm" {
        emit_llvm = true;
        source_idx = 2;
    }
    
    if source_idx >= args.len() {
        eprintln!("Error: No source file specified");
        process::exit(1);
    }
    
    let source_path = &args[source_idx];
    let output_path = if args.len() > source_idx + 1 { 
        args[source_idx + 1].clone() 
    } else if emit_llvm {
        source_path.replace(".aether", ".ll")
    } else { 
        "a.out".to_string() 
    };
    
    // Read source file
    let source = match fs::read_to_string(source_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", source_path, e);
            process::exit(1);
        }
    };
    
    println!("[1/4] Lexing {}...", source_path);
    let tokens = lexer::lex(&source);
    println!("      {} tokens", tokens.len());
    
    println!("[2/4] Parsing...");
    let ast = parser::parse(&tokens);
    println!("      {} declarations", ast.decls.len());
    
    if emit_llvm {
        println!("[3/4] Generating LLVM IR...");
        let llvm_ir = llvm_codegen::generate_llvm(&ast);
        println!("      {} bytes of LLVM IR", llvm_ir.len());
        
        println!("[4/4] Writing {}...", output_path);
        fs::write(&output_path, &llvm_ir).expect("Failed to write LLVM IR");
        println!("Done! Compile with: clang {} -o output", output_path);
        
        // Optionally compile to native using clang/llc
        if output_path.ends_with(".ll") {
            let native_path = output_path.replace(".ll", "");
            let clang_result = Command::new("clang")
                .args([&output_path, "-o", &native_path])
                .output();
            
            if let Ok(output) = clang_result {
                if output.status.success() {
                    println!("      Also compiled to native: {}", native_path);
                }
            }
        }
    } else {
        println!("[3/4] Generating ARM64 code...");
        let code = codegen::generate(&ast);
        println!("      {} bytes of machine code", code.len());
        
        println!("[4/4] Writing {}...", output_path);
        binary::write_macho(&output_path, &code);
        
        println!("Done! Run with: ./{}", output_path);
    }
}
