// crates/dryad_bytecode/src/opcode.rs
//! Definição dos opcodes da Máquina Virtual Dryad
//!
//! Este módulo define todas as instruções suportadas pela VM baseada em pilha.
//! Os opcodes são organizados por categoria para melhor organização e manutenção.

/// Opcode de uma instrução da VM
///
/// Cada opcode representa uma operação que a VM pode executar.
/// Os opcodes são armazenados em um chunk de bytecode e executados sequencialmente.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    // ============================================
    // Constantes (mais frequentes)
    // ============================================
    /// Carrega uma constante da tabela (índice de 8 bits)
    Constant(u8),
    /// Carrega uma constante da tabela (índice de 16 bits)
    ConstantLong(u16),
    /// Carrega nil na pilha
    Nil,
    /// Carrega true na pilha
    True,
    /// Carrega false na pilha
    False,

    // ============================================
    // Operações Aritméticas
    // ============================================
    /// Adição (+)
    Add,
    /// Subtração (-)
    Subtract,
    /// Multiplicação (*)
    Multiply,
    /// Divisão (/)
    Divide,
    /// Módulo (%)
    Modulo,
    /// Negação unária (-)
    Negate,

    // ============================================
    // Comparações
    // ============================================
    /// Igualdade (==)
    Equal,
    /// Diferença (!=) - implementado como Equal + Not
    /// Maior que (>)
    Greater,
    /// Menor que (<)
    Less,
    /// Maior ou igual (>=)
    GreaterEqual,
    /// Menor ou igual (<=)
    LessEqual,

    // ============================================
    // Operações Lógicas
    // ============================================
    /// Negação lógica (!)
    Not,
    /// AND lógico (&&)
    And,
    /// OR lógico (||)
    Or,

    // ============================================
    // Operações Bitwise
    // ============================================
    /// AND bitwise (&)
    BitAnd,
    /// OR bitwise (|)
    BitOr,
    /// XOR bitwise (^)
    BitXor,
    /// NOT bitwise (~)
    BitNot,
    /// Shift left (<<)
    ShiftLeft,
    /// Shift right (>>)
    ShiftRight,

    // ============================================
    // Variáveis Globais
    // ============================================
    /// Define uma variável global
    DefineGlobal(u8),
    /// Carrega uma variável global
    GetGlobal(u8),
    /// Atualiza uma variável global
    SetGlobal(u8),

    // ============================================
    // Variáveis Locais
    // ============================================
    /// Carrega uma variável local pelo índice
    GetLocal(u8),
    /// Atualiza uma variável local pelo índice
    SetLocal(u8),

    // ============================================
    // Controle de Fluxo
    // ============================================
    /// Pula para frente (offset de 16 bits)
    Jump(u16),
    /// Pula para frente se o topo da pilha for falso
    JumpIfFalse(u16),
    /// Pula para frente se o topo da pilha for verdadeiro
    JumpIfTrue(u16),
    /// Pula para trás (para loops) - offset de 16 bits
    Loop(u16),
    /// Break - sai de um loop
    Break,
    /// Continue - reinicia um loop
    Continue,

    // ============================================
    // Funções
    // ============================================
    /// Chama uma função (número de argumentos)
    Call(u8),
    /// Retorna de uma função
    Return,
    /// Cria uma closure (função + upvalues)
    Closure(u8),
    /// Carrega um upvalue
    GetUpvalue(u8),
    /// Atualiza um upvalue
    SetUpvalue(u8),
    /// Fecha upvalues até um certo índice
    CloseUpvalue,

    // ============================================
    // Classes e Objetos
    // ============================================
    /// Cria uma nova classe
    Class(u8),
    /// Define um método em uma classe
    Method(u8),
    /// Invoca um método (número de argumentos)
    Invoke(u8),
    /// Acessa uma propriedade
    GetProperty(u8),
    /// Define uma propriedade
    SetProperty(u8),
    /// Carrega 'this'
    This,
    /// Carrega 'super'
    Super(u8),

    // ============================================
    // Exceções
    // ============================================
    /// Inicia bloco try (offset para catch, offset para finally)
    TryBegin(u16, u16),
    /// Termina bloco try
    TryEnd,
    /// Lança uma exceção
    Throw,
    /// Cria objeto de exceção
    NewException(u8), // índice da mensagem
    /// Captura exceção em variável
    Catch(u8), // índice do nome da variável
    Super(u8),

    // ============================================
    // Arrays e Tuples
    // ============================================
    /// Cria um novo array (número de elementos)
    Array(u16),
    /// Acessa índice de array/tuple
    Index,
    /// Define valor em índice
    SetIndex,
    /// Cria um tuple (número de elementos)
    Tuple(u8),
    /// Acessa elemento de tuple por índice
    TupleAccess(u8),

    // ============================================
    // Manipulação de Pilha
    // ============================================
    /// Remove o topo da pilha
    Pop,
    /// Remove N itens da pilha
    PopN(u8),
    /// Duplica o topo da pilha
    Dup,
    /// Duplica o elemento N posições abaixo do topo
    DupN(u8),
    /// Troca os dois elementos do topo
    Swap,

    // ============================================
    // Operações de I/O e Debug
    // ============================================
    /// Imprime o topo da pilha
    Print,
    /// Imprime com nova linha
    PrintLn,
    /// NOP - não faz nada (útil para debug)
    Nop,
    /// Aborta a execução
    Halt,
}

impl OpCode {
    /// Retorna o nome legível do opcode
    pub fn name(&self) -> &'static str {
        match self {
            OpCode::Constant(_) => "CONSTANT",
            OpCode::ConstantLong(_) => "CONSTANT_LONG",
            OpCode::Nil => "NIL",
            OpCode::True => "TRUE",
            OpCode::False => "FALSE",
            OpCode::Add => "ADD",
            OpCode::Subtract => "SUBTRACT",
            OpCode::Multiply => "MULTIPLY",
            OpCode::Divide => "DIVIDE",
            OpCode::Modulo => "MODULO",
            OpCode::Negate => "NEGATE",
            OpCode::Equal => "EQUAL",
            OpCode::Greater => "GREATER",
            OpCode::Less => "LESS",
            OpCode::GreaterEqual => "GREATER_EQUAL",
            OpCode::LessEqual => "LESS_EQUAL",
            OpCode::Not => "NOT",
            OpCode::And => "AND",
            OpCode::Or => "OR",
            OpCode::BitAnd => "BIT_AND",
            OpCode::BitOr => "BIT_OR",
            OpCode::BitXor => "BIT_XOR",
            OpCode::BitNot => "BIT_NOT",
            OpCode::ShiftLeft => "SHIFT_LEFT",
            OpCode::ShiftRight => "SHIFT_RIGHT",
            OpCode::DefineGlobal(_) => "DEFINE_GLOBAL",
            OpCode::GetGlobal(_) => "GET_GLOBAL",
            OpCode::SetGlobal(_) => "SET_GLOBAL",
            OpCode::GetLocal(_) => "GET_LOCAL",
            OpCode::SetLocal(_) => "SET_LOCAL",
            OpCode::Jump(_) => "JUMP",
            OpCode::JumpIfFalse(_) => "JUMP_IF_FALSE",
            OpCode::JumpIfTrue(_) => "JUMP_IF_TRUE",
            OpCode::Loop(_) => "LOOP",
            OpCode::Break => "BREAK",
            OpCode::Continue => "CONTINUE",
            OpCode::Call(_) => "CALL",
            OpCode::Return => "RETURN",
            OpCode::Closure(_) => "CLOSURE",
            OpCode::GetUpvalue(_) => "GET_UPVALUE",
            OpCode::SetUpvalue(_) => "SET_UPVALUE",
            OpCode::CloseUpvalue => "CLOSE_UPVALUE",
            OpCode::Class(_) => "CLASS",
            OpCode::Method(_) => "METHOD",
            OpCode::Invoke(_) => "INVOKE",
            OpCode::GetProperty(_) => "GET_PROPERTY",
            OpCode::SetProperty(_) => "SET_PROPERTY",
            OpCode::This => "THIS",
            OpCode::Super(_) => "SUPER",
            OpCode::Array(_) => "ARRAY",
            OpCode::Index => "INDEX",
            OpCode::SetIndex => "SET_INDEX",
            OpCode::Tuple(_) => "TUPLE",
            OpCode::TupleAccess(_) => "TUPLE_ACCESS",
            OpCode::Pop => "POP",
            OpCode::PopN(_) => "POP_N",
            OpCode::Dup => "DUP",
            OpCode::DupN(_) => "DUP_N",
            OpCode::Swap => "SWAP",
            OpCode::Print => "PRINT",
            OpCode::PrintLn => "PRINT_LN",
            OpCode::Nop => "NOP",
            OpCode::Halt => "HALT",
            OpCode::TryBegin(_, _) => "TRY_BEGIN",
            OpCode::TryEnd => "TRY_END",
            OpCode::Throw => "THROW",
            OpCode::NewException(_) => "NEW_EXCEPTION",
            OpCode::Catch(_) => "CATCH",
        }
    }

    /// Retorna o tamanho da instrução em bytes (para saltos)
    pub fn size(&self) -> usize {
        // Cada opcode ocupa 1 byte + bytes adicionais para operandos
        match self {
            OpCode::Constant(_) => 2,
            OpCode::ConstantLong(_) => 3,
            OpCode::DefineGlobal(_) => 2,
            OpCode::GetGlobal(_) => 2,
            OpCode::SetGlobal(_) => 2,
            OpCode::GetLocal(_) => 2,
            OpCode::SetLocal(_) => 2,
            OpCode::Jump(_) => 3,
            OpCode::JumpIfFalse(_) => 3,
            OpCode::JumpIfTrue(_) => 3,
            OpCode::Loop(_) => 3,
            OpCode::Call(_) => 2,
            OpCode::Closure(_) => 2,
            OpCode::GetUpvalue(_) => 2,
            OpCode::SetUpvalue(_) => 2,
            OpCode::Class(_) => 2,
            OpCode::Method(_) => 2,
            OpCode::Invoke(_) => 2,
            OpCode::GetProperty(_) => 2,
            OpCode::SetProperty(_) => 2,
            OpCode::Super(_) => 2,
            OpCode::Array(_) => 3,
            OpCode::Tuple(_) => 2,
            OpCode::TupleAccess(_) => 2,
            OpCode::PopN(_) => 2,
            OpCode::DupN(_) => 2,
            OpCode::TryBegin(_, _) => 5,  // 1 + 2 + 2 bytes
            OpCode::NewException(_) => 2,
            OpCode::Catch(_) => 2,
            _ => 1,
        }
    }
}

/// Categorias de opcodes para organização
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCodeCategory {
    Constants,
    Arithmetic,
    Comparison,
    Logical,
    Bitwise,
    Variables,
    ControlFlow,
    Functions,
    Objects,
    Collections,
    Stack,
    Exceptions,
    Misc,
}

impl OpCode {
    /// Retorna a categoria do opcode
    pub fn category(&self) -> OpCodeCategory {
        match self {
            OpCode::Constant(_) | OpCode::ConstantLong(_) | OpCode::Nil | OpCode::True | OpCode::False => {
                OpCodeCategory::Constants
            }
            OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide | OpCode::Modulo | OpCode::Negate => {
                OpCodeCategory::Arithmetic
            }
            OpCode::Equal | OpCode::Greater | OpCode::Less | OpCode::GreaterEqual | OpCode::LessEqual => {
                OpCodeCategory::Comparison
            }
            OpCode::Not | OpCode::And | OpCode::Or => OpCodeCategory::Logical,
            OpCode::BitAnd | OpCode::BitOr | OpCode::BitXor | OpCode::BitNot | OpCode::ShiftLeft | OpCode::ShiftRight => {
                OpCodeCategory::Bitwise
            }
            OpCode::DefineGlobal(_) | OpCode::GetGlobal(_) | OpCode::SetGlobal(_) | OpCode::GetLocal(_) | OpCode::SetLocal(_) => {
                OpCodeCategory::Variables
            }
            OpCode::Jump(_) | OpCode::JumpIfFalse(_) | OpCode::JumpIfTrue(_) | OpCode::Loop(_) | OpCode::Break | OpCode::Continue => {
                OpCodeCategory::ControlFlow
            }
            OpCode::Call(_) | OpCode::Return | OpCode::Closure(_) | OpCode::GetUpvalue(_) | OpCode::SetUpvalue(_) | OpCode::CloseUpvalue => {
                OpCodeCategory::Functions
            }
            OpCode::Class(_) | OpCode::Method(_) | OpCode::Invoke(_) | OpCode::GetProperty(_) | OpCode::SetProperty(_) | OpCode::This | OpCode::Super(_) => {
                OpCodeCategory::Objects
            }
            OpCode::Array(_) | OpCode::Index | OpCode::SetIndex | OpCode::Tuple(_) | OpCode::TupleAccess(_) => {
                OpCodeCategory::Collections
            }
            OpCode::Pop | OpCode::PopN(_) | OpCode::Dup | OpCode::DupN(_) | OpCode::Swap => {
                OpCodeCategory::Stack
            }
            OpCode::TryBegin(_, _) | OpCode::TryEnd | OpCode::Throw | OpCode::NewException(_) | OpCode::Catch(_) => {
                OpCodeCategory::Exceptions
            }
            _ => OpCodeCategory::Misc,
        }
    }
}
