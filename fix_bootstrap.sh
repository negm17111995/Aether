#!/bin/bash
# Aether Bootstrap Compatibility Fixer
# Converts stdlib files to be compatible with Stage 1 bootstrap compiler

echo "═══════════════════════════════════════════════════════════════"
echo "AETHER BOOTSTRAP FIXER"
echo "═══════════════════════════════════════════════════════════════"

# Backup original stdlib
if [ ! -d "stdlib_full" ]; then
    echo "Backing up full stdlib to stdlib_full..."
    cp -r stdlib stdlib_full
fi

# Function to fix a single file
fix_file() {
    local file="$1"
    local temp_file="${file}.tmp"
    
    # Skip already processed or special files
    if [[ "$file" == *"_bootstrap"* ]]; then
        return
    fi
    
    # Remove generic declarations: func name<T>(...) -> func name(...)
    # Replace <T> with empty, Option<T> with Int, Result<T,E> with Int, [T] with Int
    sed -E '
        # Remove attributes like @inline, @comptime, @extern(...)
        s/@[a-z_]+(\([^)]*\))?//g
        
        # Replace generic func declarations: func name<T> -> func name
        s/func ([a-zA-Z_][a-zA-Z0-9_]*)<[^>]+>/func \1/g
        
        # Replace Option<T> with Int
        s/Option<[^>]+>/Int/g
        
        # Replace Result<T, E> or Result<T,E> with Int
        s/Result<[^>]+>/Int/g
        
        # Replace Vec<T> with Int
        s/Vec<[^>]+>/Int/g
        
        # Replace HashMap<K,V> with Int
        s/HashMap<[^>]+>/Int/g
        
        # Replace [T] with Int (generic array)
        s/\[[A-Z]\]/Int/g
        
        # Replace struct<T> declarations
        s/struct ([a-zA-Z_]+)<[^>]+>/struct \1/g
        
        # Replace enum<T> declarations  
        s/enum ([a-zA-Z_]+)<[^>]+>/enum \1/g
        
        # Replace Option::Some(x) with x
        s/Option::Some\(([^)]+)\)/\1/g
        
        # Replace Option::None with 0
        s/Option::None/0/g
        
        # Replace Result::Ok(x) with x
        s/Result::Ok\(([^)]+)\)/\1/g
        
        # Replace Result::Err(x) with 0
        s/Result::Err\([^)]+\)/0/g
        
    ' "$file" > "$temp_file"
    
    mv "$temp_file" "$file"
}

# Process all .aether files
count=0
for file in $(find stdlib -name "*.aether" -type f); do
    echo "Fixing: $file"
    fix_file "$file"
    count=$((count + 1))
done

echo ""
echo "Fixed $count files"
echo "Running verification..."
./verify_all.sh
