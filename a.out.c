/* Aether Compiler v3.0 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sched.h>
#include <sys/syscall.h>

#define print(x) printf("%lld\n",(long long)(x))
#define println(s) printf("%s\n",(s))
#define __builtin_malloc(n) ((long long)malloc(n))
#define __builtin_free(p) free((void*)(p))
#define __builtin_store64(a,v) (*(long long*)(a)=(v))
#define __builtin_load64(a) (*(long long*)(a))
#define __builtin_store8(a,v) (*(char*)(a)=(char)(v))
#define __builtin_load8(a) ((unsigned char)*(char*)(a))
#define __builtin_panic(x) (printf("PANIC: %lld\n",(long long)(x)),exit((int)(x)),0)
#define __builtin_call(f,x) ((long long (*)(long long))(f))(x)
#define __builtin_thread_id() ((long long)pthread_self())
#define __builtin_atomic_cas(ptr,old,new) __sync_bool_compare_and_swap((long long*)(ptr),(long long)(old),(long long)(new))
#define __builtin_atomic_store(ptr,val) __sync_lock_test_and_set((long long*)(ptr),(long long)(val))
#define __builtin_yield() sched_yield()
#define __builtin_syscall2(n,a1,a2) syscall((long)(n),(long)(a1),(long)(a2))
#define __memory_barrier() __sync_synchronize()
#define __builtin_or(a,b) ((a)|(b))
#define __builtin_and(a,b) ((a)&(b))
#define __builtin_xor(a,b) ((a)^(b))

typedef struct {
    long long x;
} Res;

void Res_drop(Res r) {
    (print((999)));
}

long long main(void) {
    if (1) {
    Res r = (((Res){.x=(1)}));
    Res_drop(r);
    }
    return (0);
}

