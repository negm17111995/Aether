// AETHER BOOTSTRAP COMPILER - STAGE 0
// Cross-platform compiler: Aether source → Native ARM64 or LLVM IR
// Supports: macOS, Linux, Windows on x86_64 and ARM64

mod lexer;
mod parser;
mod codegen;
mod llvm_codegen;
mod binary;

use std::env;
use std::fs;
use std::process;
use std::process::Command;

use llvm_codegen::Target;

fn print_usage() {
    eprintln!("Aether Bootstrap Compiler (Stage 0)");
    eprintln!("");
    eprintln!("Usage: aether_bootstrap [OPTIONS] <source.aether> [output]");
    eprintln!("");
    eprintln!("Options:");
    eprintln!("  --emit-llvm         Output LLVM IR instead of native binary");
    eprintln!("  --target <TARGET>   Cross-compile for specific target");
    eprintln!("  --list-targets      List all supported targets");
    eprintln!("  --version           Show version information");
    eprintln!("  --help              Show this help message");
    eprintln!("");
    eprintln!("Supported Targets:");
    eprintln!("  macos-arm64     macOS ARM64 (Apple Silicon)");
    eprintln!("  macos-x64       macOS x86_64 (Intel)");
    eprintln!("  linux-arm64     Linux ARM64");
    eprintln!("  linux-x64       Linux x86_64");
    eprintln!("  windows-x64     Windows x86_64");
    eprintln!("  wasm32          WebAssembly");
    eprintln!("");
    eprintln!("Examples:");
    eprintln!("  aether_bootstrap hello.aether hello                    # Native");
    eprintln!("  aether_bootstrap --emit-llvm hello.aether hello.ll     # LLVM IR");
    eprintln!("  aether_bootstrap --emit-llvm --target linux-x64 hello.aether hello.ll");
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
        println!("Host: {:?}", Target::detect());
        println!("Targets: macos-arm64, macos-x64, linux-arm64, linux-x64, windows-x64, wasm32");
        println!("Backends: Native ARM64, LLVM IR");
        return;
    }
    
    if args[1] == "--help" || args[1] == "-h" {
        print_usage();
        return;
    }
    
    if args[1] == "--list-targets" {
        println!("Supported targets:");
        for target in Target::all() {
            println!("  {:?} -> {}", target, target.triple());
        }
        return;
    }
    
    // Parse options
    let mut emit_llvm = false;
    let mut target: Option<Target> = None;
    let mut arg_idx = 1;
    
    while arg_idx < args.len() && args[arg_idx].starts_with("--") {
        match args[arg_idx].as_str() {
            "--emit-llvm" => {
                emit_llvm = true;
                arg_idx += 1;
            }
            "--target" => {
                if arg_idx + 1 >= args.len() {
                    eprintln!("Error: --target requires a target name");
                    process::exit(1);
                }
                target = Target::from_str(&args[arg_idx + 1]);
                if target.is_none() {
                    eprintln!("Error: Unknown target '{}'. Use --list-targets to see options.", args[arg_idx + 1]);
                    process::exit(1);
                }
                arg_idx += 2;
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", args[arg_idx]);
                process::exit(1);
            }
        }
    }
    
    if arg_idx >= args.len() {
        eprintln!("Error: No source file specified");
        process::exit(1);
    }
    
    let source_path = &args[arg_idx];
    let output_path = if args.len() > arg_idx + 1 { 
        args[arg_idx + 1].clone() 
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
        let actual_target = target.unwrap_or_else(Target::detect);
        println!("[3/4] Generating LLVM IR for {}...", actual_target.triple());
        let llvm_ir = llvm_codegen::generate_llvm_for_target(&ast, actual_target);
        println!("      {} bytes of LLVM IR", llvm_ir.len());
        
        println!("[4/4] Writing {}...", output_path);
        fs::write(&output_path, &llvm_ir).expect("Failed to write LLVM IR");
        println!("Done!");
        println!("");
        println!("To compile LLVM IR to native:");
        println!("  clang {} -o output", output_path);
        println!("  clang {} -o output --target={}", output_path, actual_target.triple());
        
        // Auto-compile if clang available
        if output_path.ends_with(".ll") {
            let native_path = output_path.replace(".ll", "");
            let clang_result = Command::new("clang")
                .args([&output_path, "-o", &native_path])
                .output();
            
            if let Ok(output) = clang_result {
                if output.status.success() {
                    println!("      Auto-compiled to: {}", native_path);
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
