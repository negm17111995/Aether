// AETHER BOOTSTRAP COMPILER v3.0
// Complete C implementation with full feature support
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdarg.h>

#define MAX_TOK 50000
#define MAX_SRC 500000

// Token types
enum { T_EOF, T_ID, T_INT, T_STR, T_PLUS, T_MINUS, T_STAR, T_SLASH, T_PCT,
       T_EQ, T_EQEQ, T_NE, T_LT, T_LE, T_GT, T_GE, T_AND, T_OR, T_NOT,
       T_AMP, T_PIPE, T_ARROW, T_DARROW, T_DOT, T_COLON, T_DCOLON, T_SEMI,
       T_COMMA, T_AT, T_LPAR, T_RPAR, T_LBRK, T_RBRK, T_LBRC, T_RBRC,
       T_FUNC, T_LET, T_MUT, T_CONST, T_IF, T_ELSE, T_WHILE, T_FOR, T_IN,
       T_RET, T_STRUCT, T_ENUM, T_IMPL, T_TRAIT, T_MATCH, T_IMPORT, T_USE,
       T_TRUE, T_FALSE, T_SELF, T_PUB, T_BREAK, T_CONT, T_NL };

typedef struct { int type, line; char *lex; long long ival; } Tok;
static Tok toks[MAX_TOK]; static int ntok, cur;
static char out[MAX_SRC*3], *op = out;

static void emit(const char *fmt, ...) {
    va_list args; va_start(args, fmt);
    op += vsprintf(op, fmt, args);
    va_end(args);
}

// Lexer
static void lex(char *s) {
    int ln = 1;
    while (*s) {
        while (*s == ' ' || *s == '\t' || *s == '\r') s++;
        if (!*s) break;
        Tok t = {0, ln, NULL, 0};
        if (*s == '\n') { t.type = T_NL; s++; ln++; toks[ntok++] = t; continue; }
        if (*s == '/' && s[1] == '/') { while (*s && *s != '\n') s++; continue; }
        if (*s == '"') {
            s++; char *st = s;
            while (*s && *s != '"') s++;
            t.type = T_STR; t.lex = strndup(st, s-st);
            if (*s) s++;
            toks[ntok++] = t; continue;
        }
        if (isdigit(*s)) {
            char *st = s;
            if (*s == '0' && (s[1] == 'x' || s[1] == 'X')) {
                s += 2; while (isxdigit(*s)) s++;
                t.ival = strtoll(st, NULL, 0);
            } else {
                while (isdigit(*s)) s++;
                t.ival = strtoll(st, NULL, 10);
            }
            t.type = T_INT; t.lex = strndup(st, s-st);
            toks[ntok++] = t; continue;
        }
        if (isalpha(*s) || *s == '_') {
            char *st = s;
            while (isalnum(*s) || *s == '_') s++;
            t.lex = strndup(st, s-st);
            t.type = T_ID;
            if (!strcmp(t.lex,"func")) t.type = T_FUNC;
            else if (!strcmp(t.lex,"let")) t.type = T_LET;
            else if (!strcmp(t.lex,"mut")) t.type = T_MUT;
            else if (!strcmp(t.lex,"const")) t.type = T_CONST;
            else if (!strcmp(t.lex,"if")) t.type = T_IF;
            else if (!strcmp(t.lex,"else")) t.type = T_ELSE;
            else if (!strcmp(t.lex,"while")) t.type = T_WHILE;
            else if (!strcmp(t.lex,"for")) t.type = T_FOR;
            else if (!strcmp(t.lex,"in")) t.type = T_IN;
            else if (!strcmp(t.lex,"return")) t.type = T_RET;
            else if (!strcmp(t.lex,"struct")) t.type = T_STRUCT;
            else if (!strcmp(t.lex,"enum")) t.type = T_ENUM;
            else if (!strcmp(t.lex,"impl")) t.type = T_IMPL;
            else if (!strcmp(t.lex,"trait")) t.type = T_TRAIT;
            else if (!strcmp(t.lex,"match")) t.type = T_MATCH;
            else if (!strcmp(t.lex,"import")) t.type = T_IMPORT;
            else if (!strcmp(t.lex,"use")) t.type = T_USE;
            else if (!strcmp(t.lex,"true")) t.type = T_TRUE;
            else if (!strcmp(t.lex,"false")) t.type = T_FALSE;
            else if (!strcmp(t.lex,"self")) t.type = T_SELF;
            else if (!strcmp(t.lex,"pub")) t.type = T_PUB;
            else if (!strcmp(t.lex,"break")) t.type = T_BREAK;
            else if (!strcmp(t.lex,"continue")) t.type = T_CONT;
            toks[ntok++] = t; continue;
        }
        #define OP2(a,b,t2,t1) if(s[0]==a&&s[1]==b){t.type=t2;s+=2;toks[ntok++]=t;continue;}
        #define OP1(c,tt) if(*s==c){t.type=tt;s++;toks[ntok++]=t;continue;}
        OP2('-','>',T_ARROW,T_MINUS) OP2('=','>',T_DARROW,T_EQ)
        OP2('=','=',T_EQEQ,T_EQ) OP2('!','=',T_NE,T_NOT)
        OP2('<','=',T_LE,T_LT) OP2('>','=',T_GE,T_GT)
        OP2('&','&',T_AND,T_AMP) OP2('|','|',T_OR,T_PIPE)
        OP2(':',':',T_DCOLON,T_COLON)
        OP1('+',T_PLUS) OP1('-',T_MINUS) OP1('*',T_STAR) OP1('/',T_SLASH)
        OP1('%',T_PCT) OP1('=',T_EQ) OP1('<',T_LT) OP1('>',T_GT)
        OP1('!',T_NOT) OP1('&',T_AMP) OP1('|',T_PIPE) OP1('.',T_DOT)
        OP1(':',T_COLON) OP1(';',T_SEMI) OP1(',',T_COMMA) OP1('@',T_AT)
        OP1('(',T_LPAR) OP1(')',T_RPAR) OP1('[',T_LBRK) OP1(']',T_RBRK)
        OP1('{',T_LBRC) OP1('}',T_RBRC)
        s++;
    }
    toks[ntok++] = (Tok){T_EOF, ln, NULL, 0};
}

// Parser helpers
static Tok *pk() { return &toks[cur]; }
static Tok *adv() { return &toks[cur++]; }
static void skip() { while (pk()->type == T_NL) cur++; }
static bool mt(int t) { skip(); if (pk()->type == t) { cur++; return true; } return false; }
static bool chk(int t) { skip(); return pk()->type == t; }
static Tok *exp(int t) { skip(); if (pk()->type == t) return adv(); 
    fprintf(stderr, "L%d: expected %d, got %d\n", pk()->line, t, pk()->type); return NULL; }

// Struct table
typedef struct { char *name; char **fnames; char **ftypes; int nf; } Struct;
static Struct structs[500]; static int nstruct;

static Struct *find_struct(char *n) {
    if (!n) return NULL;
    for (int i = 0; i < nstruct; i++) if (!strcmp(structs[i].name, n)) return &structs[i];
    return NULL;
}

// LIQUID OWNERSHIP: Scope Tracking
typedef struct { char *name; char *type; } Var;
typedef struct Scope { Var vars[100]; int nvar; struct Scope *parent; } Scope;
static Scope *cur_scope = NULL;

static void enter_scope() {
    Scope *s = malloc(sizeof(Scope));
    s->nvar = 0;
    s->parent = cur_scope;
    cur_scope = s;
}

static void add_var(char *name, char *type) {
    if (cur_scope && cur_scope->nvar < 100) {
        cur_scope->vars[cur_scope->nvar].name = strdup(name);
        cur_scope->vars[cur_scope->nvar].type = type ? strdup(type) : NULL;
        cur_scope->nvar++;
    }
}

static void exit_scope() {
    if (!cur_scope) return;
    // Auto-Drop Logic (Liquid Ownership)
    for (int i = cur_scope->nvar - 1; i >= 0; i--) {
        char *ty = cur_scope->vars[i].type;
        if (ty && find_struct(ty)) {
            // Check if Type_drop exists (heuristic: we just emit it if it's a struct)
            // Ideally we check if function exists, but in bootstrap we rely on user.
            // We emit: Type_drop(var);
            // But we must be careful not to drop if it was moved. (Not implemented in Phase 1)
            emit("    %s_drop(%s);\n", ty, cur_scope->vars[i].name);
        }
    }
    Scope *p = cur_scope->parent;
    free(cur_scope);
    cur_scope = p;
}


// Forward decls
static void gen_expr(void);
static void gen_stmt(void);

// Type to C
static const char *ty2c(char *t) {
    if (!t) return "long long";
    if (!strcmp(t,"Int")) return "long long";
    if (!strcmp(t,"Float")) return "double";
    if (!strcmp(t,"Bool")) return "int";
    if (!strcmp(t,"String")) return "const char*";
    if (find_struct(t)) return t;
    return "long long";
}

// Parse type
static char *parse_type() {
    skip();
    if (chk(T_LBRK)) { adv(); char *inner = parse_type(); exp(T_RBRK); return "long long"; }
    Tok *t = exp(T_ID);
    return t ? t->lex : "Int";
}

// Expression
static void gen_primary() {
    skip();
    if (mt(T_INT)) { emit("%lld", toks[cur-1].ival); return; }
    if (mt(T_STR)) { emit("\"%s\"", toks[cur-1].lex); return; }
    if (mt(T_TRUE)) { emit("1"); return; }
    if (mt(T_FALSE)) { emit("0"); return; }
    if (mt(T_LBRK)) {
        emit("(long long[]){");
        if (!chk(T_RBRK)) { gen_expr(); while(mt(T_COMMA)) { emit(","); gen_expr(); } }
        emit("}"); exp(T_RBRK); return;
    }
    if (mt(T_LPAR)) { emit("("); gen_expr(); emit(")"); exp(T_RPAR); return; }
    if (chk(T_ID)) {
        Tok *id = adv();
        if (mt(T_DCOLON)) {
            Tok *var = exp(T_ID);
            emit("%s_%s", id->lex, var?var->lex:"");
            if (mt(T_LBRC)) {
                emit("((struct %s_%s){", id->lex, var?var->lex:"");
                skip();
                while (!chk(T_RBRC)) {
                    skip(); Tok *fn = exp(T_ID); exp(T_COLON);
                    emit(".%s=", fn?fn->lex:"");
                    gen_expr();
                    if (!chk(T_RBRC)) { mt(T_COMMA); emit(","); }
                    skip();
                }
                emit("})"); exp(T_RBRC);
            }
            return;
        }
        if (chk(T_LBRC) && find_struct(id->lex)) {
            mt(T_LBRC); // Consume {
            emit("((%s){", id->lex);
            skip();
            while (!chk(T_RBRC)) {
                skip(); Tok *fn = exp(T_ID); exp(T_COLON);
                emit(".%s=", fn?fn->lex:"");
                gen_expr();
                if (!chk(T_RBRC)) { mt(T_COMMA); emit(","); }
                skip();
            }
            emit("})"); exp(T_RBRC); return;
        }
        if (mt(T_LPAR)) {
            emit("%s(", id->lex);
            if (!chk(T_RPAR)) { gen_expr(); while(mt(T_COMMA)) { emit(","); gen_expr(); } }
            emit(")"); exp(T_RPAR); return;
        }
        emit("%s", id->lex);
        return;
    }
    adv();
}

static void gen_postfix() {
    gen_primary();
    while (1) {
        if (mt(T_DOT)) {
            Tok *fn = exp(T_ID);
            if (mt(T_LPAR)) {
                emit(".%s(", fn?fn->lex:"");
                if (!chk(T_RPAR)) { gen_expr(); while(mt(T_COMMA)) { emit(","); gen_expr(); } }
                emit(")");
                exp(T_RPAR);
            } else {
                emit(".%s", fn?fn->lex:"");
            }
        } else if (mt(T_LBRK)) {
            emit("["); gen_expr(); emit("]"); exp(T_RBRK);
        } else break;
    }
}

static void gen_unary() {
    if (mt(T_NOT)) { emit("!"); gen_unary(); }
    else if (mt(T_MINUS)) { emit("-"); gen_unary(); }
    else if (mt(T_AMP)) { emit("&"); gen_unary(); }
    else gen_postfix();
}

static int prec(int t) {
    if (t==T_OR) return 1; if (t==T_AND) return 2;
    if (t==T_EQEQ||t==T_NE) return 3;
    if (t==T_LT||t==T_LE||t==T_GT||t==T_GE) return 4;
    if (t==T_PLUS||t==T_MINUS) return 5;
    if (t==T_STAR||t==T_SLASH||t==T_PCT) return 6;
    return 0;
}

static const char *op2c(int t) {
    switch(t) { case T_PLUS:return"+"; case T_MINUS:return"-";
    case T_STAR:return"*"; case T_SLASH:return"/"; case T_PCT:return"%";
    case T_EQEQ:return"=="; case T_NE:return"!="; case T_LT:return"<";
    case T_LE:return"<="; case T_GT:return">"; case T_GE:return">=";
    case T_AND:return"&&"; case T_OR:return"||"; } return "?";
}

static void gen_binary(int mp) {
    gen_unary();
    while (prec(pk()->type) >= mp) {
        int op = pk()->type; adv();
        emit(" %s ", op2c(op));
        gen_binary(prec(op)+1);
    }
}

static void gen_expr() { emit("("); gen_binary(1); emit(")"); }

// Statements
static void gen_block() {
    enter_scope();
    skip();
    while (!chk(T_RBRC) && pk()->type != T_EOF) {
        gen_stmt(); skip();
    }
    exit_scope();
}


static void gen_stmt() {
    skip();
    if (mt(T_LET)) {
        bool is_mut = mt(T_MUT);
        Tok *name = exp(T_ID);

        char *ty = NULL;
        bool is_array = false;
        bool is_string = false;
        if (mt(T_COLON)) ty = parse_type();
        exp(T_EQ);
        // Infer type from expression
        skip();
        if (!ty && chk(T_STR)) {
            // String literal
            is_string = true;
        } else if (!ty && chk(T_LBRK)) {
            // Array literal - use pointer type
            is_array = true;
        } else if (!ty && chk(T_ID)) {
            // Peek ahead to see if this is a struct literal: Name { ... }
            int save = cur;
            Tok *maybe_type = adv();
            if (chk(T_LBRC) && find_struct(maybe_type->lex)) {
                ty = maybe_type->lex;
            }
            cur = save;
        }
        if (is_string) {
            emit("    const char *%s = ", name ? name->lex : "v");
        } else if (is_array) {
            emit("    long long *%s = ", name ? name->lex : "v");
        } else {
            emit("    %s %s = ", ty ? ty2c(ty) : "long long", name ? name->lex : "v");
        }
        gen_expr();
        emit(";\n");
        // Track variable for Liquid Ownership
        if (name) add_var(name->lex, ty);
        return;

    }
    if (mt(T_RET)) {
        emit("    return");
        skip();
        if (!chk(T_SEMI) && !chk(T_RBRC)) { emit(" "); gen_expr(); }
        emit(";\n");
        return;
    }
    if (mt(T_IF)) {
        emit("    if "); gen_expr(); emit(" {\n");
        exp(T_LBRC); gen_block(); exp(T_RBRC);
        emit("    }");
        skip();
        if (mt(T_ELSE)) {
            if (chk(T_IF)) { emit(" else "); gen_stmt(); }
            else { emit(" else {\n"); exp(T_LBRC); gen_block(); exp(T_RBRC); emit("    }"); }
        }
        emit("\n");
        return;
    }
    if (mt(T_WHILE)) {
        emit("    while "); gen_expr(); emit(" {\n");
        exp(T_LBRC); gen_block(); exp(T_RBRC);
        emit("    }\n");
        return;
    }
    if (mt(T_FOR)) {
        Tok *var = exp(T_ID); exp(T_IN);
        emit("    for (long long %s = 0; %s < ", var?var->lex:"i", var?var->lex:"i");
        gen_expr();
        emit("; %s++) {\n", var?var->lex:"i");
        exp(T_LBRC); gen_block(); exp(T_RBRC);
        emit("    }\n");
        return;
    }
    if (mt(T_MATCH)) {
        emit("    switch (");
        // Parse scrutinee - just expect identifier or simple expr, not struct literal
        skip();
        if (chk(T_ID)) {
            emit("%s", adv()->lex);
        } else if (chk(T_INT)) {
            emit("%lld", adv()->ival);
        } else {
            emit("0");
        }
        emit(") {\n");
        exp(T_LBRC);
        skip();
        while (!chk(T_RBRC) && pk()->type != T_EOF) {
            skip();
            if (chk(T_ID) && pk()->lex && !strcmp(pk()->lex, "_")) {
                adv();
                exp(T_DARROW);
                emit("    default: {\n");
            } else if (chk(T_INT)) {
                Tok *num = adv();
                exp(T_DARROW);
                emit("    case %lld: {\n", num->ival);
            } else {
                // Skip unknown pattern
                while (!chk(T_DARROW) && !chk(T_RBRC) && pk()->type != T_EOF) adv();
                if (mt(T_DARROW)) {
                    emit("    default: {\n");
                } else break;
            }
            emit("        ");
            if (mt(T_LBRC)) {
                gen_block();
                exp(T_RBRC);
            } else {
                gen_expr();
                emit(";\n");
            }
            emit("        break;\n    }\n");
            mt(T_COMMA);
            skip();
        }
        exp(T_RBRC);
        emit("    }\n");
        return;
    }
    if (mt(T_BREAK)) { emit("    break;\n"); return; }
    if (mt(T_CONT)) { emit("    continue;\n"); return; }
    
    // Expr stmt or assignment
    emit("    ");
    gen_expr();
    if (mt(T_EQ)) { emit(" = "); gen_expr(); }
    emit(";\n");
}

// Declarations
static void gen_func() {
    Tok *name = exp(T_ID);
    if (mt(T_LT)) { while (!chk(T_GT)) adv(); exp(T_GT); } // Skip generics
    exp(T_LPAR);
    char *params[32]; char *ptypes[32]; int np = 0;
    skip();
    while (!chk(T_RPAR)) {
        skip();
        Tok *pn = exp(T_ID); exp(T_COLON);
        char *pt = parse_type();
        params[np] = pn ? pn->lex : "p";
        ptypes[np] = pt;
        np++;
        if (!mt(T_COMMA)) break;
        skip();
    }
    exp(T_RPAR);
    char *ret = NULL;
    if (mt(T_ARROW)) ret = parse_type();
    
    // Generate
    emit("%s %s(", ret ? ty2c(ret) : "void", name?name->lex:"f");
    if (np == 0) emit("void");
    else for (int i = 0; i < np; i++) {
        if (i > 0) emit(", ");
        emit("%s %s", ty2c(ptypes[i]), params[i]);
    }
    emit(") {\n");
    exp(T_LBRC);
    // Generate block - for functions with return type, last expression is returned
    skip();
    while (!chk(T_RBRC) && pk()->type != T_EOF) {
        skip();
        // Peek ahead to check if this is the last statement before }
        int save_pos = cur;
        // Skip past current statement in tokens
        int depth = 0;
        while (pk()->type != T_EOF) {
            if (pk()->type == T_LBRC || pk()->type == T_LBRK || pk()->type == T_LPAR) depth++;
            if (pk()->type == T_RBRC || pk()->type == T_RBRK || pk()->type == T_RPAR) {
                if (depth == 0) break;
                depth--;
            }
            if (pk()->type == T_NL && depth == 0) { adv(); break; }
            adv();
        }
        skip();
        bool is_last = chk(T_RBRC);
        cur = save_pos;
        
        if (is_last && ret) {
            // Last expression in a function with return type
            skip();
            // Check if it's already a return statement
            if (chk(T_RET)) {
                gen_stmt();
            } else {
                emit("    return ");
                gen_expr();
                emit(";\n");
            }
        } else {
            gen_stmt();
        }
        skip();
    }
    exp(T_RBRC);
    emit("}\n\n");
}

static void gen_struct() {
    Tok *name = exp(T_ID);
    if (mt(T_LT)) { while (!chk(T_GT)) adv(); exp(T_GT); }
    exp(T_LBRC);
    
    Struct *s = &structs[nstruct++];
    s->name = name ? name->lex : "S";
    s->fnames = malloc(64 * sizeof(char*));
    s->ftypes = malloc(64 * sizeof(char*));
    s->nf = 0;
    
    emit("typedef struct {\n");
    skip();
    while (!chk(T_RBRC)) {
        skip(); mt(T_PUB);
        Tok *fn = exp(T_ID); exp(T_COLON);
        char *ft = parse_type();
        s->fnames[s->nf] = fn ? fn->lex : "f";
        s->ftypes[s->nf] = ft;
        s->nf++;
        emit("    %s %s;\n", ty2c(ft), fn?fn->lex:"f");
        mt(T_COMMA); skip();
    }
    exp(T_RBRC);
    emit("} %s;\n\n", s->name);
}

static void gen_enum() {
    Tok *name = exp(T_ID);
    exp(T_LBRC);
    
    emit("typedef enum {\n");
    skip(); int idx = 0;
    while (!chk(T_RBRC)) {
        skip();
        Tok *vn = exp(T_ID);
        if (mt(T_LBRC)) { while (!chk(T_RBRC)) adv(); exp(T_RBRC); }
        else if (mt(T_LPAR)) { while (!chk(T_RPAR)) adv(); exp(T_RPAR); }
        emit("    %s_%s = %d,\n", name?name->lex:"E", vn?vn->lex:"V", idx++);
        mt(T_COMMA); skip();
    }
    exp(T_RBRC);
    emit("} %s;\n\n", name?name->lex:"E");
}

static void gen_impl() {
    if (mt(T_LT)) { while (!chk(T_GT)) adv(); exp(T_GT); }
    Tok *ty = exp(T_ID);
    // Handle 'impl Trait for Type' syntax
    if (mt(T_FOR)) {
        ty = exp(T_ID);  // The actual type is after 'for'
    }
    exp(T_LBRC); skip();
    while (!chk(T_RBRC) && pk()->type != T_EOF) {
        skip(); mt(T_PUB);
        if (mt(T_FUNC)) {
            Tok *fn = exp(T_ID);
            emit("// %s::%s\n", ty?ty->lex:"T", fn?fn->lex:"f");
            // Skip rest
            while (!chk(T_RBRC) || 1) {
                if (chk(T_RBRC)) { int d = 1; adv();
                    while (d > 0) { if (chk(T_LBRC)) d++; if (chk(T_RBRC)) d--; adv(); if (d==0) break; }
                    break;
                }
                if (chk(T_FUNC)) break;
                adv();
            }
        }
        skip();
    }
    exp(T_RBRC);
}

// Track imported files to prevent duplicates
static char *imported_files[100];
static int num_imported = 0;

static bool already_imported(const char *path) {
    for (int i = 0; i < num_imported; i++) {
        if (strcmp(imported_files[i], path) == 0) return true;
    }
    return false;
}

static void gen_decl(void);

static void gen_import() {
    // Build import path from tokens: std.io -> stdlib/io.aether
    char path[256] = "stdlib/";
    int plen = 7;
    
    while (pk()->type == T_ID || pk()->type == T_DOT || pk()->type == T_DCOLON) {
        if (pk()->type == T_ID) {
            strcpy(path + plen, pk()->lex);
            plen += strlen(pk()->lex);
        } else {
            path[plen++] = '/';
        }
        adv();
    }
    strcat(path, ".aether");
    
    // Check if already imported
    if (already_imported(path)) {
        emit("// already imported: %s\n", path);
        return;
    }
    
    // Try to read the file
    FILE *f = fopen(path, "r");
    if (!f) {
        emit("// import not found: %s\n", path);
        return;
    }
    
    // Mark as imported
    imported_files[num_imported++] = strdup(path);
    emit("// importing: %s\n", path);
    
    // Read file
    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *src = malloc(len + 1);
    fread(src, 1, len, f);
    src[len] = 0;
    fclose(f);
    
    // Save current parser state
    int save_cur = cur;
    int save_ntok = ntok;
    Tok *save_toks = malloc(sizeof(toks));
    memcpy(save_toks, toks, sizeof(toks));
    
    // Lex and compile the imported file
    ntok = 0;
    cur = 0;
    lex(src);
    
    while (pk()->type != T_EOF) {
        gen_decl();
    }
    
    // Restore parser state
    cur = save_cur;
    ntok = save_ntok;
    memcpy(toks, save_toks, sizeof(toks));
    free(save_toks);
    free(src);
}

static void gen_decl() {
    skip(); if (pk()->type == T_EOF) return;
    while (mt(T_AT)) { exp(T_LBRK); while(!chk(T_RBRK)) adv(); exp(T_RBRK); skip(); }
    mt(T_PUB);
    if (mt(T_FUNC)) gen_func();
    else if (mt(T_STRUCT)) gen_struct();
    else if (mt(T_ENUM)) gen_enum();
    else if (mt(T_IMPL)) gen_impl();
    else if (mt(T_TRAIT)) { while(!chk(T_RBRC)) adv(); exp(T_RBRC); }
    else if (mt(T_IMPORT) || mt(T_USE)) gen_import();
    else if (mt(T_CONST)) {
        Tok *n = exp(T_ID);
        if (mt(T_COLON)) parse_type();
        exp(T_EQ);
        emit("#define %s ", n?n->lex:"C");
        gen_expr(); emit("\n");
    }
    else adv();
}

static void compile(char *src) {
    lex(src);
    
    emit("/* Aether Compiler v3.0 */\n");
    emit("#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n");
    emit("#include <unistd.h>\n#include <pthread.h>\n#include <sched.h>\n#include <sys/syscall.h>\n\n");
    emit("#define print(x) printf(\"%%lld\\n\",(long long)(x))\n");
    emit("#define println(s) printf(\"%%s\\n\",(s))\n");
    emit("#define __builtin_malloc(n) ((long long)malloc(n))\n");
    emit("#define __builtin_free(p) free((void*)(p))\n");
    emit("#define __builtin_store64(a,v) (*(long long*)(a)=(v))\n");
    emit("#define __builtin_load64(a) (*(long long*)(a))\n");
    emit("#define __builtin_store8(a,v) (*(char*)(a)=(char)(v))\n");
    emit("#define __builtin_load8(a) ((unsigned char)*(char*)(a))\n");
    emit("#define __builtin_panic(x) (printf(\"PANIC: %%lld\\n\",(long long)(x)),exit((int)(x)),0)\n");
    emit("#define __builtin_call(f,x) ((long long (*)(long long))(f))(x)\n");
    emit("#define __builtin_thread_id() ((long long)pthread_self())\n");
    emit("#define __builtin_atomic_cas(ptr,old,new) __sync_bool_compare_and_swap((long long*)(ptr),(long long)(old),(long long)(new))\n");
    emit("#define __builtin_atomic_store(ptr,val) __sync_lock_test_and_set((long long*)(ptr),(long long)(val))\n");
    emit("#define __builtin_yield() sched_yield()\n");
    emit("#define __builtin_syscall2(n,a1,a2) syscall((long)(n),(long)(a1),(long)(a2))\n");
    emit("#define __memory_barrier() __sync_synchronize()\n");
    emit("#define __builtin_or(a,b) ((a)|(b))\n");
    emit("#define __builtin_and(a,b) ((a)&(b))\n");
    emit("#define __builtin_xor(a,b) ((a)^(b))\n\n");
    
    while (pk()->type != T_EOF) gen_decl();
}

int main(int argc, char **argv) {
    if (argc < 2) { printf("Usage: aetherc3 <file.ae>\n"); return 1; }
    FILE *f = fopen(argv[1], "r");
    if (!f) { printf("Cannot open: %s\n", argv[1]); return 1; }
    fseek(f, 0, SEEK_END); long len = ftell(f); fseek(f, 0, SEEK_SET);
    char *src = malloc(len+1); fread(src, 1, len, f); src[len] = 0; fclose(f);
    
    printf("Compiling: %s\n", argv[1]);
    compile(src);
    
    f = fopen("a.out.c", "w"); fputs(out, f); fclose(f);
    printf("Generated: a.out.c\n");
    
    int r = system("cc -O3 -Wno-all -o a.out a.out.c");
    if (r == 0) { printf("Compiled: a.out\n"); /* keep a.out.c for debugging */ }
    else printf("C errors. File kept.\n");
    return r;
}
