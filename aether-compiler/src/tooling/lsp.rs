//! Aether Language Server Protocol (LSP) Implementation
//! Provides IDE features like diagnostics and completion

use std::io::{self, BufRead, Read, Write};
use std::collections::HashMap;

/// LSP Server
pub struct LspServer {
    documents: HashMap<String, String>,
}

impl LspServer {
    pub fn new() -> Self {
        LspServer {
            documents: HashMap::new(),
        }
    }
    
    /// Start the LSP main loop over stdio
    pub fn start(&mut self) {
        eprintln!("Aether LSP started...");
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        
        for line in stdin.lock().lines() {
            let line = line.unwrap_or_default();
            if line.starts_with("Content-Length:") {
                // Read header
                let len: usize = line
                    .trim_start_matches("Content-Length: ")
                    .trim()
                    .parse()
                    .unwrap_or(0);
                    
                // Skip empty line
                let mut empty = String::new();
                stdin.lock().read_line(&mut empty).unwrap();
                
                // Read body
                let mut body = vec![0; len];
                stdin.lock().read_exact(&mut body).unwrap();
                let body_str = String::from_utf8(body).unwrap_or_default();
                
                self.handle_message(&body_str);
            }
        }
    }
    
    fn handle_message(&mut self, msg: &str) {
        // Very basic JSON handling (since we don't have serde yet in manual mode)
        // In real impl, use serde_json
        
        if msg.contains("initialize") {
            self.send_response(r#"{
                "jsonrpc": "2.0",
                "result": {
                    "capabilities": {
                        "textDocumentSync": 1,
                        "completionProvider": {}
                    }
                }
            }"#);
        } else if msg.contains("textDocument/didOpen") {
            // Handle open
        } else if msg.contains("textDocument/didChange") {
            // Check for errors
            // Use existing parser/lexer
        }
    }
    
    fn send_response(&self, content: &str) {
        let len = content.len();
        print!("Content-Length: {}\r\n\r\n{}", len, content);
        io::stdout().flush().unwrap();
    }
}
