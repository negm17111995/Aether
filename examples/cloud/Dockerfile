# Aether Cloud Container
# Runs on Any Cloud (AWS, GCP, Azure, DigitalOcean)

FROM ubuntu:22.04

# Install dependencies (World-Class C Interop)
RUN apt-get update && apt-get install -y \
    clang \
    libpq-dev \
    libcurl4-openssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Aether Compiler
COPY bin/aetherc /usr/local/bin/aetherc

# Copy Application
WORKDIR /app
COPY examples/cloud/app.aether .
COPY stdlib /app/stdlib

# Compile (Production Mode)
# This uses the LLVM backend (default) to link against libpq and libcurl
RUN aetherc build app.aether -o app

# Run
CMD ["./app"]
