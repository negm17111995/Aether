import subprocess
import os
import re
import sys

# Paths
ROOT = os.path.dirname(os.path.abspath(__file__))
BOOTSTRAP = os.path.join(ROOT, "../bootstrap")
COMPILER = os.path.join(BOOTSTRAP, "aetherc_native")

LANGS = {
    "C": {"src": ".c", "cmd": ["clang", "-O3", "-o", "{exe}", "{src}"]},
    "C++": {"src": ".cpp", "cmd": ["clang++", "-O3", "-o", "{exe}", "{src}"]},
    "Rust": {"src": ".rs", "cmd": ["rustc", "-C", "opt-level=3", "-o", "{exe}", "{src}"]},
    "Go": {"src": ".go", "cmd": ["go", "build", "-o", "{exe}", "{src}"]},
    "Aether": {"src": ".aether", "cmd": [COMPILER, "{src}", "-o", "{exe}"]}
}

TESTS = ["tak", "loop_sum"]

def run_cmd(cmd):
    try:
        subprocess.check_call(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        return True
    except:
        return False

def parse_time_output(output):
    # BSD time -l output format
    stats = {}
    
    # Simple regex for Time
    # "        0.30 real         0.29 user         0.00 sys"
    m_time = re.search(r'([\d\.]+) real\s+([\d\.]+) user\s+([\d\.]+) sys', output)
    if m_time:
        stats['real'] = float(m_time.group(1))
        stats['user'] = float(m_time.group(2))
        stats['sys'] = float(m_time.group(3))
    
    # Memory: "maximum resident set size"
    m_mem = re.search(r'(\d+)\s+maximum resident set size', output)
    if m_mem:
        stats['rss_bytes'] = int(m_mem.group(1))
        
    return stats

def main():
    # 1. Compile Compiler
    print("Building Aether Compiler...")
    subprocess.check_call(["clang", "-O3", "-o", "aetherc_native", "native_compiler.c"], cwd=BOOTSTRAP)
    
    results = {} # Test -> Lang -> Stats
    
    print(f"{'Test':<10} {'Lang':<10} {'Time(s)':<10} {'Diff':<10} {'RSS(MB)':<10} {'CPU Comp'}")
    print("-" * 70)
    
    for test in TESTS:
        results[test] = {}
        baseline_time = 0
        
        for lang, config in LANGS.items():
            src_file = os.path.join(ROOT, f"{test}{config['src']}")
            exe_file = os.path.join(ROOT, f"{test}_{lang.lower()}")
            
            # Remove old exe
            if os.path.exists(exe_file):
                os.remove(exe_file)
                
            # Compile
            cmd = [arg.format(exe=exe_file, src=src_file) for arg in config['cmd']]
            if not run_cmd(cmd):
                print(f"Failed to compile {lang} {test}")
                continue
                
            # Run with /usr/bin/time -l
            try:
                # Capture standard error where time outputs
                proc = subprocess.run(['/usr/bin/time', '-l', exe_file], capture_output=True, text=True)
                if proc.returncode != 0 and proc.returncode != 203 and proc.returncode != 256: 
                    # Some return codes are expected (Fib returns 203)
                    pass
                    
                stats = parse_time_output(proc.stderr)
                time_sec = stats.get('real', 0)
                mem_mb = stats.get('rss_bytes', 0) / (1024 * 1024)
                
                if lang == "C": # Baseline
                    baseline_time = time_sec
                
                diff = ""
                if baseline_time > 0 and time_sec > 0:
                    ratio = time_sec / baseline_time
                    diff = f"{ratio:.2f}x"
                
                # CPU Complexity/Efficiency hint? Just use user/real
                
                print(f"{test:<10} {lang:<10} {time_sec:<10.3f} {diff:<10} {mem_mb:<10.2f}")
                results[test][lang] = stats
                
            except Exception as e:
                print(f"Error running {lang}: {e}")
                
        print("-" * 70)

if __name__ == "__main__":
    main()
