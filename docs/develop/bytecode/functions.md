# Funções no Bytecode VM

## Visão Geral

O Bytecode VM do Dryad agora suporta **funções completas**, incluindo:
- ✅ Declaração de funções
- ✅ Chamadas de função
- ✅ Return de valores
- ✅ Parâmetros e argumentos
- ✅ Variáveis locais em funções
- ✅ Recursão
- ✅ Verificação de aridade

## Implementação

### 1. Tipos de Valor

Adicionamos dois novos tipos ao sistema de valores:

```rust
pub enum Value {
    // ... tipos existentes ...
    
    /// Função definida pelo usuário
    Function(Rc<Function>),
    
    /// Função nativa (do runtime)
    NativeFunction(NativeFn),
}

pub struct Function {
    pub name: String,
    pub arity: usize,           // Número de parâmetros
    pub chunk: Chunk,          // Bytecode da função
    pub upvalue_count: usize,  // Para closures
}
```

### 2. Opcodes de Função

```rust
pub enum OpCode {
    // Chama uma função (número de argumentos)
    Call(u8),
    
    // Retorna de uma função
    Return,
    
    // Cria uma closure (função + upvalues)
    Closure(u8),
    
    // Upvalues (para closures)
    GetUpvalue(u8),
    SetUpvalue(u8),
    CloseUpvalue,
}
```

### 3. Compilação

O compilador converte `FunctionDeclaration` AST para bytecode:

```rust
fn compile_function_declaration(
    &mut self,
    name: String,
    params: Vec<(String, Option<Type>)>,
    body: Stmt,
    line: usize,
) -> Result<(), String> {
    // 1. Salva o estado atual
    let enclosing_chunk = std::mem::replace(&mut self.current_chunk, Chunk::new(&name));
    
    // 2. Configura novo escopo
    self.scope_depth = 1;
    self.locals.clear();
    
    // 3. Adiciona parâmetros como variáveis locais
    for (param_name, _) in &params {
        self.add_local(param_name.clone());
    }
    
    // 4. Compila o corpo
    self.compile_statement(body)?;
    
    // 5. Garante retorno
    self.emit_op(OpCode::Nil, line);
    self.emit_op(OpCode::Return, line);
    
    // 6. Cria a função
    let function_chunk = std::mem::replace(&mut self.current_chunk, enclosing_chunk);
    let function = Function {
        name: name.clone(),
        arity: params.len(),
        chunk: function_chunk,
        upvalue_count: 0,
    };
    
    // 7. Restaura estado e define a função
    // ...
}
```

### 4. Execução na VM

#### Call

```rust
OpCode::Call(arg_count) => {
    let callee = self.peek(*arg_count as usize)?;
    
    match callee {
        Value::Function(function) => {
            self.call_function(Rc::clone(function), *arg_count)?;
        }
        Value::NativeFunction(native_fn) => {
            self.call_native(*native_fn, *arg_count)?;
        }
        _ => {
            return Err(format!("Não é possível chamar '{}'", callee.type_name()));
        }
    }
}
```

#### Chamada de Função do Usuário

```rust
fn call_function(&mut self, function: Rc<Function>, arg_count: u8) -> Result<(), String> {
    // Verifica aridade
    if function.arity != arg_count as usize {
        return Err(format!(
            "Função {} espera {} argumentos, mas recebeu {}",
            function.name, function.arity, arg_count
        ));
    }

    // Verifica limite de recursão
    if self.frames.len() >= self.max_frames {
        return Err("Stack overflow".to_string());
    }

    // Calcula onde os argumentos começam
    let stack_start = self.stack.len() - arg_count as usize - 1;

    // Cria novo frame
    self.frames.push(CallFrame::new(
        function.chunk.clone(), 
        stack_start
    ));

    Ok(())
}
```

#### Return

```rust
OpCode::Return => {
    // Pega o valor de retorno
    let result = self.pop()?;
    
    // Remove argumentos e função da pilha
    let frame = self.frames.pop().ok_or("Não há frame para retornar")?;
    while self.stack.len() > frame.stack_start {
        self.stack.pop();
    }
    
    // Empilha o resultado
    self.push(result);
    
    return Ok(ExecutionControl::Return);
}
```

## Exemplo de Bytecode

### Código Fonte

```dryad
fn add(a, b) {
    return a + b;
}

print add(10, 20);
```

### Bytecode Gerado

```
== add ==
Constants:
  [   0] 'a'
  [   1] 'b'

Bytecode:
0000    1 GET_LOCAL      0    # carrega 'a'
0002    1 GET_LOCAL      1    # carrega 'b'
0004    1 ADD
0005    1 RETURN
0006    1 NIL
0007    1 RETURN

== script ==
Constants:
  [   0] <fn add>

Bytecode:
0000    1 CONSTANT       0    # carrega função 'add'
0002    2 CONSTANT       1    # carrega 10
0004    2 CONSTANT       2    # carrega 20
0006    2 CALL           2    # chama add(10, 20)
0008    2 PRINT_LN
...
```

## Stack Frame

Durante a execução de uma função, a pilha fica assim:

```
Pilha durante execução de add(10, 20):

[Frame anterior]
  ...
  [ add ]          <- Função
  [ 10 ]           <- Arg 0 (a)
  [ 20 ]           <- Arg 1 (b)
  [ resultado ]    <- Return value (após execução)
```

## Testes

Arquivo: `crates/dryad_bytecode/tests/function_tests.rs`

```rust
#[test]
fn test_function_call() {
    let program = Program {
        statements: vec![
            // fn add(a, b) { return a + b; }
            Stmt::FunctionDeclaration { ... },
            // print add(1, 2);
            Stmt::Print(Expr::Call(...)),
        ],
    };

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(program).unwrap();

    let mut vm = VM::new();
    let result = vm.interpret(chunk);
    assert_eq!(result, InterpretResult::Ok);
}
```

## Exemplos de Uso

### Exemplo 1: Função Simples

```dryad
fn greet(nome) {
    print "Ola, " + nome;
}

greet("Mundo");
```

### Exemplo 2: Função com Return

```dryad
fn quadrado(x) {
    return x * x;
}

print quadrado(5);  # 25
```

### Exemplo 3: Recursão

```dryad
fn fatorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * fatorial(n - 1);
}

print fatorial(5);  # 120
```

### Exemplo 4: Variáveis Locais

```dryad
fn calcula(x, y) {
    var soma = x + y;
    var produto = x * y;
    return soma + produto;
}

print calcula(2, 3);  # (2+3) + (2*3) = 11
```

## Limitações Atuais

1. **Closures**: Upvalues implementados nos opcodes, mas não funcionais na VM ainda
2. **Métodos de classe**: Não implementado
3. **Funções anônimas/lambdas**: Não implementado
4. **Funções nativas**: Suporte básico, mas necessita integração com runtime

## Próximos Passos

1. **Implementar closures completos**
   - Captura de variáveis do escopo externo
   - Upvalues funcionais

2. **Integrar com runtime**
   - Permitir funções nativas do Dryad
   - Acesso a módulos e bibliotecas

3. **Otimizar chamadas de função**
   - Tail call optimization
   - Inline de funções pequenas

## Debugging

Para debugar funções no bytecode:

```bash
# Ver bytecode de funções
DRYAD_DEBUG_BYTECODE=1 dryad run script.dryad --compile

# Ver execução passo a passo
DRYAD_DEBUG_VM=1 dryad run script.dryad --compile
```

O disassembler mostra cada função separadamente com seu próprio chunk de bytecode.
