// C++ Extreme Stress Test
#include <iostream>
#include <vector>
#include <cstring>

#define ALLOC_SIZE 1048576  // 1MB

long test_allocations(int count) {
    std::vector<char*> ptrs;
    ptrs.reserve(count);
    long success = 0;
    
    try {
        for (int i = 0; i < count; i++) {
            char* ptr = new char[ALLOC_SIZE];
            memset(ptr, i & 0xFF, ALLOC_SIZE);
            ptrs.push_back(ptr);
            success++;
        }
    } catch (std::bad_alloc& e) {
        std::cout << "  FAILED at allocation #" << success << ": " << e.what() << std::endl;
    }
    
    // Cleanup
    for (auto ptr : ptrs) {
        delete[] ptr;
    }
    
    return success;
}

int main(int argc, char** argv) {
    int target = 1000;
    if (argc > 1) target = atoi(argv[1]);
    
    std::cout << "C++ Stress Test: " << target << " x 1MB allocations" << std::endl;
    
    long success = test_allocations(target);
    
    std::cout << "Result: " << success << "/" << target << " allocations succeeded" << std::endl;
    std::cout << "Memory used: " << success << " MB" << std::endl;
    
    if (success < target) {
        std::cout << "STATUS: FAILED at " << success << std::endl;
        return 1;
    }
    std::cout << "STATUS: PASSED" << std::endl;
    return 0;
}
