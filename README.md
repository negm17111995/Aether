# Aether Programming Language

**The world's first pure Aether self-hosted compiler - no external dependencies required.**

## Quick Start

```bash
# Compile and run an Aether program
./aetherc_native your_program.aether -o your_program
./your_program
```

## Project Structure

```
aether-core/
├── bin/                    # Compiled binaries
│   └── aetherc             # Pure Aether compiler
├── compiler/               # Self-hosted compiler source (Pure Aether)
│   ├── lexer.aether        # Tokenizer
│   ├── parser.aether       # Parser
│   ├── typechecker.aether  # Type checker
│   ├── codegen/            # Code generation
│   └── main.aether         # Compiler entry point
├── runtime/                # Runtime library (Pure Aether)
│   ├── core.aether         # Core functions
│   ├── vec.aether          # Dynamic arrays
│   ├── map.aether          # Hash maps
│   ├── str.aether          # String operations
│   ├── http.aether         # HTTP client
│   ├── tls.aether          # TLS 1.2 encryption
│   ├── dns.aether          # DNS resolution
│   ├── net.aether          # TCP/UDP sockets
│   └── crypto/             # Cryptography
│       ├── sha256.aether   # SHA-256 hashing
│       └── aes_gcm.aether  # AES-GCM encryption
├── stdlib/                 # Standard library (Pure Aether)
│   ├── firebase/           # Firebase integration
│   │   ├── firebase.aether # Core Firebase SDK
│   │   ├── dataconnect.aether # GraphQL Data Connect
│   │   └── app_hosting.aether # App Hosting API
│   ├── database/           # Database drivers
│   │   └── postgres.aether # PostgreSQL wire protocol
│   ├── cloud/              # Cloud providers
│   │   ├── cloudsql.aether # Google Cloud SQL
│   │   └── cloudrun.aether # Google Cloud Run
│   └── std/                # Standard utilities
├── docs/                   # Documentation
├── examples/               # Example programs
└── tests/                  # Test suite
```

## Self-Hosting

Aether is fully self-hosted. The compiler is written in Aether and compiles itself:

```bash
# The pure Aether compiler compiling itself
./aetherc_native compiler/main.aether -o bin/aetherc_new

# Use the new compiler
./bin/aetherc_new your_program.aether
```

## Platform Support

| Platform | Status |
|----------|--------|
| macOS ARM64 (Apple Silicon) | ✅ Fully supported |
| macOS x64 | ✅ Supported |
| Linux ARM64 | ✅ Supported |
| Linux x64 | ✅ Supported |

## Features

### Pure Aether Networking
- **TCP/UDP Sockets** - Direct syscall implementation
- **HTTP Client** - Full HTTP/1.1 support
- **TLS 1.2** - Secure HTTPS connections
- **DNS Resolution** - Hostname to IP lookup

### Pure Aether Cryptography
- **SHA-256** - Cryptographic hashing
- **AES-GCM** - Authenticated encryption

### Cloud Integration
- **Firebase** - Auth, Realtime DB, Data Connect, App Hosting
- **PostgreSQL** - Real wire protocol implementation
- **Google Cloud** - Cloud SQL, Cloud Run

## Build from Source

```bash
# One-time bootstrap (uses Rust to create initial compiler)
cd aether-compiler && cargo build --release
./target/release/aether-compiler ../compiler/main.aether -o ../aetherc_native

# From now on, use pure Aether
cd ..
./aetherc_native compiler/main.aether -o bin/aetherc
```

## License

Apache 2.0 - See LICENSE file.
