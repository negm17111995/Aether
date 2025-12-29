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

// importing: stdlib/std.aether
long long ae_abs(long long x) {
    if (x < 0) {
    return (0 - x);
    }
    return (x);
}

long long ae_min(long long a, long long b) {
    if (a < b) {
    return (a);
    }
    return (b);
}

long long ae_max(long long a, long long b) {
    if (a > b) {
    return (a);
    }
    return (b);
}

long long ae_clamp(long long x, long long lo, long long hi) {
    return (ae_min((ae_max((x),(lo))),(hi)));
}

long long ae_malloc(long long size) {
    return (__builtin_malloc((size)));
}

void ae_store64(long long addr, long long val) {
    (__builtin_store64((addr),(val)));
}

long long ae_load64(long long addr) {
    return (__builtin_load64((addr)));
}

void ae_store8(long long addr, long long val) {
    (__builtin_store8((addr),(val)));
}

long long ae_load8(long long addr) {
    return (__builtin_load8((addr)));
}

long long vec_new(void) {
    long long v = (ae_malloc((24)));
    long long data = (ae_malloc((1024)));
    (ae_store64((v),(data)));
    (ae_store64((v + 8),(0)));
    (ae_store64((v + 16),(1024)));
    return (v);
}

long long vec_len(long long v) {
    return (ae_load64((v + 8)));
}

void vec_push(long long v, long long val) {
    long long len = (ae_load64((v + 8)));
    long long data = (ae_load64((v)));
    (ae_store64((data + len * 8),(val)));
    (ae_store64((v + 8),(len + 1)));
}

long long vec_get(long long v, long long i) {
    return (ae_load64((ae_load64((v)) + i * 8)));
}

void vec_set(long long v, long long i, long long val) {
    (ae_store64((ae_load64((v)) + i * 8),(val)));
}

void assert(int cond) {
    if (!cond) {
    (print((0 - 1)));
    }
}

void assert_eq(long long a, long long b) {
    if (a != b) {
    (print((a)));
    (print((b)));
    (print((0 - 1)));
    }
}

#define AST_INT_LIT (1)
#define AST_STR_LIT (2)
#define AST_IDENT (3)
#define AST_BINARY (4)
#define AST_UNARY (5)
#define AST_CALL (6)
#define AST_INDEX (7)
#define AST_FIELD (8)
#define AST_STRUCT_LIT (9)
#define AST_ARRAY_LIT (10)
#define AST_LAMBDA (11)
#define AST_LET (20)
#define AST_ASSIGN (21)
#define AST_RETURN (22)
#define AST_IF (23)
#define AST_WHILE (24)
#define AST_FOR (25)
#define AST_MATCH (26)
#define AST_EXPR_STMT (27)
#define AST_BLOCK (28)
#define AST_FUNC (40)
#define AST_STRUCT (41)
#define AST_TRAIT (42)
#define AST_IMPL (43)
#define AST_CONST (44)
#define AST_IMPORT (45)
#define AST_MODULE (46)
#define AST_TYPE_NAME (60)
#define AST_TYPE_GENERIC (61)
#define AST_TYPE_FUNC (62)
#define AST_TYPE_ARRAY (63)
#define NODE_SIZE (56)
long long ast_new(long long kind, long long line, long long col) {
    long long n = (ae_malloc((NODE_SIZE)));
    (ae_store64((n),(kind)));
    (ae_store64((n + 8),(line)));
    (ae_store64((n + 16),(col)));
    (ae_store64((n + 24),(0)));
    (ae_store64((n + 32),(0)));
    (ae_store64((n + 40),(0)));
    (ae_store64((n + 48),(0)));
    return (n);
}

long long ast_kind(long long n) {
    return (ae_load64((n)));
}

long long ast_line(long long n) {
    return (ae_load64((n + 8)));
}

long long ast_col(long long n) {
    return (ae_load64((n + 16)));
}

long long ast_data1(long long n) {
    return (ae_load64((n + 24)));
}

long long ast_data2(long long n) {
    return (ae_load64((n + 32)));
}

long long ast_data3(long long n) {
    return (ae_load64((n + 40)));
}

long long ast_data4(long long n) {
    return (ae_load64((n + 48)));
}

void ast_set_data1(long long n, long long v) {
    (ae_store64((n + 24),(v)));
}

void ast_set_data2(long long n, long long v) {
    (ae_store64((n + 32),(v)));
}

void ast_set_data3(long long n, long long v) {
    (ae_store64((n + 40),(v)));
}

void ast_set_data4(long long n, long long v) {
    (ae_store64((n + 48),(v)));
}

long long ast_int(long long val, long long line, long long col) {
    long long n = (ast_new((AST_INT_LIT),(line),(col)));
    (ast_set_data1((n),(val)));
    return (n);
}

long long ast_ident(long long name, long long line, long long col) {
    long long n = (ast_new((AST_IDENT),(line),(col)));
    (ast_set_data1((n),(name)));
    return (n);
}

long long ast_binary(long long op, long long left, long long right, long long line, long long col) {
    long long n = (ast_new((AST_BINARY),(line),(col)));
    (ast_set_data1((n),(op)));
    (ast_set_data2((n),(left)));
    (ast_set_data3((n),(right)));
    return (n);
}

long long ast_unary(long long op, long long operand, long long line, long long col) {
    long long n = (ast_new((AST_UNARY),(line),(col)));
    (ast_set_data1((n),(op)));
    (ast_set_data2((n),(operand)));
    return (n);
}

long long ast_call(long long func_expr, long long args, long long line, long long col) {
    long long n = (ast_new((AST_CALL),(line),(col)));
    (ast_set_data1((n),(func_expr)));
    (ast_set_data2((n),(args)));
    return (n);
}

long long ast_field(long long expr, long long field, long long line, long long col) {
    long long n = (ast_new((AST_FIELD),(line),(col)));
    (ast_set_data1((n),(expr)));
    (ast_set_data2((n),(field)));
    return (n);
}

long long ast_let(long long name, long long typ, long long init, long long is_mut, long long line, long long col) {
    long long n = (ast_new((AST_LET),(line),(col)));
    (ast_set_data1((n),(name)));
    (ast_set_data2((n),(typ)));
    (ast_set_data3((n),(init)));
    (ast_set_data4((n),(is_mut)));
    return (n);
}

long long ast_return(long long expr, long long line, long long col) {
    long long n = (ast_new((AST_RETURN),(line),(col)));
    (ast_set_data1((n),(expr)));
    return (n);
}

long long ast_if(long long cond, long long then_blk, long long else_blk, long long line, long long col) {
    long long n = (ast_new((AST_IF),(line),(col)));
    (ast_set_data1((n),(cond)));
    (ast_set_data2((n),(then_blk)));
    (ast_set_data3((n),(else_blk)));
    return (n);
}

long long ast_while(long long cond, long long body, long long line, long long col) {
    long long n = (ast_new((AST_WHILE),(line),(col)));
    (ast_set_data1((n),(cond)));
    (ast_set_data2((n),(body)));
    return (n);
}

long long ast_block(long long stmts, long long line, long long col) {
    long long n = (ast_new((AST_BLOCK),(line),(col)));
    (ast_set_data1((n),(stmts)));
    return (n);
}

long long ast_func(long long name, long long params, long long ret_type, long long body, long long line, long long col) {
    long long n = (ast_new((AST_FUNC),(line),(col)));
    (ast_set_data1((n),(name)));
    (ast_set_data2((n),(params)));
    (ast_set_data3((n),(ret_type)));
    (ast_set_data4((n),(body)));
    return (n);
}

long long ast_struct(long long name, long long fields, long long generics, long long line, long long col) {
    long long n = (ast_new((AST_STRUCT),(line),(col)));
    (ast_set_data1((n),(name)));
    (ast_set_data2((n),(fields)));
    (ast_set_data3((n),(generics)));
    return (n);
}

long long ast_trait(long long name, long long methods, long long generics, long long line, long long col) {
    long long n = (ast_new((AST_TRAIT),(line),(col)));
    (ast_set_data1((n),(name)));
    (ast_set_data2((n),(methods)));
    (ast_set_data3((n),(generics)));
    return (n);
}

long long ast_impl(long long trait_name, long long for_type, long long methods, long long line, long long col) {
    long long n = (ast_new((AST_IMPL),(line),(col)));
    (ast_set_data1((n),(trait_name)));
    (ast_set_data2((n),(for_type)));
    (ast_set_data3((n),(methods)));
    return (n);
}

long long ast_import(long long path, long long line, long long col) {
    long long n = (ast_new((AST_IMPORT),(line),(col)));
    (ast_set_data1((n),(path)));
    return (n);
}

long long ast_const(long long name, long long typ, long long val, long long line, long long col) {
    long long n = (ast_new((AST_CONST),(line),(col)));
    (ast_set_data1((n),(name)));
    (ast_set_data2((n),(typ)));
    (ast_set_data3((n),(val)));
    return (n);
}

long long ast_module(long long decls) {
    long long n = (ast_new((AST_MODULE),(0),(0)));
    (ast_set_data1((n),(decls)));
    return (n);
}

// already imported: stdlib/std.aether
#define TOK_EOF (0)
#define TOK_ID (1)
#define TOK_INT (2)
#define TOK_STR (3)
#define TOK_LPAREN (10)
#define TOK_RPAREN (11)
#define TOK_LBRACE (12)
#define TOK_RBRACE (13)
#define TOK_LBRACK (14)
#define TOK_RBRACK (15)
#define TOK_COMMA (16)
#define TOK_COLON (17)
#define TOK_SEMI (18)
#define TOK_DOT (19)
#define TOK_ARROW (20)
#define TOK_EQ (21)
#define TOK_EQEQ (22)
#define TOK_NE (23)
#define TOK_LT (24)
#define TOK_LE (25)
#define TOK_GT (26)
#define TOK_GE (27)
#define TOK_PLUS (28)
#define TOK_MINUS (29)
#define TOK_STAR (30)
#define TOK_SLASH (31)
#define TOK_AMP (32)
#define TOK_PIPE (33)
#define TOK_BANG (34)
#define TOK_DCOLON (35)
#define TOK_FUNC (50)
#define TOK_LET (51)
#define TOK_MUT (52)
#define TOK_IF (53)
#define TOK_ELSE (54)
#define TOK_WHILE (55)
#define TOK_FOR (56)
#define TOK_RETURN (57)
#define TOK_STRUCT (58)
#define TOK_IMPL (59)
#define TOK_TRAIT (60)
#define TOK_IMPORT (61)
#define TOK_CONST (62)
#define TOK_PUB (63)
#define TOK_MATCH (64)
#define TOK_TRUE (65)
#define TOK_FALSE (66)
#define TOKEN_SIZE (40)
long long token_new(long long typ, long long val, long long line, long long col, long long len) {
    long long t = (ae_malloc((TOKEN_SIZE)));
    (ae_store64((t),(typ)));
    (ae_store64((t + 8),(val)));
    (ae_store64((t + 16),(line)));
    (ae_store64((t + 24),(col)));
    (ae_store64((t + 32),(len)));
    return (t);
}

long long token_type(long long t) {
    return (ae_load64((t)));
}

long long token_value(long long t) {
    return (ae_load64((t + 8)));
}

long long token_line(long long t) {
    return (ae_load64((t + 16)));
}

long long token_col(long long t) {
    return (ae_load64((t + 24)));
}

#define LEXER_SIZE (40)
long long lexer_new(long long src, long long len) {
    long long l = (ae_malloc((LEXER_SIZE)));
    (ae_store64((l),(src)));
    (ae_store64((l + 8),(len)));
    (ae_store64((l + 16),(0)));
    (ae_store64((l + 24),(1)));
    (ae_store64((l + 32),(1)));
    return (l);
}

long long lexer_src(long long l) {
    return (ae_load64((l)));
}

long long lexer_len(long long l) {
    return (ae_load64((l + 8)));
}

long long lexer_pos(long long l) {
    return (ae_load64((l + 16)));
}

long long lexer_line(long long l) {
    return (ae_load64((l + 24)));
}

long long lexer_col(long long l) {
    return (ae_load64((l + 32)));
}

void lexer_set_pos(long long l, long long p) {
    (ae_store64((l + 16),(p)));
}

void lexer_set_line(long long l, long long ln) {
    (ae_store64((l + 24),(ln)));
}

void lexer_set_col(long long l, long long c) {
    (ae_store64((l + 32),(c)));
}

long long is_digit(long long c) {
    if (c >= 48 && c <= 57) {
    return (1);
    }
    return (0);
}

long long is_alpha(long long c) {
    if (c >= 65 && c <= 90) {
    return (1);
    }
    if (c >= 97 && c <= 122) {
    return (1);
    }
    if (c == 95) {
    return (1);
    }
    return (0);
}

long long is_alnum(long long c) {
    if (is_alpha((c)) == 1) {
    return (1);
    }
    if (is_digit((c)) == 1) {
    return (1);
    }
    return (0);
}

long long is_space(long long c) {
    if (c == 32) {
    return (1);
    }
    if (c == 9) {
    return (1);
    }
    if (c == 10) {
    return (1);
    }
    if (c == 13) {
    return (1);
    }
    return (0);
}

long long lexer_peek(long long l) {
    long long pos = (lexer_pos((l)));
    long long len = (lexer_len((l)));
    if (pos >= len) {
    return (0);
    }
    return (ae_load8((lexer_src((l)) + pos)));
}

long long lexer_peek_n(long long l, long long n) {
    long long pos = (lexer_pos((l)) + n);
    long long len = (lexer_len((l)));
    if (pos >= len) {
    return (0);
    }
    return (ae_load8((lexer_src((l)) + pos)));
}

void lexer_advance(long long l) {
    long long c = (lexer_peek((l)));
    long long pos = (lexer_pos((l)));
    (lexer_set_pos((l),(pos + 1)));
    if (c == 10) {
    (lexer_set_line((l),(lexer_line((l)) + 1)));
    (lexer_set_col((l),(1)));
    } else {
    (lexer_set_col((l),(lexer_col((l)) + 1)));
    }
}

void lexer_skip_ws(long long l) {
    while (is_space((lexer_peek((l)))) == 1) {
    (lexer_advance((l)));
    }
    if (lexer_peek((l)) == 47 && lexer_peek_n((l),(1)) == 47) {
    while (lexer_peek((l)) != 10 && lexer_peek((l)) != 0) {
    (lexer_advance((l)));
    }
    (lexer_skip_ws((l)));
    }
}

long long lexer_read_number(long long l) {
    long long start = (lexer_pos((l)));
    long long val = (0);
    while (is_digit((lexer_peek((l)))) == 1) {
    (val) = (val * 10 + ((lexer_peek((l)) - 48)));
    (lexer_advance((l)));
    }
    return (val);
}

long long lexer_read_ident(long long l) {
    long long start = (lexer_pos((l)));
    long long src = (lexer_src((l)));
    while (is_alnum((lexer_peek((l)))) == 1) {
    (lexer_advance((l)));
    }
    long long len = (lexer_pos((l)) - start);
    long long buf = (ae_malloc((len + 1)));
    long long i = (0);
    while (i < len) {
    (ae_store8((buf + i),(ae_load8((src + start + i)))));
    (i) = (i + 1);
    }
    (ae_store8((buf + len),(0)));
    return (buf);
}

long long lexer_next(long long l) {
    (lexer_skip_ws((l)));
    long long line = (lexer_line((l)));
    long long col = (lexer_col((l)));
    long long c = (lexer_peek((l)));
    if (c == 0) {
    return (token_new((TOK_EOF),(0),(line),(col),(0)));
    }
    if (is_digit((c)) == 1) {
    long long val = (lexer_read_number((l)));
    return (token_new((TOK_INT),(val),(line),(col),(1)));
    }
    if (is_alpha((c)) == 1) {
    long long id = (lexer_read_ident((l)));
    return (token_new((TOK_ID),(id),(line),(col),(1)));
    }
    long long c2 = (lexer_peek_n((l),(1)));
    if (c == 45 && c2 == 62) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_ARROW),(0),(line),(col),(2)));
    }
    if (c == 61 && c2 == 61) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_EQEQ),(0),(line),(col),(2)));
    }
    if (c == 33 && c2 == 61) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_NE),(0),(line),(col),(2)));
    }
    if (c == 60 && c2 == 61) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_LE),(0),(line),(col),(2)));
    }
    if (c == 62 && c2 == 61) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_GE),(0),(line),(col),(2)));
    }
    if (c == 58 && c2 == 58) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_DCOLON),(0),(line),(col),(2)));
    }
    if (c == 38 && c2 == 38) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_AMP),(0),(line),(col),(2)));
    }
    if (c == 124 && c2 == 124) {
    (lexer_advance((l)));
    (lexer_advance((l)));
    return (token_new((TOK_PIPE),(0),(line),(col),(2)));
    }
    (lexer_advance((l)));
    if (c == 40) {
    return (token_new((TOK_LPAREN),(0),(line),(col),(1)));
    }
    if (c == 41) {
    return (token_new((TOK_RPAREN),(0),(line),(col),(1)));
    }
    if (c == 123) {
    return (token_new((TOK_LBRACE),(0),(line),(col),(1)));
    }
    if (c == 125) {
    return (token_new((TOK_RBRACE),(0),(line),(col),(1)));
    }
    if (c == 91) {
    return (token_new((TOK_LBRACK),(0),(line),(col),(1)));
    }
    if (c == 93) {
    return (token_new((TOK_RBRACK),(0),(line),(col),(1)));
    }
    if (c == 44) {
    return (token_new((TOK_COMMA),(0),(line),(col),(1)));
    }
    if (c == 58) {
    return (token_new((TOK_COLON),(0),(line),(col),(1)));
    }
    if (c == 59) {
    return (token_new((TOK_SEMI),(0),(line),(col),(1)));
    }
    if (c == 46) {
    return (token_new((TOK_DOT),(0),(line),(col),(1)));
    }
    if (c == 61) {
    return (token_new((TOK_EQ),(0),(line),(col),(1)));
    }
    if (c == 60) {
    return (token_new((TOK_LT),(0),(line),(col),(1)));
    }
    if (c == 62) {
    return (token_new((TOK_GT),(0),(line),(col),(1)));
    }
    if (c == 43) {
    return (token_new((TOK_PLUS),(0),(line),(col),(1)));
    }
    if (c == 45) {
    return (token_new((TOK_MINUS),(0),(line),(col),(1)));
    }
    if (c == 42) {
    return (token_new((TOK_STAR),(0),(line),(col),(1)));
    }
    if (c == 47) {
    return (token_new((TOK_SLASH),(0),(line),(col),(1)));
    }
    if (c == 33) {
    return (token_new((TOK_BANG),(0),(line),(col),(1)));
    }
    return (token_new((TOK_EOF),(0),(line),(col),(0)));
}

long long tokenize(long long src, long long len) {
    long long l = (lexer_new((src),(len)));
    long long tokens = (vec_new());
    long long done = (0);
    while (done == 0) {
    long long tok = (lexer_next((l)));
    (vec_push((tokens),(tok)));
    if (token_type((tok)) == TOK_EOF) {
    (done) = (1);
    }
    }
    return (tokens);
}

// already imported: stdlib/std.aether
#define TYPE_INT (1)
#define TYPE_FLOAT (2)
#define TYPE_BOOL (3)
#define TYPE_STRING (4)
#define TYPE_VOID (5)
#define TYPE_STRUCT (6)
#define TYPE_TRAIT (7)
#define TYPE_ARRAY (8)
#define TYPE_FUNC (9)
#define TYPE_GENERIC (10)
#define TYPE_REF (11)
#define TYPE_MUT_REF (12)
#define TYPE_UNKNOWN (99)
#define TYPE_SIZE (32)
long long type_new(long long kind, long long name) {
    long long t = (ae_malloc((TYPE_SIZE)));
    (ae_store64((t),(kind)));
    (ae_store64((t + 8),(name)));
    (ae_store64((t + 16),(0)));
    (ae_store64((t + 24),(0)));
    return (t);
}

long long type_kind(long long t) {
    return (ae_load64((t)));
}

long long type_name(long long t) {
    return (ae_load64((t + 8)));
}

long long type_inner(long long t) {
    return (ae_load64((t + 16)));
}

long long type_generics(long long t) {
    return (ae_load64((t + 24)));
}

void type_set_inner(long long t, long long i) {
    (ae_store64((t + 16),(i)));
}

void type_set_generics(long long t, long long g) {
    (ae_store64((t + 24),(g)));
}

long long type_int(void) {
    return (type_new((TYPE_INT),(0)));
}

long long type_float(void) {
    return (type_new((TYPE_FLOAT),(0)));
}

long long type_bool(void) {
    return (type_new((TYPE_BOOL),(0)));
}

long long type_string(void) {
    return (type_new((TYPE_STRING),(0)));
}

long long type_void(void) {
    return (type_new((TYPE_VOID),(0)));
}

long long type_array(long long inner) {
    long long t = (type_new((TYPE_ARRAY),(0)));
    (type_set_inner((t),(inner)));
    return (t);
}

long long type_ref(long long inner) {
    long long t = (type_new((TYPE_REF),(0)));
    (type_set_inner((t),(inner)));
    return (t);
}

long long type_mut_ref(long long inner) {
    long long t = (type_new((TYPE_MUT_REF),(0)));
    (type_set_inner((t),(inner)));
    return (t);
}

long long type_env_new(long long parent) {
    long long env = (ae_malloc((24)));
    (ae_store64((env),(vec_new())));
    (ae_store64((env + 8),(vec_new())));
    (ae_store64((env + 16),(parent)));
    return (env);
}

long long type_env_vars(long long env) {
    return (ae_load64((env)));
}

long long type_env_types(long long env) {
    return (ae_load64((env + 8)));
}

long long type_env_parent(long long env) {
    return (ae_load64((env + 16)));
}

void type_env_add(long long env, long long name, long long typ) {
    long long binding = (ae_malloc((16)));
    (ae_store64((binding),(name)));
    (ae_store64((binding + 8),(typ)));
    (vec_push((type_env_vars((env))),(binding)));
}

long long type_env_lookup(long long env, long long name) {
    if (env == 0) {
    return (0);
    }
    long long vars = (type_env_vars((env)));
    long long count = (vec_len((vars)));
    long long i = (0);
    while (i < count) {
    long long binding = (vec_get((vars),(i)));
    if (str_eq((ae_load64((binding))),(name)) == 1) {
    return (ae_load64((binding + 8)));
    }
    (i) = (i + 1);
    }
    return (type_env_lookup((type_env_parent((env))),(name)));
}

long long generic_ctx_new(void) {
    return (vec_new());
}

void generic_ctx_add(long long ctx, long long name, long long typ) {
    long long pair = (ae_malloc((16)));
    (ae_store64((pair),(name)));
    (ae_store64((pair + 8),(typ)));
    (vec_push((ctx),(pair)));
}

long long generic_ctx_resolve(long long ctx, long long name) {
    long long count = (vec_len((ctx)));
    long long i = (0);
    while (i < count) {
    long long pair = (vec_get((ctx),(i)));
    if (str_eq((ae_load64((pair))),(name)) == 1) {
    return (ae_load64((pair + 8)));
    }
    (i) = (i + 1);
    }
    return (0);
}

long long type_substitute(long long typ, long long ctx) {
    if (typ == 0) {
    return (0);
    }
    long long kind = (type_kind((typ)));
    if (kind == TYPE_GENERIC) {
    long long resolved = (generic_ctx_resolve((ctx),(type_name((typ)))));
    if (resolved != 0) {
    return (resolved);
    }
    return (typ);
    }
    if (kind == TYPE_ARRAY) {
    long long inner = (type_substitute((type_inner((typ))),(ctx)));
    return (type_array((inner)));
    }
    if (kind == TYPE_REF) {
    long long inner = (type_substitute((type_inner((typ))),(ctx)));
    return (type_ref((inner)));
    }
    return (typ);
}

long long types_compatible(long long a, long long b) {
    if (a == 0 || b == 0) {
    return (1);
    }
    long long ka = (type_kind((a)));
    long long kb = (type_kind((b)));
    if (ka != kb) {
    return (0);
    }
    if (ka == TYPE_ARRAY) {
    return (types_compatible((type_inner((a))),(type_inner((b)))));
    }
    if (ka == TYPE_STRUCT) {
    return (str_eq((type_name((a))),(type_name((b)))));
    }
    return (1);
}

long long infer_type(long long env, long long node) {
    if (node == 0) {
    return (type_void());
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_INT_LIT) {
    return (type_int());
    }
    if (kind == AST_IDENT) {
    long long typ = (type_env_lookup((env),(ast_data1((node)))));
    if (typ != 0) {
    return (typ);
    }
    return (type_int());
    }
    if (kind == AST_BINARY) {
    long long left_t = (infer_type((env),(ast_data2((node)))));
    long long right_t = (infer_type((env),(ast_data3((node)))));
    long long op = (ast_data1((node)));
    if (op == TOK_EQEQ) {
    return (type_bool());
    }
    if (op == TOK_NE) {
    return (type_bool());
    }
    if (op == TOK_LT) {
    return (type_bool());
    }
    if (op == TOK_LE) {
    return (type_bool());
    }
    if (op == TOK_GT) {
    return (type_bool());
    }
    if (op == TOK_GE) {
    return (type_bool());
    }
    if (op == TOK_AMP) {
    return (type_bool());
    }
    if (op == TOK_PIPE) {
    return (type_bool());
    }
    return (left_t);
    }
    if (kind == AST_CALL) {
    long long func_t = (infer_type((env),(ast_data1((node)))));
    if (type_kind((func_t)) == TYPE_FUNC) {
    return (type_inner((func_t)));
    }
    return (type_int());
    }
    if (kind == AST_ARRAY_LIT) {
    long long elems = (ast_data1((node)));
    if (vec_len((elems)) > 0) {
    long long elem_t = (infer_type((env),(vec_get((elems),(0)))));
    return (type_array((elem_t)));
    }
    return (type_array((type_int())));
    }
    if (kind == AST_STRUCT_LIT) {
    long long struct_t = (type_new((TYPE_STRUCT),(ast_data1((node)))));
    return (struct_t);
    }
    return (type_int());
}

long long check_stmt(long long env, long long node) {
    if (node == 0) {
    return (1);
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_LET) {
    long long name = (ast_data1((node)));
    long long init = (ast_data3((node)));
    long long var_type = (ast_data2((node)));
    long long inferred = (infer_type((env),(init)));
    (type_env_add((env),(name),(inferred)));
    return (1);
    }
    if (kind == AST_IF) {
    long long cond_t = (infer_type((env),(ast_data1((node)))));
    (check_block((env),(ast_data2((node)))));
    long long else_blk = (ast_data3((node)));
    if (else_blk != 0) {
    (check_block((env),(else_blk)));
    }
    return (1);
    }
    if (kind == AST_WHILE) {
    (check_block((env),(ast_data2((node)))));
    return (1);
    }
    if (kind == AST_FOR) {
    long long loop_env = (type_env_new((env)));
    (type_env_add((loop_env),(ast_data1((node))),(type_int())));
    (check_block((loop_env),(ast_data4((node)))));
    return (1);
    }
    return (1);
}

long long check_block(long long env, long long block) {
    if (block == 0) {
    return (1);
    }
    long long block_env = (type_env_new((env)));
    long long stmts = (ast_data1((block)));
    if (stmts == 0) {
    return (1);
    }
    long long count = (vec_len((stmts)));
    long long i = (0);
    while (i < count) {
    (check_stmt((block_env),(vec_get((stmts),(i)))));
    (i) = (i + 1);
    }
    return (1);
}

long long check_func(long long env, long long node) {
    long long name = (ast_data1((node)));
    long long params = (ast_data2((node)));
    long long body = (ast_data4((node)));
    long long func_env = (type_env_new((env)));
    long long count = (vec_len((params)));
    long long i = (0);
    while (i < count) {
    long long param = (vec_get((params),(i)));
    long long param_name = (ae_load64((param)));
    (type_env_add((func_env),(param_name),(type_int())));
    (i) = (i + 1);
    }
    (check_block((func_env),(body)));
    return (1);
}

long long check_module(long long node) {
    long long env = (type_env_new((0)));
    long long decls = (ast_data1((node)));
    long long count = (vec_len((decls)));
    long long i = (0);
    while (i < count) {
    long long decl = (vec_get((decls),(i)));
    long long kind = (ast_kind((decl)));
    if (kind == AST_FUNC) {
    long long func_type = (type_new((TYPE_FUNC),(ast_data1((decl)))));
    (type_set_inner((func_type),(type_int())));
    (type_env_add((env),(ast_data1((decl))),(func_type)));
    }
    (i) = (i + 1);
    }
    (i) = (0);
    while (i < count) {
    long long decl = (vec_get((decls),(i)));
    if (ast_kind((decl)) == AST_FUNC) {
    (check_func((env),(decl)));
    }
    (i) = (i + 1);
    }
    return (1);
}

long long str_eq(long long a, long long b) {
    if (a == 0 || b == 0) {
    if (a == b) {
    return (1);
    }
    return (0);
    }
    long long i = (0);
    while (1 == 1) {
    long long ca = (ae_load8((a + i)));
    long long cb = (ae_load8((b + i)));
    if (ca != cb) {
    return (0);
    }
    if (ca == 0) {
    return (1);
    }
    (i) = (i + 1);
    }
    return (0);
}

// already imported: stdlib/std.aether
#define OWN_OWNED (1)
#define OWN_BORROWED (2)
#define OWN_MUT_BORROWED (3)
#define OWN_MOVED (4)
#define OWN_PARTIAL (5)
#define BORROW_SIZE (32)
long long borrow_new(long long target, long long kind, long long start) {
    long long b = (ae_malloc((BORROW_SIZE)));
    (ae_store64((b),(target)));
    (ae_store64((b + 8),(kind)));
    (ae_store64((b + 16),(start)));
    (ae_store64((b + 24),(0)));
    return (b);
}

long long borrow_target(long long b) {
    return (ae_load64((b)));
}

long long borrow_kind(long long b) {
    return (ae_load64((b + 8)));
}

long long borrow_start(long long b) {
    return (ae_load64((b + 16)));
}

long long borrow_end(long long b) {
    return (ae_load64((b + 24)));
}

void borrow_set_end(long long b, long long e) {
    (ae_store64((b + 24),(e)));
}

long long ownership_ctx_new(long long parent, long long scope_id) {
    long long ctx = (ae_malloc((32)));
    (ae_store64((ctx),(vec_new())));
    (ae_store64((ctx + 8),(vec_new())));
    (ae_store64((ctx + 16),(scope_id)));
    (ae_store64((ctx + 24),(parent)));
    return (ctx);
}

long long ctx_vars(long long ctx) {
    return (ae_load64((ctx)));
}

long long ctx_borrows(long long ctx) {
    return (ae_load64((ctx + 8)));
}

long long ctx_scope(long long ctx) {
    return (ae_load64((ctx + 16)));
}

long long ctx_parent(long long ctx) {
    return (ae_load64((ctx + 24)));
}

long long var_state_new(long long name, long long state) {
    long long vs = (ae_malloc((24)));
    (ae_store64((vs),(name)));
    (ae_store64((vs + 8),(state)));
    (ae_store64((vs + 16),(0)));
    return (vs);
}

long long var_state_name(long long vs) {
    return (ae_load64((vs)));
}

long long var_state_get(long long vs) {
    return (ae_load64((vs + 8)));
}

void var_state_set(long long vs, long long s) {
    (ae_store64((vs + 8),(s)));
}

long long var_state_dropped(long long vs) {
    return (ae_load64((vs + 16)));
}

void var_state_mark_dropped(long long vs) {
    (ae_store64((vs + 16),(1)));
}

void ctx_add_var(long long ctx, long long name) {
    (vec_push((ctx_vars((ctx))),(var_state_new((name),(OWN_OWNED)))));
}

long long ctx_find_var(long long ctx, long long name) {
    if (ctx == 0) {
    return (0);
    }
    long long vars = (ctx_vars((ctx)));
    long long count = (vec_len((vars)));
    long long i = (0);
    while (i < count) {
    long long vs = (vec_get((vars),(i)));
    if (str_eq((var_state_name((vs))),(name)) == 1) {
    return (vs);
    }
    (i) = (i + 1);
    }
    return (ctx_find_var((ctx_parent((ctx))),(name)));
}

long long ctx_move_var(long long ctx, long long name) {
    long long vs = (ctx_find_var((ctx),(name)));
    if (vs == 0) {
    return (0);
    }
    long long state = (var_state_get((vs)));
    if (state == OWN_MOVED) {
    return (0 - 1);
    }
    if (state == OWN_BORROWED || state == OWN_MUT_BORROWED) {
    return (0 - 2);
    }
    (var_state_set((vs),(OWN_MOVED)));
    return (1);
}

long long ctx_borrow_var(long long ctx, long long name, long long is_mut) {
    long long vs = (ctx_find_var((ctx),(name)));
    if (vs == 0) {
    return (0);
    }
    long long state = (var_state_get((vs)));
    if (state == OWN_MOVED) {
    return (0 - 1);
    }
    if (is_mut == 1) {
    if (state == OWN_BORROWED || state == OWN_MUT_BORROWED) {
    return (0 - 2);
    }
    (var_state_set((vs),(OWN_MUT_BORROWED)));
    } else {
    if (state == OWN_MUT_BORROWED) {
    return (0 - 3);
    }
    (var_state_set((vs),(OWN_BORROWED)));
    }
    long long borrow = (borrow_new((name),(is_mut),(ctx_scope((ctx)))));
    (vec_push((ctx_borrows((ctx))),(borrow)));
    return (1);
}

void ctx_return_borrow(long long ctx, long long name) {
    long long vs = (ctx_find_var((ctx),(name)));
    if (vs == 0) {
    return;
    }
    long long borrows = (ctx_borrows((ctx)));
    long long count = (vec_len((borrows)));
    long long remaining_borrows = (0);
    long long i = (0);
    while (i < count) {
    long long b = (vec_get((borrows),(i)));
    if (str_eq((borrow_target((b))),(name)) == 1) {
    if (borrow_end((b)) == 0) {
    (borrow_set_end((b),(ctx_scope((ctx)))));
    }
    }
    (i) = (i + 1);
    }
    (var_state_set((vs),(OWN_OWNED)));
}

long long ctx_get_drops(long long ctx) {
    long long drops = (vec_new());
    long long vars = (ctx_vars((ctx)));
    long long count = (vec_len((vars)));
    long long i = (0);
    while (i < count) {
    long long vs = (vec_get((vars),(i)));
    long long state = (var_state_get((vs)));
    if (state == OWN_OWNED) {
    if (var_state_dropped((vs)) == 0) {
    (vec_push((drops),(var_state_name((vs)))));
    (var_state_mark_dropped((vs)));
    }
    }
    (i) = (i + 1);
    }
    return (drops);
}

long long check_expr_ownership(long long ctx, long long node) {
    if (node == 0) {
    return (1);
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_IDENT) {
    long long name = (ast_data1((node)));
    long long vs = (ctx_find_var((ctx),(name)));
    if (vs != 0) {
    long long state = (var_state_get((vs)));
    if (state == OWN_MOVED) {
    return (0);
    }
    }
    return (1);
    }
    if (kind == AST_UNARY) {
    long long op = (ast_data1((node)));
    long long operand = (ast_data2((node)));
    if (op == TOK_AMP) {
    if (ast_kind((operand)) == AST_IDENT) {
    return (ctx_borrow_var((ctx),(ast_data1((operand))),(0)));
    }
    }
    if (op == TOK_STAR) {
    return (check_expr_ownership((ctx),(operand)));
    }
    return (check_expr_ownership((ctx),(operand)));
    }
    if (kind == AST_BINARY) {
    long long r1 = (check_expr_ownership((ctx),(ast_data2((node)))));
    long long r2 = (check_expr_ownership((ctx),(ast_data3((node)))));
    if (r1 < 1) {
    return (r1);
    }
    if (r2 < 1) {
    return (r2);
    }
    return (1);
    }
    if (kind == AST_CALL) {
    long long args = (ast_data2((node)));
    long long count = (vec_len((args)));
    long long i = (0);
    while (i < count) {
    long long arg = (vec_get((args),(i)));
    if (ast_kind((arg)) == AST_IDENT) {
    long long result = (ctx_move_var((ctx),(ast_data1((arg)))));
    if (result < 0) {
    return (result);
    }
    } else {
    long long result = (check_expr_ownership((ctx),(arg)));
    if (result < 1) {
    return (result);
    }
    }
    (i) = (i + 1);
    }
    return (check_expr_ownership((ctx),(ast_data1((node)))));
    }
    return (1);
}

long long check_stmt_ownership(long long ctx, long long node) {
    if (node == 0) {
    return (1);
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_LET) {
    long long name = (ast_data1((node)));
    long long init = (ast_data3((node)));
    long long result = (check_expr_ownership((ctx),(init)));
    if (result < 1) {
    return (result);
    }
    (ctx_add_var((ctx),(name)));
    return (1);
    }
    if (kind == AST_ASSIGN) {
    long long target = (ast_data1((node)));
    long long value = (ast_data2((node)));
    long long result = (check_expr_ownership((ctx),(value)));
    if (result < 1) {
    return (result);
    }
    return (1);
    }
    if (kind == AST_RETURN) {
    return (check_expr_ownership((ctx),(ast_data1((node)))));
    }
    if (kind == AST_IF) {
    (check_expr_ownership((ctx),(ast_data1((node)))));
    (check_block_ownership((ctx),(ast_data2((node)))));
    long long else_blk = (ast_data3((node)));
    if (else_blk != 0) {
    (check_block_ownership((ctx),(else_blk)));
    }
    return (1);
    }
    if (kind == AST_WHILE) {
    (check_expr_ownership((ctx),(ast_data1((node)))));
    (check_block_ownership((ctx),(ast_data2((node)))));
    return (1);
    }
    if (kind == AST_EXPR_STMT) {
    return (check_expr_ownership((ctx),(ast_data1((node)))));
    }
    return (1);
}

long long check_block_ownership(long long parent_ctx, long long block) {
    if (block == 0) {
    return (1);
    }
    long long scope_id = (ctx_scope((parent_ctx)) + 1);
    long long ctx = (ownership_ctx_new((parent_ctx),(scope_id)));
    long long stmts = (ast_data1((block)));
    if (stmts == 0) {
    return (1);
    }
    long long count = (vec_len((stmts)));
    long long i = (0);
    while (i < count) {
    long long result = (check_stmt_ownership((ctx),(vec_get((stmts),(i)))));
    if (result < 1) {
    return (result);
    }
    (i) = (i + 1);
    }
    long long drops = (ctx_get_drops((ctx)));
    return (1);
}

long long check_ownership(long long module) {
    long long decls = (ast_data1((module)));
    long long count = (vec_len((decls)));
    long long i = (0);
    while (i < count) {
    long long decl = (vec_get((decls),(i)));
    if (ast_kind((decl)) == AST_FUNC) {
    long long ctx = (ownership_ctx_new((0),(0)));
    long long params = (ast_data2((decl)));
    long long pcount = (vec_len((params)));
    long long j = (0);
    while (j < pcount) {
    long long param = (vec_get((params),(j)));
    (ctx_add_var((ctx),(ae_load64((param)))));
    (j) = (j + 1);
    }
    long long result = (check_block_ownership((ctx),(ast_data4((decl)))));
    if (result < 1) {
    return (result);
    }
    }
    (i) = (i + 1);
    }
    return (1);
}

// already imported: stdlib/std.aether
#define EFF_NONE (0)
#define EFF_IO (1)
#define EFF_ASYNC (2)
#define EFF_THROW (3)
#define EFF_STATE (4)
#define EFF_YIELD (5)
#define EFF_RESUME (6)
long long effect_new(long long kind, long long name) {
    long long e = (ae_malloc((24)));
    (ae_store64((e),(kind)));
    (ae_store64((e + 8),(name)));
    (ae_store64((e + 16),(vec_new())));
    return (e);
}

long long effect_kind(long long e) {
    return (ae_load64((e)));
}

long long effect_name(long long e) {
    return (ae_load64((e + 8)));
}

long long effect_params(long long e) {
    return (ae_load64((e + 16)));
}

long long effect_set_new(void) {
    return (vec_new());
}

void effect_set_add(long long set, long long eff) {
    long long count = (vec_len((set)));
    long long i = (0);
    while (i < count) {
    long long existing = (vec_get((set),(i)));
    if (effect_kind((existing)) == effect_kind((eff))) {
    return;
    }
    (i) = (i + 1);
    }
    (vec_push((set),(eff)));
}

long long effect_set_has(long long set, long long kind) {
    long long count = (vec_len((set)));
    long long i = (0);
    while (i < count) {
    long long e = (vec_get((set),(i)));
    if (effect_kind((e)) == kind) {
    return (1);
    }
    (i) = (i + 1);
    }
    return (0);
}

long long effect_set_union(long long a, long long b) {
    long long result = (effect_set_new());
    long long count_a = (vec_len((a)));
    long long i = (0);
    while (i < count_a) {
    (effect_set_add((result),(vec_get((a),(i)))));
    (i) = (i + 1);
    }
    long long count_b = (vec_len((b)));
    (i) = (0);
    while (i < count_b) {
    (effect_set_add((result),(vec_get((b),(i)))));
    (i) = (i + 1);
    }
    return (result);
}

long long handler_new(long long kind, long long handler, long long resume) {
    long long h = (ae_malloc((24)));
    (ae_store64((h),(kind)));
    (ae_store64((h + 8),(handler)));
    (ae_store64((h + 16),(resume)));
    return (h);
}

long long handler_kind(long long h) {
    return (ae_load64((h)));
}

long long handler_func(long long h) {
    return (ae_load64((h + 8)));
}

long long handler_resume(long long h) {
    return (ae_load64((h + 16)));
}

long long handler_stack_new(void) {
    return (vec_new());
}

void handler_stack_push(long long stack, long long handler) {
    (vec_push((stack),(handler)));
}

long long handler_stack_find(long long stack, long long kind) {
    long long count = (vec_len((stack)));
    long long i = (count - 1);
    while (i >= 0) {
    long long h = (vec_get((stack),(i)));
    if (handler_kind((h)) == kind) {
    return (h);
    }
    (i) = (i - 1);
    }
    return (0);
}

long long effect_ctx_new(long long parent) {
    long long ctx = (ae_malloc((24)));
    (ae_store64((ctx),(handler_stack_new())));
    (ae_store64((ctx + 8),(effect_set_new())));
    (ae_store64((ctx + 16),(parent)));
    return (ctx);
}

long long ctx_handlers(long long ctx) {
    return (ae_load64((ctx)));
}

long long ctx_effects(long long ctx) {
    return (ae_load64((ctx + 8)));
}

long long ctx_effect_parent(long long ctx) {
    return (ae_load64((ctx + 16)));
}

void ctx_add_effect(long long ctx, long long kind) {
    long long eff = (effect_new((kind),(0)));
    (effect_set_add((ctx_effects((ctx))),(eff)));
}

void infer_effects_expr(long long ctx, long long node) {
    if (node == 0) {
    return;
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_CALL) {
    long long func_expr = (ast_data1((node)));
    if (ast_kind((func_expr)) == AST_IDENT) {
    long long name = (ast_data1((func_expr)));
    if (str_has_prefix((name),("print")) == 1) {
    (ctx_add_effect((ctx),(EFF_IO)));
    }
    if (str_has_prefix((name),("async")) == 1) {
    (ctx_add_effect((ctx),(EFF_ASYNC)));
    }
    if (str_has_prefix((name),("panic")) == 1) {
    (ctx_add_effect((ctx),(EFF_THROW)));
    }
    }
    long long args = (ast_data2((node)));
    long long count = (vec_len((args)));
    long long i = (0);
    while (i < count) {
    (infer_effects_expr((ctx),(vec_get((args),(i)))));
    (i) = (i + 1);
    }
    return;
    }
    if (kind == AST_BINARY) {
    (infer_effects_expr((ctx),(ast_data2((node)))));
    (infer_effects_expr((ctx),(ast_data3((node)))));
    return;
    }
    if (kind == AST_UNARY) {
    (infer_effects_expr((ctx),(ast_data2((node)))));
    return;
    }
}

void infer_effects_stmt(long long ctx, long long node) {
    if (node == 0) {
    return;
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_LET) {
    (infer_effects_expr((ctx),(ast_data3((node)))));
    return;
    }
    if (kind == AST_RETURN) {
    (infer_effects_expr((ctx),(ast_data1((node)))));
    return;
    }
    if (kind == AST_IF) {
    (infer_effects_expr((ctx),(ast_data1((node)))));
    (infer_effects_block((ctx),(ast_data2((node)))));
    long long else_blk = (ast_data3((node)));
    if (else_blk != 0) {
    (infer_effects_block((ctx),(else_blk)));
    }
    return;
    }
    if (kind == AST_WHILE) {
    (infer_effects_expr((ctx),(ast_data1((node)))));
    (infer_effects_block((ctx),(ast_data2((node)))));
    return;
    }
    if (kind == AST_EXPR_STMT) {
    (infer_effects_expr((ctx),(ast_data1((node)))));
    return;
    }
}

void infer_effects_block(long long ctx, long long block) {
    if (block == 0) {
    return;
    }
    long long stmts = (ast_data1((block)));
    if (stmts == 0) {
    return;
    }
    long long count = (vec_len((stmts)));
    long long i = (0);
    while (i < count) {
    (infer_effects_stmt((ctx),(vec_get((stmts),(i)))));
    (i) = (i + 1);
    }
}

long long infer_func_effects(long long node) {
    long long ctx = (effect_ctx_new((0)));
    long long body = (ast_data4((node)));
    (infer_effects_block((ctx),(body)));
    return (ctx_effects((ctx)));
}

long long verify_effects(long long module) {
    long long decls = (ast_data1((module)));
    long long count = (vec_len((decls)));
    long long i = (0);
    while (i < count) {
    long long decl = (vec_get((decls),(i)));
    if (ast_kind((decl)) == AST_FUNC) {
    long long effects = (infer_func_effects((decl)));
    long long eff_count = (vec_len((effects)));
    long long name = (ast_data1((decl)));
    if (str_eq((name),("main")) == 1) {
    } else {
    }
    }
    (i) = (i + 1);
    }
    return (1);
}

long long str_has_prefix(long long s, long long prefix) {
    if (s == 0 || prefix == 0) {
    return (0);
    }
    long long i = (0);
    while (1 == 1) {
    long long pc = (ae_load8((prefix + i)));
    if (pc == 0) {
    return (1);
    }
    long long sc = (ae_load8((s + i)));
    if (sc != pc) {
    return (0);
    }
    (i) = (i + 1);
    }
    return (0);
}

// already imported: stdlib/std.aether
long long parser_new(long long tokens) {
    long long p = (ae_malloc((24)));
    (ae_store64((p),(tokens)));
    (ae_store64((p + 8),(0)));
    (ae_store64((p + 16),(vec_len((tokens)))));
    return (p);
}

long long parser_tokens(long long p) {
    return (ae_load64((p)));
}

long long parser_pos(long long p) {
    return (ae_load64((p + 8)));
}

long long parser_count(long long p) {
    return (ae_load64((p + 16)));
}

void parser_set_pos(long long p, long long pos) {
    (ae_store64((p + 8),(pos)));
}

long long parser_peek(long long p) {
    long long pos = (parser_pos((p)));
    if (pos >= parser_count((p))) {
    return (0);
    }
    return (vec_get((parser_tokens((p))),(pos)));
}

long long parser_advance(long long p) {
    long long tok = (parser_peek((p)));
    (parser_set_pos((p),(parser_pos((p)) + 1)));
    return (tok);
}

long long parser_check(long long p, long long typ) {
    long long tok = (parser_peek((p)));
    if (tok == 0) {
    return (0);
    }
    if (token_type((tok)) == typ) {
    return (1);
    }
    return (0);
}

long long parser_match(long long p, long long typ) {
    if (parser_check((p),(typ)) == 1) {
    (parser_advance((p)));
    return (1);
    }
    return (0);
}

long long parser_expect(long long p, long long typ) {
    if (parser_check((p),(typ)) == 1) {
    return (parser_advance((p)));
    }
    return (0);
}

long long get_prec(long long typ) {
    if (typ == TOK_PIPE) {
    return (1);
    }
    if (typ == TOK_AMP) {
    return (2);
    }
    if (typ == TOK_EQEQ) {
    return (3);
    }
    if (typ == TOK_NE) {
    return (3);
    }
    if (typ == TOK_LT) {
    return (4);
    }
    if (typ == TOK_LE) {
    return (4);
    }
    if (typ == TOK_GT) {
    return (4);
    }
    if (typ == TOK_GE) {
    return (4);
    }
    if (typ == TOK_PLUS) {
    return (5);
    }
    if (typ == TOK_MINUS) {
    return (5);
    }
    if (typ == TOK_STAR) {
    return (6);
    }
    if (typ == TOK_SLASH) {
    return (6);
    }
    return (0);
}

long long parse_type(long long p) {
    long long tok = (parser_peek((p)));
    if (tok == 0) {
    return (0);
    }
    if (parser_check((p),(TOK_LBRACK)) == 1) {
    (parser_advance((p)));
    long long inner = (parse_type((p)));
    (parser_expect((p),(TOK_RBRACK)));
    long long arr_type = (ast_new((AST_TYPE_ARRAY),(token_line((tok))),(token_col((tok)))));
    (ast_set_data1((arr_type),(inner)));
    return (arr_type);
    }
    long long name_tok = (parser_expect((p),(TOK_ID)));
    if (name_tok == 0) {
    return (0);
    }
    long long type_node = (ast_new((AST_TYPE_NAME),(token_line((name_tok))),(token_col((name_tok)))));
    (ast_set_data1((type_node),(token_value((name_tok)))));
    if (parser_check((p),(TOK_LT)) == 1) {
    (parser_advance((p)));
    long long generics = (vec_new());
    (vec_push((generics),(parse_type((p)))));
    while (parser_match((p),(TOK_COMMA)) == 1) {
    (vec_push((generics),(parse_type((p)))));
    }
    (parser_expect((p),(TOK_GT)));
    (ast_set_data2((type_node),(generics)));
    }
    return (type_node);
}

long long parse_primary(long long p) {
    long long tok = (parser_peek((p)));
    if (tok == 0) {
    return (0);
    }
    long long typ = (token_type((tok)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    if (typ == TOK_INT) {
    (parser_advance((p)));
    return (ast_int((token_value((tok))),(line),(col)));
    }
    if (typ == TOK_ID) {
    (parser_advance((p)));
    long long id = (ast_ident((token_value((tok))),(line),(col)));
    if (parser_check((p),(TOK_LBRACE)) == 1) {
    (parser_advance((p)));
    long long fields = (vec_new());
    if (parser_check((p),(TOK_RBRACE)) == 0) {
    long long field_name = (parser_expect((p),(TOK_ID)));
    (parser_expect((p),(TOK_COLON)));
    long long field_val = (parse_expr((p)));
    long long pair = (ae_malloc((16)));
    (ae_store64((pair),(token_value((field_name)))));
    (ae_store64((pair + 8),(field_val)));
    (vec_push((fields),(pair)));
    while (parser_match((p),(TOK_COMMA)) == 1) {
    if (parser_check((p),(TOK_RBRACE)) == 1) {
    break;
    }
    long long fn2 = (parser_expect((p),(TOK_ID)));
    (parser_expect((p),(TOK_COLON)));
    long long fv2 = (parse_expr((p)));
    long long p2 = (ae_malloc((16)));
    (ae_store64((p2),(token_value((fn2)))));
    (ae_store64((p2 + 8),(fv2)));
    (vec_push((fields),(p2)));
    }
    }
    (parser_expect((p),(TOK_RBRACE)));
    long long lit = (ast_new((AST_STRUCT_LIT),(line),(col)));
    (ast_set_data1((lit),(token_value((tok)))));
    (ast_set_data2((lit),(fields)));
    return (lit);
    }
    return (id);
    }
    if (typ == TOK_TRUE) {
    (parser_advance((p)));
    return (ast_int((1),(line),(col)));
    }
    if (typ == TOK_FALSE) {
    (parser_advance((p)));
    return (ast_int((0),(line),(col)));
    }
    if (typ == TOK_LPAREN) {
    (parser_advance((p)));
    long long inner = (parse_expr((p)));
    (parser_expect((p),(TOK_RPAREN)));
    return (inner);
    }
    if (typ == TOK_LBRACK) {
    (parser_advance((p)));
    long long elements = (vec_new());
    if (parser_check((p),(TOK_RBRACK)) == 0) {
    (vec_push((elements),(parse_expr((p)))));
    while (parser_match((p),(TOK_COMMA)) == 1) {
    if (parser_check((p),(TOK_RBRACK)) == 1) {
    break;
    }
    (vec_push((elements),(parse_expr((p)))));
    }
    }
    (parser_expect((p),(TOK_RBRACK)));
    long long arr = (ast_new((AST_ARRAY_LIT),(line),(col)));
    (ast_set_data1((arr),(elements)));
    return (arr);
    }
    return (0);
}

long long parse_postfix(long long p) {
    long long expr = (parse_primary((p)));
    if (expr == 0) {
    return (0);
    }
    long long done = (0);
    while (done == 0) {
    if (parser_check((p),(TOK_LPAREN)) == 1) {
    (parser_advance((p)));
    long long args = (vec_new());
    if (parser_check((p),(TOK_RPAREN)) == 0) {
    (vec_push((args),(parse_expr((p)))));
    while (parser_match((p),(TOK_COMMA)) == 1) {
    (vec_push((args),(parse_expr((p)))));
    }
    }
    (parser_expect((p),(TOK_RPAREN)));
    (expr) = (ast_call((expr),(args),(ast_line((expr))),(ast_col((expr)))));
    } else {
    if (parser_check((p),(TOK_DOT)) == 1) {
    (parser_advance((p)));
    long long field = (parser_expect((p),(TOK_ID)));
    if (field != 0) {
    (expr) = (ast_field((expr),(token_value((field))),(ast_line((expr))),(ast_col((expr)))));
    }
    } else {
    if (parser_check((p),(TOK_LBRACK)) == 1) {
    (parser_advance((p)));
    long long index = (parse_expr((p)));
    (parser_expect((p),(TOK_RBRACK)));
    long long idx_node = (ast_new((AST_INDEX),(ast_line((expr))),(ast_col((expr)))));
    (ast_set_data1((idx_node),(expr)));
    (ast_set_data2((idx_node),(index)));
    (expr) = (idx_node);
    } else {
    (done) = (1);
    }
    }
    }
    }
    return (expr);
}

long long parse_unary(long long p) {
    long long tok = (parser_peek((p)));
    if (tok == 0) {
    return (0);
    }
    long long typ = (token_type((tok)));
    if (typ == TOK_MINUS) {
    (parser_advance((p)));
    long long operand = (parse_unary((p)));
    return (ast_unary((TOK_MINUS),(operand),(token_line((tok))),(token_col((tok)))));
    }
    if (typ == TOK_BANG) {
    (parser_advance((p)));
    long long operand = (parse_unary((p)));
    return (ast_unary((TOK_BANG),(operand),(token_line((tok))),(token_col((tok)))));
    }
    if (typ == TOK_AMP) {
    (parser_advance((p)));
    long long operand = (parse_unary((p)));
    return (ast_unary((TOK_AMP),(operand),(token_line((tok))),(token_col((tok)))));
    }
    if (typ == TOK_STAR) {
    (parser_advance((p)));
    long long operand = (parse_unary((p)));
    return (ast_unary((TOK_STAR),(operand),(token_line((tok))),(token_col((tok)))));
    }
    return (parse_postfix((p)));
}

long long parse_binary(long long p, long long min_prec) {
    long long left = (parse_unary((p)));
    if (left == 0) {
    return (0);
    }
    long long done = (0);
    while (done == 0) {
    long long tok = (parser_peek((p)));
    if (tok == 0) {
    (done) = (1);
    } else {
    long long prec = (get_prec((token_type((tok)))));
    if (prec < min_prec) {
    (done) = (1);
    } else {
    (parser_advance((p)));
    long long right = (parse_binary((p),(prec + 1)));
    (left) = (ast_binary((token_type((tok))),(left),(right),(ast_line((left))),(ast_col((left)))));
    }
    }
    }
    return (left);
}

long long parse_expr(long long p) {
    return (parse_binary((p),(1)));
}

long long parse_block(long long p) {
    long long tok = (parser_peek((p)));
    long long line = (0);
    long long col = (0);
    if (tok != 0) {
    (line) = (token_line((tok)));
    (col) = (token_col((tok)));
    }
    (parser_expect((p),(TOK_LBRACE)));
    long long stmts = (vec_new());
    while (parser_check((p),(TOK_RBRACE)) == 0) {
    if (parser_check((p),(TOK_EOF)) == 1) {
    return (ast_block((stmts),(line),(col)));
    }
    long long stmt = (parse_stmt((p)));
    if (stmt != 0) {
    (vec_push((stmts),(stmt)));
    }
    }
    (parser_expect((p),(TOK_RBRACE)));
    return (ast_block((stmts),(line),(col)));
}

long long parse_stmt(long long p) {
    long long tok = (parser_peek((p)));
    if (tok == 0) {
    return (0);
    }
    long long typ = (token_type((tok)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    if (typ == TOK_LET) {
    (parser_advance((p)));
    long long is_mut = (0);
    if (parser_match((p),(TOK_MUT)) == 1) {
    (is_mut) = (1);
    }
    long long name_tok = (parser_expect((p),(TOK_ID)));
    long long name = (0);
    if (name_tok != 0) {
    (name) = (token_value((name_tok)));
    }
    long long var_type = (0);
    if (parser_match((p),(TOK_COLON)) == 1) {
    (var_type) = (parse_type((p)));
    }
    (parser_expect((p),(TOK_EQ)));
    long long init = (parse_expr((p)));
    return (ast_let((name),(var_type),(init),(is_mut),(line),(col)));
    }
    if (typ == TOK_RETURN) {
    (parser_advance((p)));
    long long ret_expr = (0);
    if (parser_check((p),(TOK_RBRACE)) == 0) {
    (ret_expr) = (parse_expr((p)));
    }
    return (ast_return((ret_expr),(line),(col)));
    }
    if (typ == TOK_IF) {
    (parser_advance((p)));
    long long cond = (parse_expr((p)));
    long long then_blk = (parse_block((p)));
    long long else_blk = (0);
    if (parser_match((p),(TOK_ELSE)) == 1) {
    if (parser_check((p),(TOK_IF)) == 1) {
    (else_blk) = (parse_stmt((p)));
    } else {
    (else_blk) = (parse_block((p)));
    }
    }
    return (ast_if((cond),(then_blk),(else_blk),(line),(col)));
    }
    if (typ == TOK_WHILE) {
    (parser_advance((p)));
    long long cond = (parse_expr((p)));
    long long body = (parse_block((p)));
    return (ast_while((cond),(body),(line),(col)));
    }
    if (typ == TOK_FOR) {
    (parser_advance((p)));
    long long var_tok = (parser_expect((p),(TOK_ID)));
    long long var_name = (0);
    if (var_tok != 0) {
    (var_name) = (token_value((var_tok)));
    }
    (parser_expect((p),(TOK_IN)));
    long long range_start = (parse_expr((p)));
    (parser_match((p),(TOK_DOT)));
    (parser_match((p),(TOK_DOT)));
    long long range_end = (parse_expr((p)));
    long long body = (parse_block((p)));
    long long for_node = (ast_new((AST_FOR),(line),(col)));
    (ast_set_data1((for_node),(var_name)));
    (ast_set_data2((for_node),(range_start)));
    (ast_set_data3((for_node),(range_end)));
    (ast_set_data4((for_node),(body)));
    return (for_node);
    }
    if (typ == TOK_MATCH) {
    (parser_advance((p)));
    long long scrutinee = (parse_expr((p)));
    (parser_expect((p),(TOK_LBRACE)));
    long long arms = (vec_new());
    while (parser_check((p),(TOK_RBRACE)) == 0) {
    long long pattern = (parse_expr((p)));
    (parser_expect((p),(TOK_DARROW)));
    long long arm_body = (parse_expr((p)));
    long long arm = (ae_malloc((16)));
    (ae_store64((arm),(pattern)));
    (ae_store64((arm + 8),(arm_body)));
    (vec_push((arms),(arm)));
    (parser_match((p),(TOK_COMMA)));
    }
    (parser_expect((p),(TOK_RBRACE)));
    long long match_node = (ast_new((AST_MATCH),(line),(col)));
    (ast_set_data1((match_node),(scrutinee)));
    (ast_set_data2((match_node),(arms)));
    return (match_node);
    }
    long long expr = (parse_expr((p)));
    if (expr != 0) {
    if (parser_check((p),(TOK_EQ)) == 1) {
    (parser_advance((p)));
    long long value = (parse_expr((p)));
    long long assign = (ast_new((AST_ASSIGN),(line),(col)));
    (ast_set_data1((assign),(expr)));
    (ast_set_data2((assign),(value)));
    return (assign);
    }
    long long stmt = (ast_new((AST_EXPR_STMT),(line),(col)));
    (ast_set_data1((stmt),(expr)));
    return (stmt);
    }
    return (0);
}

long long parse_func(long long p) {
    long long tok = (parser_peek((p)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    (parser_expect((p),(TOK_FUNC)));
    long long name_tok = (parser_expect((p),(TOK_ID)));
    long long name = (0);
    if (name_tok != 0) {
    (name) = (token_value((name_tok)));
    }
    long long generics = (vec_new());
    if (parser_check((p),(TOK_LT)) == 1) {
    (parser_advance((p)));
    long long gen_tok = (parser_expect((p),(TOK_ID)));
    if (gen_tok != 0) {
    (vec_push((generics),(token_value((gen_tok)))));
    }
    while (parser_match((p),(TOK_COMMA)) == 1) {
    long long g2 = (parser_expect((p),(TOK_ID)));
    if (g2 != 0) {
    (vec_push((generics),(token_value((g2)))));
    }
    }
    (parser_expect((p),(TOK_GT)));
    }
    (parser_expect((p),(TOK_LPAREN)));
    long long params = (vec_new());
    if (parser_check((p),(TOK_RPAREN)) == 0) {
    long long param_name = (parser_expect((p),(TOK_ID)));
    (parser_expect((p),(TOK_COLON)));
    long long param_type = (parse_type((p)));
    long long param = (ae_malloc((16)));
    (ae_store64((param),(token_value((param_name)))));
    (ae_store64((param + 8),(param_type)));
    (vec_push((params),(param)));
    while (parser_match((p),(TOK_COMMA)) == 1) {
    long long pn = (parser_expect((p),(TOK_ID)));
    (parser_expect((p),(TOK_COLON)));
    long long pt = (parse_type((p)));
    long long pm = (ae_malloc((16)));
    (ae_store64((pm),(token_value((pn)))));
    (ae_store64((pm + 8),(pt)));
    (vec_push((params),(pm)));
    }
    }
    (parser_expect((p),(TOK_RPAREN)));
    long long ret_type = (0);
    if (parser_match((p),(TOK_ARROW)) == 1) {
    (ret_type) = (parse_type((p)));
    }
    long long body = (parse_block((p)));
    long long func_node = (ast_func((name),(params),(ret_type),(body),(line),(col)));
    if (vec_len((generics)) > 0) {
    }
    return (func_node);
}

long long parse_struct(long long p) {
    long long tok = (parser_peek((p)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    (parser_expect((p),(TOK_STRUCT)));
    long long name_tok = (parser_expect((p),(TOK_ID)));
    long long name = (0);
    if (name_tok != 0) {
    (name) = (token_value((name_tok)));
    }
    long long generics = (vec_new());
    if (parser_check((p),(TOK_LT)) == 1) {
    (parser_advance((p)));
    long long gen_tok = (parser_expect((p),(TOK_ID)));
    if (gen_tok != 0) {
    (vec_push((generics),(token_value((gen_tok)))));
    }
    while (parser_match((p),(TOK_COMMA)) == 1) {
    long long g2 = (parser_expect((p),(TOK_ID)));
    if (g2 != 0) {
    (vec_push((generics),(token_value((g2)))));
    }
    }
    (parser_expect((p),(TOK_GT)));
    }
    (parser_expect((p),(TOK_LBRACE)));
    long long fields = (vec_new());
    while (parser_check((p),(TOK_RBRACE)) == 0) {
    long long field_name = (parser_expect((p),(TOK_ID)));
    (parser_expect((p),(TOK_COLON)));
    long long field_type = (parse_type((p)));
    long long field = (ae_malloc((16)));
    (ae_store64((field),(token_value((field_name)))));
    (ae_store64((field + 8),(field_type)));
    (vec_push((fields),(field)));
    (parser_match((p),(TOK_COMMA)));
    }
    (parser_expect((p),(TOK_RBRACE)));
    return (ast_struct((name),(fields),(generics),(line),(col)));
}

long long parse_trait(long long p) {
    long long tok = (parser_peek((p)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    (parser_expect((p),(TOK_TRAIT)));
    long long name_tok = (parser_expect((p),(TOK_ID)));
    long long name = (0);
    if (name_tok != 0) {
    (name) = (token_value((name_tok)));
    }
    long long generics = (vec_new());
    if (parser_check((p),(TOK_LT)) == 1) {
    (parser_advance((p)));
    long long gen_tok = (parser_expect((p),(TOK_ID)));
    if (gen_tok != 0) {
    (vec_push((generics),(token_value((gen_tok)))));
    }
    while (parser_match((p),(TOK_COMMA)) == 1) {
    long long g2 = (parser_expect((p),(TOK_ID)));
    if (g2 != 0) {
    (vec_push((generics),(token_value((g2)))));
    }
    }
    (parser_expect((p),(TOK_GT)));
    }
    (parser_expect((p),(TOK_LBRACE)));
    long long methods = (vec_new());
    while (parser_check((p),(TOK_RBRACE)) == 0) {
    if (parser_check((p),(TOK_FUNC)) == 1) {
    (vec_push((methods),(parse_func((p)))));
    } else {
    (parser_advance((p)));
    }
    }
    (parser_expect((p),(TOK_RBRACE)));
    return (ast_trait((name),(methods),(generics),(line),(col)));
}

long long parse_impl(long long p) {
    long long tok = (parser_peek((p)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    (parser_expect((p),(TOK_IMPL)));
    long long first = (parse_type((p)));
    long long trait_name = (0);
    long long for_type = (first);
    if (parser_match((p),(TOK_FOR)) == 1) {
    (trait_name) = (first);
    (for_type) = (parse_type((p)));
    }
    (parser_expect((p),(TOK_LBRACE)));
    long long methods = (vec_new());
    while (parser_check((p),(TOK_RBRACE)) == 0) {
    if (parser_check((p),(TOK_FUNC)) == 1) {
    (vec_push((methods),(parse_func((p)))));
    } else {
    (parser_advance((p)));
    }
    }
    (parser_expect((p),(TOK_RBRACE)));
    return (ast_impl((trait_name),(for_type),(methods),(line),(col)));
}

long long parse_const(long long p) {
    long long tok = (parser_peek((p)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    (parser_expect((p),(TOK_CONST)));
    long long name_tok = (parser_expect((p),(TOK_ID)));
    long long name = (0);
    if (name_tok != 0) {
    (name) = (token_value((name_tok)));
    }
    (parser_expect((p),(TOK_COLON)));
    long long typ = (parse_type((p)));
    (parser_expect((p),(TOK_EQ)));
    long long val = (parse_expr((p)));
    return (ast_const((name),(typ),(val),(line),(col)));
}

long long parse_import(long long p) {
    long long tok = (parser_peek((p)));
    long long line = (token_line((tok)));
    long long col = (token_col((tok)));
    (parser_expect((p),(TOK_IMPORT)));
    long long path = (vec_new());
    long long name_tok = (parser_expect((p),(TOK_ID)));
    if (name_tok != 0) {
    (vec_push((path),(token_value((name_tok)))));
    }
    while (parser_match((p),(TOK_DOT)) == 1) {
    long long next = (parser_expect((p),(TOK_ID)));
    if (next != 0) {
    (vec_push((path),(token_value((next)))));
    }
    }
    return (ast_import((path),(line),(col)));
}

long long parse_module(long long p) {
    long long decls = (vec_new());
    while (parser_check((p),(TOK_EOF)) == 0) {
    long long tok = (parser_peek((p)));
    if (tok == 0) {
    return (ast_module((decls)));
    }
    long long typ = (token_type((tok)));
    if (typ == TOK_FUNC) {
    (vec_push((decls),(parse_func((p)))));
    } else {
    if (typ == TOK_STRUCT) {
    (vec_push((decls),(parse_struct((p)))));
    } else {
    if (typ == TOK_TRAIT) {
    (vec_push((decls),(parse_trait((p)))));
    } else {
    if (typ == TOK_IMPL) {
    (vec_push((decls),(parse_impl((p)))));
    } else {
    if (typ == TOK_CONST) {
    (vec_push((decls),(parse_const((p)))));
    } else {
    if (typ == TOK_IMPORT) {
    (vec_push((decls),(parse_import((p)))));
    } else {
    (parser_advance((p)));
    }
    }
    }
    }
    }
    }
    }
    return (ast_module((decls)));
}

long long parse(long long tokens) {
    long long p = (parser_new((tokens)));
    return (parse_module((p)));
}

// already imported: stdlib/std.aether
long long codegen_new(void) {
    long long g = (ae_malloc((48)));
    long long cap = (131072);
    (ae_store64((g),(ae_malloc((cap)))));
    (ae_store64((g + 8),(0)));
    (ae_store64((g + 16),(cap)));
    (ae_store64((g + 24),(0)));
    (ae_store64((g + 32),(vec_new())));
    (ae_store64((g + 40),(vec_new())));
    return (g);
}

long long cg_out(long long g) {
    return (ae_load64((g)));
}

long long cg_len(long long g) {
    return (ae_load64((g + 8)));
}

long long cg_indent(long long g) {
    return (ae_load64((g + 24)));
}

void cg_set_len(long long g, long long n) {
    (ae_store64((g + 8),(n)));
}

void cg_set_indent(long long g, long long n) {
    (ae_store64((g + 24),(n)));
}

long long cg_structs(long long g) {
    return (ae_load64((g + 32)));
}

long long cg_traits(long long g) {
    return (ae_load64((g + 40)));
}

void cg_emit_char(long long g, long long c) {
    long long len = (cg_len((g)));
    long long out = (cg_out((g)));
    (ae_store8((out + len),(c)));
    (cg_set_len((g),(len + 1)));
}

void cg_emit_cstr(long long g, long long s) {
    if (s == 0) {
    return;
    }
    long long i = (0);
    long long c = (ae_load8((s + i)));
    while (c != 0) {
    (cg_emit_char((g),(c)));
    (i) = (i + 1);
    (c) = (ae_load8((s + i)));
    }
}

void cg_emit_int(long long g, long long n) {
    if (n < 0) {
    (cg_emit_char((g),(45)));
    (n) = (0 - n);
    }
    if (n == 0) {
    (cg_emit_char((g),(48)));
    return;
    }
    long long buf = (ae_malloc((32)));
    long long i = (0);
    while (n > 0) {
    (ae_store8((buf + i),(48 + ((n % 10)))));
    (n) = (n / 10);
    (i) = (i + 1);
    }
    while (i > 0) {
    (i) = (i - 1);
    (cg_emit_char((g),(ae_load8((buf + i)))));
    }
}

void cg_emit_indent(long long g) {
    long long indent = (cg_indent((g)));
    long long i = (0);
    while (i < indent) {
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(32)));
    (i) = (i + 1);
    }
}

void cg_emit_nl(long long g) {
    (cg_emit_char((g),(10)));
}

void cg_emit_space(long long g) {
    (cg_emit_char((g),(32)));
}

void cg_emit_header(long long g) {
    (cg_emit_char((g),(47)));
    (cg_emit_char((g),(42)));
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(65)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(104)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(42)));
    (cg_emit_char((g),(47)));
    (cg_emit_nl((g)));
    (cg_emit_char((g),(35)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(117)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(60)));
    (cg_emit_char((g),(115)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(46)));
    (cg_emit_char((g),(104)));
    (cg_emit_char((g),(62)));
    (cg_emit_nl((g)));
    (cg_emit_char((g),(35)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(117)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(60)));
    (cg_emit_char((g),(115)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(98)));
    (cg_emit_char((g),(46)));
    (cg_emit_char((g),(104)));
    (cg_emit_char((g),(62)));
    (cg_emit_nl((g)));
    (cg_emit_char((g),(35)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(117)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(32)));
    (cg_emit_char((g),(60)));
    (cg_emit_char((g),(115)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_char((g),(46)));
    (cg_emit_char((g),(104)));
    (cg_emit_char((g),(62)));
    (cg_emit_nl((g)));
    (cg_emit_nl((g)));
    (cg_emit_runtime_macros((g)));
    (cg_emit_nl((g)));
}

void cg_emit_runtime_macros(long long g) {
    (cg_emit_char((g),(35)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(102)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(101)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(112)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(120)));
    (cg_emit_char((g),(41)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(112)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(102)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(34)));
    (cg_emit_char((g),(37)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(92)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(34)));
    (cg_emit_char((g),(44)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_char((g),(41)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(120)));
    (cg_emit_char((g),(41)));
    (cg_emit_char((g),(41)));
    (cg_emit_nl((g)));
    (cg_emit_char((g),(35)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(102)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(101)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(97)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(95)));
    (cg_emit_char((g),(109)));
    (cg_emit_char((g),(97)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(41)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_char((g),(41)));
    (cg_emit_char((g),(109)));
    (cg_emit_char((g),(97)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(41)));
    (cg_emit_char((g),(41)));
    (cg_emit_nl((g)));
}

void cg_emit_expr(long long g, long long node) {
    if (node == 0) {
    return;
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_INT_LIT) {
    (cg_emit_int((g),(ast_data1((node)))));
    return;
    }
    if (kind == AST_IDENT) {
    (cg_emit_cstr((g),(ast_data1((node)))));
    return;
    }
    if (kind == AST_BINARY) {
    (cg_emit_char((g),(40)));
    (cg_emit_expr((g),(ast_data2((node)))));
    (cg_emit_space((g)));
    long long op = (ast_data1((node)));
    if (op == TOK_PLUS) {
    (cg_emit_char((g),(43)));
    }
    if (op == TOK_MINUS) {
    (cg_emit_char((g),(45)));
    }
    if (op == TOK_STAR) {
    (cg_emit_char((g),(42)));
    }
    if (op == TOK_SLASH) {
    (cg_emit_char((g),(47)));
    }
    if (op == TOK_EQEQ) {
    (cg_emit_char((g),(61)));
    (cg_emit_char((g),(61)));
    }
    if (op == TOK_NE) {
    (cg_emit_char((g),(33)));
    (cg_emit_char((g),(61)));
    }
    if (op == TOK_LT) {
    (cg_emit_char((g),(60)));
    }
    if (op == TOK_LE) {
    (cg_emit_char((g),(60)));
    (cg_emit_char((g),(61)));
    }
    if (op == TOK_GT) {
    (cg_emit_char((g),(62)));
    }
    if (op == TOK_GE) {
    (cg_emit_char((g),(62)));
    (cg_emit_char((g),(61)));
    }
    if (op == TOK_AMP) {
    (cg_emit_char((g),(38)));
    (cg_emit_char((g),(38)));
    }
    if (op == TOK_PIPE) {
    (cg_emit_char((g),(124)));
    (cg_emit_char((g),(124)));
    }
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ast_data3((node)))));
    (cg_emit_char((g),(41)));
    return;
    }
    if (kind == AST_UNARY) {
    long long op = (ast_data1((node)));
    if (op == TOK_MINUS) {
    (cg_emit_char((g),(45)));
    }
    if (op == TOK_BANG) {
    (cg_emit_char((g),(33)));
    }
    if (op == TOK_AMP) {
    (cg_emit_char((g),(38)));
    }
    if (op == TOK_STAR) {
    (cg_emit_char((g),(42)));
    }
    (cg_emit_expr((g),(ast_data2((node)))));
    return;
    }
    if (kind == AST_CALL) {
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(40)));
    long long args = (ast_data2((node)));
    long long count = (vec_len((args)));
    long long i = (0);
    while (i < count) {
    if (i > 0) {
    (cg_emit_char((g),(44)));
    (cg_emit_space((g)));
    }
    (cg_emit_expr((g),(vec_get((args),(i)))));
    (i) = (i + 1);
    }
    (cg_emit_char((g),(41)));
    return;
    }
    if (kind == AST_FIELD) {
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(46)));
    (cg_emit_cstr((g),(ast_data2((node)))));
    return;
    }
    if (kind == AST_INDEX) {
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(91)));
    (cg_emit_expr((g),(ast_data2((node)))));
    (cg_emit_char((g),(93)));
    return;
    }
    if (kind == AST_STRUCT_LIT) {
    (cg_emit_char((g),(40)));
    (cg_emit_cstr((g),(ast_data1((node)))));
    (cg_emit_char((g),(41)));
    (cg_emit_char((g),(123)));
    long long fields = (ast_data2((node)));
    long long count = (vec_len((fields)));
    long long i = (0);
    while (i < count) {
    if (i > 0) {
    (cg_emit_char((g),(44)));
    (cg_emit_space((g)));
    }
    long long pair = (vec_get((fields),(i)));
    (cg_emit_char((g),(46)));
    (cg_emit_cstr((g),(ae_load64((pair)))));
    (cg_emit_char((g),(61)));
    (cg_emit_expr((g),(ae_load64((pair + 8)))));
    (i) = (i + 1);
    }
    (cg_emit_char((g),(125)));
    return;
    }
    if (kind == AST_ARRAY_LIT) {
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_char((g),(91)));
    (cg_emit_char((g),(93)));
    (cg_emit_char((g),(41)));
    (cg_emit_char((g),(123)));
    long long elems = (ast_data1((node)));
    long long count = (vec_len((elems)));
    long long i = (0);
    while (i < count) {
    if (i > 0) {
    (cg_emit_char((g),(44)));
    (cg_emit_space((g)));
    }
    (cg_emit_expr((g),(vec_get((elems),(i)))));
    (i) = (i + 1);
    }
    (cg_emit_char((g),(125)));
    return;
    }
}

void cg_emit_stmt(long long g, long long node) {
    if (node == 0) {
    return;
    }
    long long kind = (ast_kind((node)));
    if (kind == AST_LET) {
    (cg_emit_indent((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(ast_data1((node)))));
    (cg_emit_space((g)));
    (cg_emit_char((g),(61)));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ast_data3((node)))));
    (cg_emit_char((g),(59)));
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_ASSIGN) {
    (cg_emit_indent((g)));
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_space((g)));
    (cg_emit_char((g),(61)));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ast_data2((node)))));
    (cg_emit_char((g),(59)));
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_RETURN) {
    (cg_emit_indent((g)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(117)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(110)));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(59)));
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_IF) {
    (cg_emit_indent((g)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(102)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(40)));
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(41)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(123)));
    (cg_emit_nl((g)));
    (cg_set_indent((g),(cg_indent((g)) + 1)));
    (cg_emit_block_stmts((g),(ast_data2((node)))));
    (cg_set_indent((g),(cg_indent((g)) - 1)));
    (cg_emit_indent((g)));
    (cg_emit_char((g),(125)));
    long long else_blk = (ast_data3((node)));
    if (else_blk != 0) {
    (cg_emit_space((g)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(115)));
    (cg_emit_char((g),(101)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(123)));
    (cg_emit_nl((g)));
    (cg_set_indent((g),(cg_indent((g)) + 1)));
    (cg_emit_block_stmts((g),(else_blk)));
    (cg_set_indent((g),(cg_indent((g)) - 1)));
    (cg_emit_indent((g)));
    (cg_emit_char((g),(125)));
    }
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_WHILE) {
    (cg_emit_indent((g)));
    (cg_emit_char((g),(119)));
    (cg_emit_char((g),(104)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(101)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(40)));
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(41)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(123)));
    (cg_emit_nl((g)));
    (cg_set_indent((g),(cg_indent((g)) + 1)));
    (cg_emit_block_stmts((g),(ast_data2((node)))));
    (cg_set_indent((g),(cg_indent((g)) - 1)));
    (cg_emit_indent((g)));
    (cg_emit_char((g),(125)));
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_FOR) {
    (cg_emit_indent((g)));
    (cg_emit_char((g),(102)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(114)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(40)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(ast_data1((node)))));
    (cg_emit_space((g)));
    (cg_emit_char((g),(61)));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ast_data2((node)))));
    (cg_emit_char((g),(59)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(ast_data1((node)))));
    (cg_emit_space((g)));
    (cg_emit_char((g),(60)));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ast_data3((node)))));
    (cg_emit_char((g),(59)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(ast_data1((node)))));
    (cg_emit_char((g),(43)));
    (cg_emit_char((g),(43)));
    (cg_emit_char((g),(41)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(123)));
    (cg_emit_nl((g)));
    (cg_set_indent((g),(cg_indent((g)) + 1)));
    (cg_emit_block_stmts((g),(ast_data4((node)))));
    (cg_set_indent((g),(cg_indent((g)) - 1)));
    (cg_emit_indent((g)));
    (cg_emit_char((g),(125)));
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_MATCH) {
    (cg_emit_indent((g)));
    (cg_emit_char((g),(115)));
    (cg_emit_char((g),(119)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(104)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(40)));
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(41)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(123)));
    (cg_emit_nl((g)));
    long long arms = (ast_data2((node)));
    long long count = (vec_len((arms)));
    long long i = (0);
    while (i < count) {
    long long arm = (vec_get((arms),(i)));
    (cg_emit_indent((g)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(97)));
    (cg_emit_char((g),(115)));
    (cg_emit_char((g),(101)));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ae_load64((arm)))));
    (cg_emit_char((g),(58)));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ae_load64((arm + 8)))));
    (cg_emit_char((g),(59)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(98)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(97)));
    (cg_emit_char((g),(107)));
    (cg_emit_char((g),(59)));
    (cg_emit_nl((g)));
    (i) = (i + 1);
    }
    (cg_emit_indent((g)));
    (cg_emit_char((g),(125)));
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_EXPR_STMT) {
    (cg_emit_indent((g)));
    (cg_emit_expr((g),(ast_data1((node)))));
    (cg_emit_char((g),(59)));
    (cg_emit_nl((g)));
    return;
    }
    if (kind == AST_BLOCK) {
    (cg_emit_block_stmts((g),(node)));
    return;
    }
}

void cg_emit_block_stmts(long long g, long long block) {
    if (block == 0) {
    return;
    }
    long long stmts = (ast_data1((block)));
    if (stmts == 0) {
    return;
    }
    long long count = (vec_len((stmts)));
    long long i = (0);
    while (i < count) {
    (cg_emit_stmt((g),(vec_get((stmts),(i)))));
    (i) = (i + 1);
    }
}

void cg_emit_struct(long long g, long long node) {
    long long name = (ast_data1((node)));
    long long fields = (ast_data2((node)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(121)));
    (cg_emit_char((g),(112)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(102)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(115)));
    (cg_emit_char((g),(116)));
    (cg_emit_char((g),(114)));
    (cg_emit_char((g),(117)));
    (cg_emit_char((g),(99)));
    (cg_emit_char((g),(116)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(123)));
    (cg_emit_nl((g)));
    long long count = (vec_len((fields)));
    long long i = (0);
    while (i < count) {
    long long field = (vec_get((fields),(i)));
    (cg_emit_indent((g)));
    (cg_emit_space((g)));
    (cg_emit_space((g)));
    (cg_emit_space((g)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(ae_load64((field)))));
    (cg_emit_char((g),(59)));
    (cg_emit_nl((g)));
    (i) = (i + 1);
    }
    (cg_emit_char((g),(125)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(name)));
    (cg_emit_char((g),(59)));
    (cg_emit_nl((g)));
    (cg_emit_nl((g)));
}

void cg_emit_func(long long g, long long node) {
    long long name = (ast_data1((node)));
    long long params = (ast_data2((node)));
    long long body = (ast_data4((node)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(name)));
    (cg_emit_char((g),(40)));
    long long count = (vec_len((params)));
    if (count == 0) {
    (cg_emit_char((g),(118)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(100)));
    } else {
    long long i = (0);
    while (i < count) {
    if (i > 0) {
    (cg_emit_char((g),(44)));
    (cg_emit_space((g)));
    }
    long long param = (vec_get((params),(i)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(108)));
    (cg_emit_char((g),(111)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(103)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(ae_load64((param)))));
    (i) = (i + 1);
    }
    }
    (cg_emit_char((g),(41)));
    (cg_emit_space((g)));
    (cg_emit_char((g),(123)));
    (cg_emit_nl((g)));
    (cg_set_indent((g),(1)));
    (cg_emit_block_stmts((g),(body)));
    (cg_emit_char((g),(125)));
    (cg_emit_nl((g)));
    (cg_emit_nl((g)));
    (cg_set_indent((g),(0)));
}

void cg_emit_const(long long g, long long node) {
    (cg_emit_char((g),(35)));
    (cg_emit_char((g),(100)));
    (cg_emit_char((g),(101)));
    (cg_emit_char((g),(102)));
    (cg_emit_char((g),(105)));
    (cg_emit_char((g),(110)));
    (cg_emit_char((g),(101)));
    (cg_emit_space((g)));
    (cg_emit_cstr((g),(ast_data1((node)))));
    (cg_emit_space((g)));
    (cg_emit_expr((g),(ast_data3((node)))));
    (cg_emit_nl((g)));
}

void cg_emit_module(long long g, long long node) {
    (cg_emit_header((g)));
    long long decls = (ast_data1((node)));
    long long count = (vec_len((decls)));
    long long i = (0);
    while (i < count) {
    long long decl = (vec_get((decls),(i)));
    if (ast_kind((decl)) == AST_CONST) {
    (cg_emit_const((g),(decl)));
    }
    (i) = (i + 1);
    }
    (cg_emit_nl((g)));
    (i) = (0);
    while (i < count) {
    long long decl = (vec_get((decls),(i)));
    if (ast_kind((decl)) == AST_STRUCT) {
    (cg_emit_struct((g),(decl)));
    }
    (i) = (i + 1);
    }
    (i) = (0);
    while (i < count) {
    long long decl = (vec_get((decls),(i)));
    if (ast_kind((decl)) == AST_FUNC) {
    (cg_emit_func((g),(decl)));
    }
    (i) = (i + 1);
    }
}

long long generate(long long ast) {
    long long g = (codegen_new());
    (cg_emit_module((g),(ast)));
    (cg_emit_char((g),(0)));
    return (cg_out((g)));
}

// already imported: stdlib/std.aether
long long str_len(long long s) {
    long long i = (0);
    while (ae_load8((s + i)) != 0) {
    (i) = (i + 1);
    }
    return (i);
}

long long get_test_source(void) {
    long long src = (ae_malloc((256)));
    long long i = (0);
    (ae_store8((src + i),(115)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(114)));
    (i) = (i + 1);
    (ae_store8((src + i),(117)));
    (i) = (i + 1);
    (ae_store8((src + i),(99)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(80)));
    (i) = (i + 1);
    (ae_store8((src + i),(111)));
    (i) = (i + 1);
    (ae_store8((src + i),(105)));
    (i) = (i + 1);
    (ae_store8((src + i),(110)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(123)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(120)));
    (i) = (i + 1);
    (ae_store8((src + i),(58)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(73)));
    (i) = (i + 1);
    (ae_store8((src + i),(110)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(44)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(121)));
    (i) = (i + 1);
    (ae_store8((src + i),(58)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(73)));
    (i) = (i + 1);
    (ae_store8((src + i),(110)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(125)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(102)));
    (i) = (i + 1);
    (ae_store8((src + i),(117)));
    (i) = (i + 1);
    (ae_store8((src + i),(110)));
    (i) = (i + 1);
    (ae_store8((src + i),(99)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(109)));
    (i) = (i + 1);
    (ae_store8((src + i),(97)));
    (i) = (i + 1);
    (ae_store8((src + i),(105)));
    (i) = (i + 1);
    (ae_store8((src + i),(110)));
    (i) = (i + 1);
    (ae_store8((src + i),(40)));
    (i) = (i + 1);
    (ae_store8((src + i),(41)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(45)));
    (i) = (i + 1);
    (ae_store8((src + i),(62)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(73)));
    (i) = (i + 1);
    (ae_store8((src + i),(110)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(123)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(108)));
    (i) = (i + 1);
    (ae_store8((src + i),(101)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(120)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(61)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(52)));
    (i) = (i + 1);
    (ae_store8((src + i),(50)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(105)));
    (i) = (i + 1);
    (ae_store8((src + i),(102)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(120)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(62)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(48)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(123)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(112)));
    (i) = (i + 1);
    (ae_store8((src + i),(114)));
    (i) = (i + 1);
    (ae_store8((src + i),(105)));
    (i) = (i + 1);
    (ae_store8((src + i),(110)));
    (i) = (i + 1);
    (ae_store8((src + i),(116)));
    (i) = (i + 1);
    (ae_store8((src + i),(40)));
    (i) = (i + 1);
    (ae_store8((src + i),(120)));
    (i) = (i + 1);
    (ae_store8((src + i),(41)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(125)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(32)));
    (i) = (i + 1);
    (ae_store8((src + i),(48)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(125)));
    (i) = (i + 1);
    (ae_store8((src + i),(10)));
    (i) = (i + 1);
    (ae_store8((src + i),(0)));
    return (src);
}

long long compile_full(long long src, long long len) {
    long long tokens = (tokenize((src),(len)));
    long long tok_count = (vec_len((tokens)));
    (print((tok_count)));
    long long ast = (parse((tokens)));
    long long type_ok = (check_module((ast)));
    long long borrow_ok = (check_ownership((ast)));
    long long effects_ok = (verify_effects((ast)));
    long long c_code = (generate((ast)));
    return (c_code);
}

long long main(void) {
    long long test_src = (get_test_source());
    long long len = (str_len((test_src)));
    (print((len)));
    long long result = (compile_full((test_src),(len)));
    (println((result)));
    return (0);
}

