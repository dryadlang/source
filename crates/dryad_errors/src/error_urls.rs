// crates/dryad_errors/src/error_urls.rs

/// Gera URLs de documentação baseadas no código do erro
pub fn get_error_documentation_url(error_code: u16) -> String {
    let base_url = "https://dryadlang.org/errors";
    
    match error_code {
        // ✅ Erros Léxicos Implementados (1000-1999)
        1001 => format!("{}#e1001-unexpected-character", base_url),
        1002 => format!("{}#e1002-unterminated-string-literal", base_url),
        1003 => format!("{}#e1003-unterminated-comment-block", base_url),
        1004 => format!("{}#e1004-invalid-number-format", base_url),
        1005 => format!("{}#e1005-invalid-escape-sequence", base_url),
        1006 => format!("{}#e1006-invalid-native-directive", base_url),
        
        // ✅ Erros de Parser Implementados (2000-2999)  
        2001 => format!("{}#e2001-unexpected-token", base_url),
        2003 => format!("{}#e2003-missing-semicolon", base_url),
        2005 => format!("{}#e2005-missing-closing-parenthesis", base_url),
        2011 => format!("{}#e2011-invalid-variable-declaration", base_url),
        
        // 🟡 Erros de Parser Esperados (2000-2999)
        2017 => format!("{}#e2017-missing-function-parameters", base_url),
        2018 => format!("{}#e2018-missing-while-condition", base_url),
        2019 => format!("{}#e2019-missing-for-components", base_url),
        
        // ✅ Erros de Runtime Implementados (3000-3999)
        3001 => format!("{}#e3001-undefined-variable", base_url),
        3005 => format!("{}#e3005-invalid-arithmetic-operation", base_url),
        3006 => format!("{}#e3006-invalid-multiplication", base_url),
        3007 => format!("{}#e3007-division-by-zero", base_url),
        3009 => format!("{}#e3009-invalid-comparison", base_url),
        3010 => format!("{}#e3010-break-outside-loop", base_url),
        3011 => format!("{}#e3011-continue-outside-loop", base_url),
        3020 => format!("{}#e3020-exception-thrown", base_url),
        3021 => format!("{}#e3021-function-return", base_url),
        3022 => format!("{}#e3022-invalid-this-context", base_url),
        3023 => format!("{}#e3023-super-not-implemented", base_url),
        3034 => format!("{}#e3034-invalid-property-assignment", base_url),
        3040 => format!("{}#e3040-stack-overflow", base_url),
        
        // 🟡 Erros de Tipo Planejados (4000-4999)
        4001 => format!("{}#e4001-incompatible-types", base_url),
        4002 => format!("{}#e4002-invalid-conversion", base_url),
        
        // 🟡 Erros de I/O Planejados (5000-5999)
        5001 => format!("{}#e5001-file-not-found", base_url),
        5002 => format!("{}#e5002-permission-denied", base_url),
        
        // ✅ Erros de Módulo Implementados (6000-6999)
        6001 => format!("{}#e6001-unknown-native-module", base_url),
        6002 => format!("{}#e6002-import-circular", base_url),
        
        // 🟡 Erros de Sintaxe Planejados (7000-7999)
        7001 => format!("{}#e7001-invalid-syntax-declaration", base_url),
        
        // 🟡 Warnings Planejados (8000-8999)
        8001 => format!("{}#w8001-unused-variable", base_url),
        8002 => format!("{}#w8002-deprecated-function", base_url),
        8003 => format!("{}#w8003-potential-memory-leak", base_url),
        
        // 🟡 Erros de Sistema Planejados (9000-9999)
        9001 => format!("{}#e9001-insufficient-memory", base_url),
        9002 => format!("{}#e9002-stack-overflow", base_url),
        
        // URL genérica para códigos não mapeados
        _ => {
            let category = error_code / 1000;
            match category {
                1 => format!("{}#lexer-errors-1000-1999", base_url),
                2 => format!("{}#parser-errors-2000-2999", base_url),
                3 => format!("{}#runtime-errors-3000-3999", base_url),
                4 => format!("{}#type-errors-4000-4999", base_url),
                5 => format!("{}#io-errors-5000-5999", base_url),
                6 => format!("{}#module-errors-6000-6999", base_url),
                7 => format!("{}#syntax-errors-7000-7999", base_url),
                8 => format!("{}#warnings-8000-8999", base_url),
                9 => format!("{}#system-errors-9000-9999", base_url),
                _ => base_url.to_string(),
            }
        }
    }
}

/// Gera sugestões contextuais baseadas no código do erro
pub fn get_error_suggestions(error_code: u16) -> Vec<String> {
    match error_code {
        // ✅ Erros Léxicos Implementados
        1001 => vec![
            "Use apenas letras, números, underscore e operadores válidos".to_string(),
            "Remova caracteres especiais não suportados (@, $, etc.)".to_string(),
            "Verifique se não há caracteres de controle invisíveis".to_string(),
        ],
        1002 => vec![
            "Adicione \" para fechar a string".to_string(),
            "Verifique se não há quebras de linha não intencionais".to_string(),
            "Use \\\" para incluir aspas dentro de strings".to_string(),
        ],
        1003 => vec![
            "Adicione */ para fechar o comentário de bloco".to_string(),
            "Considere usar // para comentários de linha única".to_string(),
            "Verifique se há comentários aninhados incorretamente".to_string(),
        ],
        1004 => vec![
            "Para números decimais use apenas um ponto: 3.14".to_string(),
            "Para binário use apenas 0 e 1: 0b1010".to_string(),
            "Para octal use apenas 0-7: 0o755".to_string(),
            "Para hexadecimal use apenas 0-9, A-F: 0xFF".to_string(),
        ],
        1005 => vec![
            "Use sequências válidas: \\n, \\t, \\r, \\\\, \\\", \\'".to_string(),
            "Para Unicode use \\u{XXXX} com 4 dígitos hex".to_string(),
            "Escape caracteres especiais em strings".to_string(),
        ],
        1006 => vec![
            "Use formato correto: #<module_name>".to_string(),
            "Módulos válidos: console_io, file_io, http, crypto, etc.".to_string(),
            "Use apenas caracteres alfanuméricos e underscore".to_string(),
        ],
        
        // ✅ Erros de Parser Implementados  
        2001 => vec![
            "Verifique se parênteses e chaves estão balanceados".to_string(),
            "Adicione tokens em falta (vírgulas, operadores, etc.)".to_string(),
            "Remova tokens desnecessários ou duplicados".to_string(),
        ],
        2003 => vec![
            "Adicione ; no final da declaração".to_string(),
            "Separe múltiplas declarações com ;".to_string(),
            "Verifique se a sintaxe da declaração está correta".to_string(),
        ],
        2005 => vec![
            "Adicione ) para fechar a expressão".to_string(),
            "Verifique se todos os parênteses estão balanceados".to_string(),
            "Use parênteses apenas onde necessário".to_string(),
        ],
        2011 => vec![
            "Use: let nome_variavel = valor;".to_string(),
            "Nome da variável deve começar com letra ou _".to_string(),
            "Não use números no início do nome da variável".to_string(),
        ],
        
        // 🟡 Erros de Parser Esperados
        2017 => vec![
            "Adicione () após o nome da função".to_string(),
            "Exemplo: function teste() { ... }".to_string(),
        ],
        2018 => vec![
            "Adicione condição entre parênteses".to_string(),
            "Exemplo: while (condicao) { ... }".to_string(),
        ],
        2019 => vec![
            "Use: for (init; condicao; incremento) { ... }".to_string(),
            "Todos os componentes são opcionais mas ; são obrigatórios".to_string(),
        ],
        
        // ✅ Erros de Runtime Implementados
        3001 => vec![
            "Declare a variável: let nome_variavel = valor;".to_string(),
            "Verifique a grafia do nome da variável".to_string(),
            "Verifique se a variável está no escopo correto".to_string(),
        ],
        3005 => vec![
            "Use operações compatíveis: números com números".to_string(),
            "Para strings use apenas + (concatenação)".to_string(),
            "Converta tipos quando necessário".to_string(),
        ],
        3006 => vec![
            "Use * apenas entre números".to_string(),
            "Para strings use repetição: String(valor) * numero".to_string(),
        ],
        3007 => vec![
            "Verifique se divisor != 0 antes da operação".to_string(),
            "Use: if (divisor != 0) { resultado = a / divisor; }".to_string(),
            "Implemente tratamento de erro para divisão por zero".to_string(),
        ],
        3009 => vec![
            "Compare tipos compatíveis: números com números".to_string(),
            "Strings são comparadas lexicograficamente".to_string(),
            "Use === para comparação estrita".to_string(),
        ],
        3010 => vec![
            "Use break apenas dentro de while, for ou do-while".to_string(),
            "Para sair de funções use return".to_string(),
        ],
        3011 => vec![
            "Use continue apenas dentro de while, for ou do-while".to_string(),
            "Continue pula para próxima iteração do loop".to_string(),
        ],
        3020 => vec![
            "Use try/catch para capturar exceções".to_string(),
            "Exemplo: try { ... } catch (e) { ... }".to_string(),
        ],
        3022 => vec![
            "Use this apenas em métodos de classe".to_string(),
            "this refere-se à instância atual da classe".to_string(),
        ],
        3023 => vec![
            "super será implementado em versões futuras".to_string(),
            "Use chamadas diretas por enquanto".to_string(),
        ],
        3034 => vec![
            "Use propriedades apenas em instâncias de classe".to_string(),
            "Exemplo: instancia.propriedade = valor;".to_string(),
        ],
        3040 => vec![
            "Verifique se há recursão infinita no seu código".to_string(),
            "Aumente o limite de recursão se necessário (configuração do runtime)".to_string(),
            "Tente converter recursão para iteração (loops)".to_string(),
        ],
        
        // ✅ Erros de Módulo Implementados
        6001 => vec![
            "Verifique se o módulo existe".to_string(),
            "Módulos disponíveis: console_io, file_io, http, crypto, etc.".to_string(),
            "Use #<nome_modulo> no início do arquivo".to_string(),
        ],
        
        // 🟡 Warnings Planejados
        8001 => vec![
            "Remova a variável se não for necessária".to_string(),
            "Use a variável no código".to_string(),
            "Prefixe com _ se for intencional: let _variavel = valor;".to_string(),
        ],
        
        // Sugestões genéricas
        _ => vec![
            "Consulte o guia de erros para mais informações".to_string(),
            "Verifique o contexto e stack trace do erro".to_string(),
            "Revise a sintaxe oficial no arquivo SYNTAX.md".to_string(),
        ]
    }
}