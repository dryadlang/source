#!/usr/bin/env python3
"""Migrate all DryadError::new() calls in parser.rs to use error catalog."""

import re

with open('crates/dryad_parser/src/parser.rs', 'r') as f:
    content = f.read()

# Track replacements made
count = 0

def make_catalog_call(code, msg, has_format=False):
    """Generate the appropriate from_catalog or from_catalog_fmt call."""
    catalog_fn = f"error_catalog::e{code}()"
    if has_format:
        return f"DryadError::from_catalog_fmt({catalog_fn}, {msg}, SourceLocation::unknown())"
    else:
        return f"DryadError::from_catalog({catalog_fn}, SourceLocation::unknown())"

# Define all replacements as (old_pattern, new_pattern) tuples
# We'll do them as exact string replacements

replacements = []

# === POSTFIX: Line ~578 - 2071 ===
replacements.append((
    'DryadError::new(2071, "Esperado \']\' após índice do array")',
    'DryadError::from_catalog(error_catalog::e2071(), SourceLocation::unknown())',
))

# === POSTFIX: Line ~603 - 2073 method args ===
replacements.append((
    'DryadError::new(2073, "Esperado \',\' ou \')\' na lista de argumentos do método")',
    'DryadError::from_catalog(error_catalog::e2073(), SourceLocation::unknown())',
))

# === POSTFIX: Line ~610-614 - 2074 method args close ===
replacements.append((
    'DryadError::new(\n                                         2074,\n                                         "Esperado \')\' após argumentos do método",\n                                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2082(), "Expected \')\' after method arguments", SourceLocation::unknown())',
))

# === POSTFIX: Line ~623-627 - 2072 property access ===
replacements.append((
    'DryadError::new(\n                                 2072,\n                                 "Esperado número ou identificador após \'.\' para acesso",\n                             )',
    'DryadError::from_catalog_fmt(error_catalog::e2033(), "Expected number or identifier after \'.\'", SourceLocation::unknown())',
))

# === POSTFIX: Line ~649-653 - 2075 call args ===
replacements.append((
    'DryadError::new(\n                                         2075,\n                                         "Esperado \',\' ou \')\' na lista de argumentos da chamada",\n                                     )',
    'DryadError::from_catalog(error_catalog::e2075(), SourceLocation::unknown())',
))

# === POSTFIX: Line ~660-664 - 2076 call close ===
replacements.append((
    'DryadError::new(\n                             2076,\n                             "Esperado \')\' após argumentos da chamada",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2082(), "Expected \')\' after call arguments", SourceLocation::unknown())',
))

# === POSTFIX: Line ~680-684 - 2083 namespace ===
replacements.append((
    'DryadError::new(\n                                     2083,\n                                     "Esperado identificador após \'::\' para acesso a namespace",\n                                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2083(), "Expected identifier after \'::\' for namespace access", SourceLocation::unknown())',
))

# === PRIMARY super: Line ~740-744 - 2080 super dot ===
replacements.append((
    'DryadError::new(\n                                 2080,\n                                 "Esperado identificador após \'.\' em \'super\'",\n                             )',
    'DryadError::from_catalog_fmt(error_catalog::e2080(), "Expected identifier after \'.\' in \'super\'", SourceLocation::unknown())',
))

# === PRIMARY super: Line ~765-769 - 2081 super args ===
replacements.append((
    'DryadError::new(\n                                             2081,\n                                             "Esperado \',\' ou \')\' após argumentos",\n                                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2081(), "Expected \',\' or \')\' after arguments", SourceLocation::unknown())',
))

# === PRIMARY super: Line ~775 - 2082 super args close ===
replacements.append((
    'DryadError::new(2082, "Esperado \')\' após argumentos")',
    'DryadError::from_catalog(error_catalog::e2082(), SourceLocation::unknown())',
))

# === PRIMARY mutex: Line ~801 - 2029 ===
replacements.append((
    'DryadError::new(2029, "Esperado \')\' após \'mutex(\'")',
    'DryadError::from_catalog(error_catalog::e2029(), SourceLocation::unknown())',
))

# === PRIMARY mutex: Line ~804 - 2030 ===
replacements.append((
    'DryadError::new(2030, "Esperado \'(\' após \'mutex\'")',
    'DryadError::from_catalog(error_catalog::e2030(), SourceLocation::unknown())',
))

# === PRIMARY new: Line ~819 - 2090 (KEEP original - class name after new) ===
replacements.append((
    'DryadError::new(2090, "Esperado nome da classe após \'new\'")',
    'DryadError::from_catalog(error_catalog::e2090(), SourceLocation::unknown())',
))

# === PRIMARY new: Line ~841-845 - 2091 constructor args (no dedup needed for constructor context) ===
replacements.append((
    'DryadError::new(\n                                         2091,\n                                         "Esperado \',\' ou \')\' na lista de argumentos do construtor",\n                                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2082(), "Expected \',\' or \')\' in constructor argument list", SourceLocation::unknown())',
))

# === PRIMARY new: Line ~851-855 - 2092 constructor close ===
replacements.append((
    'DryadError::new(\n                             2092,\n                             "Esperado \')\' após argumentos do construtor",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2092(), "Expected \')\' after constructor arguments", SourceLocation::unknown())',
))

# === PRIMARY thread: Line ~886-890 - 2031 thread close ===
replacements.append((
    'DryadError::new(\n                             2031,\n                             "Esperado \')\' após argumentos do thread",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2032(), "Expected \')\' after thread arguments", SourceLocation::unknown())',
))

# === PRIMARY thread: Line ~892 - 2032 ===
replacements.append((
    'DryadError::new(2032, "Esperado \'(\' após \'thread\'")',
    'DryadError::from_catalog(error_catalog::e2032(), SourceLocation::unknown())',
))

# === PRIMARY identifier call: Line ~931-935 - 2017 (KEEP original - ',' or ')' in args) ===
replacements.append((
    'DryadError::new(\n                                         2017,\n                                         "Esperado \',\' ou \')\' na lista de argumentos",\n                                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2017(), "Expected \',\' or \')\' in argument list", SourceLocation::unknown())',
))

# === PRIMARY identifier call: Line ~942 - 2018 (KEEP original - ')' after arguments) ===
replacements.append((
    'DryadError::new(2018, "Esperado \')\' após argumentos")',
    'DryadError::from_catalog(error_catalog::e2018(), SourceLocation::unknown())',
))

# === PRIMARY lambda: Line ~1050-1054 - 2019 lambda param ===
replacements.append((
    'DryadError::new(\n                                         2019,\n                                         "Esperado identificador de parâmetro",\n                                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2014(), "Expected parameter identifier", SourceLocation::unknown())',
))

# === PRIMARY lambda: Line ~1064-1068 - 2020 lambda close ===
replacements.append((
    'DryadError::new(\n                                     2020,\n                                     "Esperado \')\' após parâmetros da lambda",\n                                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2016(), "Expected \')\' after lambda parameters", SourceLocation::unknown())',
))

# === PRIMARY lambda arrow: Line ~1088-1092 - 2021 lambda arrow ===
replacements.append((
    'DryadError::new(\n                             2021,\n                             "Esperado \'=>\' após parâmetros da lambda",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2035(), "Expected \'=>\' after lambda parameters", SourceLocation::unknown())',
))

# === PRIMARY tuple: Line ~1125 - 2005 tuple ===
replacements.append((
    'DryadError::new(2005, "Esperado \')\' após tupla")',
    'DryadError::from_catalog_fmt(error_catalog::e2005(), "Expected \')\' after tuple", SourceLocation::unknown())',
))

# === PRIMARY expression: Line ~1132 - 2005 expr ===
replacements.append((
    'DryadError::new(2005, "Esperado \')\' após expressão")',
    'DryadError::from_catalog(error_catalog::e2005(), SourceLocation::unknown())',
))

# === PRIMARY unexpected token: Line ~1139-1142 - 2001 ===
replacements.append((
    'DryadError::new(\n                 2001,\n                 &format!("Token inesperado: {:?}", self.peek()),\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2033(), &format!("Unexpected token: {:?}", self.peek()), SourceLocation::unknown())',
))

# === ASSIGNMENT_STATEMENT: Line ~1151-1155 - 2012 assignment identifier ===
replacements.append((
    'DryadError::new(\n                     2012,\n                     "Esperado identificador para assignment",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2012(), "Expected identifier for assignment", SourceLocation::unknown())',
))

# === ASSIGNMENT_STATEMENT: Line ~1224 - 2013 (DEDUP → 2042) ===
replacements.append((
    'DryadError::new(2013, "Operador de assignment inválido")',
    'DryadError::from_catalog(error_catalog::e2042(), SourceLocation::unknown())',
))

# === EXPORT: Line ~1247-1249 - 4001 ===
replacements.append((
    'DryadError::new(\n                 4001,\n                 "Export deve ser seguido por \'function\', \'class\' ou \'let\'",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2033(), "Export must be followed by \'function\', \'class\', or \'let\'", SourceLocation::unknown())',
))

# === WHILE: Line ~1291 - 2051 (KEEP original - while condition) ===
replacements.append((
    'DryadError::new(2051, "Esperado \')\' após condição do while")',
    'DryadError::from_catalog(error_catalog::e2051(), SourceLocation::unknown())',
))

# === WHILE: Line ~1302-1305 - 2052 while brace ===
replacements.append((
    'DryadError::new(\n                 2052,\n                 "Esperado \'{\' após parênteses do while",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2053(), "Expected \'{\' after while parentheses", SourceLocation::unknown())',
))

# === DO-WHILE: Line ~1320 - 2053 ===
replacements.append((
    'DryadError::new(2053, "Esperado \'{\' após \'do\'")',
    'DryadError::from_catalog(error_catalog::e2053(), SourceLocation::unknown())',
))

# === DO-WHILE: Line ~1328-1331 - 2054 ===
replacements.append((
    'DryadError::new(\n                 2054,\n                 "Esperado \'while\' após corpo do do-while",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2051(), "Expected \'while\' after do-while body", SourceLocation::unknown())',
))

# === DO-WHILE: Line ~1337-1341 - 2065 ===
replacements.append((
    'DryadError::new(\n                 2065,\n                 "Esperado \'(\' após \'while\' no do-while - sintaxe: do { ... } while (condição);",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2051(), "Expected \'(\' after \'while\' in do-while - syntax: do { ... } while (condition);", SourceLocation::unknown())',
))

# === DO-WHILE: Line ~1349-1352 - 2066 ===
replacements.append((
    'DryadError::new(\n                 2066,\n                 "Esperado \')\' após condição do do-while",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2051(), "Expected \')\' after do-while condition", SourceLocation::unknown())',
))

# === DO-WHILE: Line ~1358-1361 - 2067 ===
replacements.append((
    'DryadError::new(\n                 2067,\n                 "Esperado \';\' após parênteses do do-while",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2033(), "Expected \';\' after do-while parentheses", SourceLocation::unknown())',
))

# === FOR: Line ~1398-1401 - 2055 ===
replacements.append((
    'DryadError::new(\n                 2055,\n                 "Esperado \'(\' após \'for\' - sintaxe: for (init; condition; update)",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \'(\' after \'for\' - syntax: for (init; condition; update)", SourceLocation::unknown())',
))

# === FOR: Line ~1432-1435 - 2056 ===
replacements.append((
    'DryadError::new(\n                         2056,\n                         "Esperado \'=\' na inicialização do for - sintaxe: for (i = 0; ...)",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \'=\' in for initialization - syntax: for (i = 0; ...)", SourceLocation::unknown())',
))

# === FOR: Line ~1446-1449 - 2057 ===
replacements.append((
    'DryadError::new(\n                     2057,\n                     "Esperado identificador na inicialização do for - sintaxe: for (i = 0; ...)",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected identifier in for initialization - syntax: for (i = 0; ...)", SourceLocation::unknown())',
))

# === FOR: Line ~1455-1458 - 2058 ===
replacements.append((
    'DryadError::new(\n                 2058,\n                 "Esperado \';\' após inicialização do for - sintaxe: for (i = 0; condition; ...)",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \';\' after for initialization - syntax: for (i = 0; condition; ...)", SourceLocation::unknown())',
))

# === FOR: Line ~1471-1474 - 2059 ===
replacements.append((
    'DryadError::new(\n                 2059,\n                 "Esperado \';\' após condição do for - sintaxe: for (...; i < 10; update)",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \';\' after for condition - syntax: for (...; i < 10; update)", SourceLocation::unknown())',
))

# === FOR: Line ~1525-1527 - 2060 ===
replacements.append((
    'DryadError::new(\n                         2060, "Esperado \'=\', \'++\' ou \'--\' no update do for - sintaxe: for (...; ...; i++)"',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \'=\', \'+\' or \'--\' in for update - syntax: for (...; ...; i++)", SourceLocation::unknown()',
))

# === FOR: Line ~1530-1533 - 2061 ===
replacements.append((
    'DryadError::new(\n                     2061,\n                     "Esperado identificador no update do for - sintaxe: for (...; ...; i++)",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected identifier in for update - syntax: for (...; ...; i++)", SourceLocation::unknown())',
))

# === FOR: Line ~1539-1542 - 2062 ===
replacements.append((
    'DryadError::new(\n                 2062,\n                 "Esperado \')\' após declaração do for - sintaxe: for (init; condition; update)",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \')\' after for declaration - syntax: for (init; condition; update)", SourceLocation::unknown())',
))

# === FOR: Line ~1548 - 2063 ===
replacements.append((
    'DryadError::new(2063, "Esperado \'{\' após parênteses do for")',
    'DryadError::from_catalog(error_catalog::e2063(), SourceLocation::unknown())',
))

# === FOREACH: Line ~1561-1564 - 2068 ===
replacements.append((
    'DryadError::new(\n                 2068,\n                 "Esperado \'in\' em foreach loop - sintaxe: for (item in array)",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \'in\' in foreach loop - syntax: for (item in array)", SourceLocation::unknown())',
))

# === FOREACH: Line ~1573-1576 - 2069 ===
replacements.append((
    'DryadError::new(\n                 2069,\n                 "Esperado \')\' após expressão do foreach - sintaxe: for (item in array)",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \')\' after foreach expression - syntax: for (item in array)", SourceLocation::unknown())',
))

# === FOREACH: Line ~1582-1585 - 2070 ===
replacements.append((
    'DryadError::new(\n                 2070,\n                 "Esperado \'{\' após parênteses do foreach",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2063(), "Expected \'{\' after foreach parentheses", SourceLocation::unknown())',
))

# === FUNCTION_DECL: Line ~1606 - 2012 ===
replacements.append((
    '_ => return Err(DryadError::new(2012, "Esperado nome da função")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2012(), SourceLocation::unknown())),',
))

# === FUNCTION_DECL: Line ~1611 - 2013 (DEDUP → 2043) ===
replacements.append((
    'DryadError::new(2013, "Esperado \'(\' após nome da função")',
    'DryadError::from_catalog(error_catalog::e2043(), SourceLocation::unknown())',
))

# === FUNCTION_DECL: Line ~1637 - 2014 ===
replacements.append((
    '_ => return Err(DryadError::new(2014, "Esperado nome do parâmetro")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2014(), SourceLocation::unknown())),',
))

# === FUNCTION_DECL: Line ~1647-1650 - 2015 ===
replacements.append((
    'DryadError::new(\n                             2015,\n                             "Esperado \',\' ou \')\' na lista de parâmetros",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2016(), "Expected \',\' or \')\' in parameter list", SourceLocation::unknown())',
))

# === FUNCTION_DECL: Line ~1658 - 2016 ===
replacements.append((
    'DryadError::new(2016, "Esperado \')\' após parâmetros")',
    'DryadError::from_catalog(error_catalog::e2016(), SourceLocation::unknown())',
))

# === ASYNC_FUNC: Line ~1690 - 2017 ===
replacements.append((
    '_ => return Err(DryadError::new(2017, "Esperado \'function\' após \'async\'")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2017(), SourceLocation::unknown())),',
))

# === ASYNC_FUNC: Line ~1696 - 2018 (DEDUP → 2044) ===
replacements.append((
    '_ => return Err(DryadError::new(2018, "Esperado nome da função async")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2044(), SourceLocation::unknown())),',
))

# === ASYNC_FUNC: Line ~1701-1704 - 2019 async open paren ===
replacements.append((
    'DryadError::new(\n                 2019,\n                 "Esperado \'(\' após nome da função async",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2043(), "Expected \'(\' after async function name", SourceLocation::unknown())',
))

# === ASYNC_FUNC: Line ~1731-1734 - 2020 async param ===
replacements.append((
    'DryadError::new(\n                             2020,\n                             "Esperado nome do parâmetro na função async",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2014(), "Expected parameter name in async function", SourceLocation::unknown())',
))

# === ASYNC_FUNC: Line ~1745-1748 - 2021 async param list ===
replacements.append((
    'DryadError::new(\n                             2021,\n                             "Esperado \',\' ou \')\' na lista de parâmetros da função async",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2016(), "Expected \',\' or \')\' in async function parameter list", SourceLocation::unknown())',
))

# === ASYNC_FUNC: Line ~1756-1759 - 2022 async close paren ===
replacements.append((
    'DryadError::new(\n                 2022,\n                 "Esperado \')\' após parâmetros da função async",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2016(), "Expected \')\' after async function parameters", SourceLocation::unknown())',
))

# === THREAD_FUNC: Line ~1791 - 2023 ===
replacements.append((
    '_ => return Err(DryadError::new(2023, "Esperado \'function\' após \'thread\'")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2023(), SourceLocation::unknown())),',
))

# === THREAD_FUNC: Line ~1797 - 2024 ===
replacements.append((
    '_ => return Err(DryadError::new(2024, "Esperado nome da função thread")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2024(), SourceLocation::unknown())),',
))

# === THREAD_FUNC: Line ~1802-1805 - 2025 thread open paren ===
replacements.append((
    'DryadError::new(\n                 2025,\n                 "Esperado \'(\' após nome da função thread",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2043(), "Expected \'(\' after thread function name", SourceLocation::unknown())',
))

# === THREAD_FUNC: Line ~1824-1827 - 2026 thread param ===
replacements.append((
    'DryadError::new(\n                             2026,\n                             "Esperado nome do parâmetro na função thread",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2014(), "Expected parameter name in thread function", SourceLocation::unknown())',
))

# === THREAD_FUNC: Line ~1838-1841 - 2027 thread param list ===
replacements.append((
    'DryadError::new(\n                             2027,\n                             "Esperado \',\' ou \')\' na lista de parâmetros da função thread",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2016(), "Expected \',\' or \')\' in thread function parameter list", SourceLocation::unknown())',
))

# === THREAD_FUNC: Line ~1849-1852 - 2028 thread close paren ===
replacements.append((
    'DryadError::new(\n                 2028,\n                 "Esperado \')\' após parâmetros da função thread",\n             )',
    'DryadError::from_catalog(error_catalog::e2028(), SourceLocation::unknown())',
))

# === CLASS_DECL: Line ~1893-1896 - 2087 ===
replacements.append((
    'DryadError::new(\n                     2087,\n                     "Esperado nome da classe após \'class\'",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2012(), "Expected class name after \'class\'", SourceLocation::unknown())',
))

# === CLASS_DECL: Line ~1913-1916 - 2088 extends ===
replacements.append((
    'DryadError::new(\n                         2088,\n                         "Esperado nome da classe pai após \'extends\'",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2012(), "Expected parent class name after \'extends\'", SourceLocation::unknown())',
))

# === CLASS_DECL: Line ~1929-1932 - 2089 class brace ===
replacements.append((
    'DryadError::new(\n                 2089,\n                 "Esperado \'{\' após declaração da classe",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2046(), "Expected \'{\' after class declaration", SourceLocation::unknown())',
))

# === CLASS_DECL: Line ~1944 - 2090 (DEDUP → 2046) ===
replacements.append((
    'DryadError::new(2090, "Esperado \'}\' para fechar classe")',
    'DryadError::from_catalog(error_catalog::e2046(), SourceLocation::unknown())',
))

# === NAMESPACE: Line ~1992-1995 - 2113 ===
replacements.append((
    'DryadError::new(\n                     2113,\n                     "Esperado nome do namespace após \'namespace\'",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2115(), "Expected namespace name after \'namespace\'", SourceLocation::unknown())',
))

# === NAMESPACE: Line ~2001-2004 - 2114 ===
replacements.append((
    'DryadError::new(\n                 2114,\n                 "Esperado \'{\' após declaração do namespace",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2115(), "Expected \'{\' after namespace declaration", SourceLocation::unknown())',
))

# === NAMESPACE: Line ~2018 - 2115 ===
replacements.append((
    'DryadError::new(2115, "Esperado \'}\' para fechar namespace")',
    'DryadError::from_catalog(error_catalog::e2115(), SourceLocation::unknown())',
))

# === INTERFACE_DECL: Line ~2038-2041 - 2105 ===
replacements.append((
    'DryadError::new(\n                     2105,\n                     "Esperado nome da interface após \'interface\'",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2107(), "Expected interface name after \'interface\'", SourceLocation::unknown())',
))

# === INTERFACE_DECL: Line ~2047-2050 - 2106 ===
replacements.append((
    'DryadError::new(\n                 2106,\n                 "Esperado \'{\' após declaração da interface",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2107(), "Expected \'{\' after interface declaration", SourceLocation::unknown())',
))

# === INTERFACE_DECL: Line ~2062 - 2107 ===
replacements.append((
    'DryadError::new(2107, "Esperado \'}\' para fechar interface")',
    'DryadError::from_catalog(error_catalog::e2107(), SourceLocation::unknown())',
))

# === INTERFACE_MEMBER: Line ~2075 - 2108 ===
replacements.append((
    'DryadError::new(2108, "Esperado \'function\' em interface")',
    'DryadError::from_catalog(error_catalog::e2108(), SourceLocation::unknown())',
))

# === INTERFACE_MEMBER: Line ~2086 - 2109 ===
replacements.append((
    '_ => return Err(DryadError::new(2109, "Esperado nome do método")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2109(), SourceLocation::unknown())),',
))

# === INTERFACE_MEMBER: Line ~2091 - 2110 ===
replacements.append((
    'DryadError::new(2110, "Esperado \'(\' após nome do método")',
    'DryadError::from_catalog(error_catalog::e2110(), SourceLocation::unknown())',
))

# === INTERFACE_MEMBER: Line ~2115 - 2111 ===
replacements.append((
    'DryadError::new(2111, "Esperado nome do parâmetro")',
    'DryadError::from_catalog(error_catalog::e2111(), SourceLocation::unknown())',
))

# === INTERFACE_MEMBER: Line ~2121 - 2112 ===
replacements.append((
    'DryadError::new(2112, "Esperado \')\' após parâmetros")',
    'DryadError::from_catalog(error_catalog::e2112(), SourceLocation::unknown())',
))

# === CLASS_MEMBER async: Line ~2173 - 2091 (KEEP original - async method name) ===
replacements.append((
    '_ => return Err(DryadError::new(2091, "Esperado nome do método async")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2091(), SourceLocation::unknown())),',
))

# === CLASS_MEMBER async: Line ~2178-2181 - 2092 async method open paren ===
replacements.append((
    'DryadError::new(\n                             2092,\n                             "Esperado \'(\' após nome do método async",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2092(), "Expected \'(\' after async method name", SourceLocation::unknown())',
))

# === CLASS_MEMBER async: Line ~2206 - 2094 param ===
# This one has the exact same text as line ~2294; handle uniquely by context
# Line 2206 is inside async method block
replacements.append((
    'return Err(DryadError::new(2094, "Esperado nome do parâmetro"));',
    'return Err(DryadError::from_catalog(error_catalog::e2094(), SourceLocation::unknown()));',
))

# === CLASS_MEMBER async: Line ~2239-2242 - 2096 async in class ===
replacements.append((
    'DryadError::new(\n                         2096,\n                         "Esperado \'function\' após \'async\' em classe",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2017(), "Expected \'function\' after \'async\' in class", SourceLocation::unknown())',
))

# === CLASS_MEMBER regular: Line ~2255 - 2091 (DEDUP → 2047) ===
replacements.append((
    '_ => return Err(DryadError::new(2091, "Esperado nome do método")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2047(), SourceLocation::unknown())),',
))

# === CLASS_MEMBER regular: Line ~2260 - 2092 regular method paren ===
replacements.append((
    'DryadError::new(2092, "Esperado \'(\' após nome do método")',
    'DryadError::from_catalog(error_catalog::e2092(), SourceLocation::unknown())',
))

# === CLASS_MEMBER regular: Line ~2288-2291 - 2093 ===
replacements.append((
    'DryadError::new(\n                                     2093,\n                                     "Esperado \',\' ou \')\' na lista de parâmetros",\n                                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2016(), "Expected \',\' or \')\' in parameter list", SourceLocation::unknown())',
))

# === CLASS_MEMBER regular: Line ~2294 - 2094 ===
replacements.append((
    '_ => return Err(DryadError::new(2094, "Esperado nome do parâmetro")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2094(), SourceLocation::unknown())),',
))

# === CLASS_MEMBER getter: Line ~2331-2334 - 2097 getter name ===
replacements.append((
    'DryadError::new(\n                             2097,\n                             "Esperado nome da propriedade após \'get\'",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2098(), "Expected property name after \'get\'", SourceLocation::unknown())',
))

# === CLASS_MEMBER getter: Line ~2340 - 2098 getter paren ===
replacements.append((
    'DryadError::new(2098, "Esperado \'(\' após nome do getter")',
    'DryadError::from_catalog(error_catalog::e2098(), SourceLocation::unknown())',
))

# === CLASS_MEMBER getter: Line ~2344 - 2099 getter close ===
replacements.append((
    'DryadError::new(2099, "Esperado \')\' no getter")',
    'DryadError::from_catalog(error_catalog::e2099(), SourceLocation::unknown())',
))

# === CLASS_MEMBER setter: Line ~2369-2372 - 2100 setter name ===
replacements.append((
    'DryadError::new(\n                             2100,\n                             "Esperado nome da propriedade após \'set\'",\n                         )',
    'DryadError::from_catalog(error_catalog::e2100(), SourceLocation::unknown())',
))

# === CLASS_MEMBER setter: Line ~2378 - 2101 setter paren ===
replacements.append((
    'DryadError::new(2101, "Esperado \'(\' após nome do setter")',
    'DryadError::from_catalog(error_catalog::e2101(), SourceLocation::unknown())',
))

# === CLASS_MEMBER setter: Line ~2389 - 2102 (KEEP original - setter parameter) ===
replacements.append((
    '_ => return Err(DryadError::new(2102, "Esperado parâmetro no setter")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2102(), SourceLocation::unknown())),',
))

# === CLASS_MEMBER setter: Line ~2393 - 2103 (KEEP original - setter close) ===
replacements.append((
    'DryadError::new(2103, "Esperado \')\' no setter")',
    'DryadError::from_catalog(error_catalog::e2103(), SourceLocation::unknown())',
))

# === CLASS_MEMBER property let: Line ~2418 - 2095 ===
# Both property ones have the same string so use replaceAll behavior
replacements.append((
    '_ => return Err(DryadError::new(2095, "Esperado nome da propriedade")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2095(), SourceLocation::unknown())),',
))

# === CLASS_MEMBER default: Line ~2490-2493 - 2096 ===
replacements.append((
    'DryadError::new(\n                 2096,\n                 "Esperado declaração de método ou propriedade",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2033(), "Expected method or property declaration", SourceLocation::unknown())',
))

# === PARSE_TYPE: Line ~2556-2559 - 2100 tuple type ===
replacements.append((
    'DryadError::new(\n                         2100,\n                         "Esperado \')\' após lista de tipos de tupla",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2005(), "Expected \')\' after tuple type list", SourceLocation::unknown())',
))

# === PARSE_TYPE: Line ~2563-2565 - 2101 invalid type ===
replacements.append((
    'DryadError::new(\n                 2101,\n                 &format!("Tipo inválido: {:?}", token),\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2033(), &format!("Invalid type: {:?}", token), SourceLocation::unknown())',
))

# === IF_STATEMENT else: Line ~2658 - 2051 (DEDUP → 2045) ===
replacements.append((
    'DryadError::new(2051, "Esperado \'{\' após \'else\'")',
    'DryadError::from_catalog(error_catalog::e2045(), SourceLocation::unknown())',
))

# === PARSE_ARRAY: Line ~2709-2712 - 2070 array close ===
replacements.append((
    'DryadError::new(\n                 2070,\n                 "Esperado \']\' após elementos do array",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2071(), "Expected \']\' after array elements", SourceLocation::unknown())',
))

# === PARSE_OBJECT: Line ~2747-2750 - 2071 object close ===
replacements.append((
    'DryadError::new(\n                 2071,\n                 "Esperado \'}\' após propriedades do objeto",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2041(), "Expected \'}\' after object properties", SourceLocation::unknown())',
))

# === PARSE_OBJECT_PROPERTY: Line ~2763-2766 - 2072 property key ===
replacements.append((
    'DryadError::new(\n                     2072,\n                     "Esperado identificador ou string como chave da propriedade",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2095(), "Expected identifier or string as property key", SourceLocation::unknown())',
))

# === PARSE_OBJECT_PROPERTY: Line ~2804-2807 - 2073 obj method param ===
replacements.append((
    'DryadError::new(\n                                 2073,\n                                 "Esperado identificador de parâmetro",\n                             )',
    'DryadError::from_catalog_fmt(error_catalog::e2014(), "Expected parameter identifier", SourceLocation::unknown())',
))

# === PARSE_OBJECT_PROPERTY: Line ~2817-2820 - 2074 obj method param list ===
replacements.append((
    'DryadError::new(\n                                     2074,\n                                     "Esperado \',\' ou \')\' na lista de parâmetros",\n                                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2016(), "Expected \',\' or \')\' in parameter list", SourceLocation::unknown())',
))

# === PARSE_OBJECT_PROPERTY: Line ~2827 - 2075 obj method close ===
replacements.append((
    'DryadError::new(2075, "Esperado \')\' após parâmetros")',
    'DryadError::from_catalog(error_catalog::e2075(), SourceLocation::unknown())',
))

# === PARSE_OBJECT_PROPERTY: Line ~2839-2842 - 2076 obj method body ===
replacements.append((
    'DryadError::new(\n                         2076,\n                         "Esperado \'{\' após parâmetros do método",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2033(), "Expected \'{\' after method parameters", SourceLocation::unknown())',
))

# === PARSE_OBJECT_PROPERTY: Line ~2853-2856 - 2077 ===
replacements.append((
    'DryadError::new(\n                 2077,\n                 "Esperado \':\' ou \'(\' após chave da propriedade",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2095(), "Expected \':\' or \'(\' after property key", SourceLocation::unknown())',
))

# === TRY: Line ~2866 - 2080 ===
replacements.append((
    'DryadError::new(2080, "Esperado \'{\' após \'try\'")',
    'DryadError::from_catalog(error_catalog::e2080(), SourceLocation::unknown())',
))

# === TRY: Line ~2877 - 2081 ===
replacements.append((
    'DryadError::new(2081, "Esperado \'(\' após \'catch\'")',
    'DryadError::from_catalog(error_catalog::e2081(), SourceLocation::unknown())',
))

# === TRY: Line ~2884-2887 - 2082 catch var ===
replacements.append((
    'DryadError::new(\n                         2082,\n                         "Esperado nome da variável de exceção",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2082(), "Expected exception variable name", SourceLocation::unknown())',
))

# === TRY: Line ~2892 - 2083 catch close ===
replacements.append((
    'DryadError::new(2083, "Esperado \')\' após variável de catch")',
    'DryadError::from_catalog(error_catalog::e2083(), SourceLocation::unknown())',
))

# === TRY: Line ~2898-2901 - 2084 catch body ===
replacements.append((
    'DryadError::new(\n                     2084,\n                     "Esperado \'{\' após parâmetro de catch",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2080(), "Expected \'{\' after catch parameter", SourceLocation::unknown())',
))

# === TRY: Line ~2915 - 2085 ===
replacements.append((
    'DryadError::new(2085, "Esperado \'{\' após \'finally\'")',
    'DryadError::from_catalog(error_catalog::e2085(), SourceLocation::unknown())',
))

# === TRY: Line ~2922-2925 - 2086 ===
replacements.append((
    'DryadError::new(\n                 2086,\n                 "Bloco try deve ter pelo menos um catch ou finally",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2080(), "try block must have at least a catch or finally", SourceLocation::unknown())',
))

# === PROPERTY_ASSIGNMENT: Line ~2960-2963 - 2097 ===
replacements.append((
    'DryadError::new(\n                     2097,\n                     "Esperado \'this\' ou identificador para property assignment",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2095(), "Expected \'this\' or identifier for property assignment", SourceLocation::unknown())',
))

# === PROPERTY_ASSIGNMENT: Line ~2969-2972 - 2098 property dot ===
replacements.append((
    'DryadError::new(\n                 2098,\n                 "Esperado \'.\' após objeto para property assignment",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2095(), "Expected \'.\' after object for property assignment", SourceLocation::unknown())',
))

# === PROPERTY_ASSIGNMENT: Line ~2984-2987 - 2099 property name ===
replacements.append((
    'DryadError::new(\n                     2099,\n                     "Esperado nome da propriedade após \'.\'",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2095(), "Expected property name after \'.\'", SourceLocation::unknown())',
))

# === PROPERTY_ASSIGNMENT: Line ~2993-2996 - 2100 property assign ===
replacements.append((
    'DryadError::new(\n                 2100,\n                 "Esperado \'=\' para property assignment",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2104(), "Expected \'=\' for property assignment", SourceLocation::unknown())',
))

# === INDEX_ASSIGNMENT: Line ~3021-3024 - 2101 ===
replacements.append((
    'DryadError::new(\n                     2101,\n                     "Esperado identificador para index assignment",\n                 )',
    'DryadError::from_catalog_fmt(error_catalog::e2048(), "Expected identifier for index assignment", SourceLocation::unknown())',
))

# === INDEX_ASSIGNMENT: Line ~3030 - 2102 (DEDUP → 2048) ===
replacements.append((
    'DryadError::new(2102, "Esperado \'[\' após identificador")',
    'DryadError::from_catalog(error_catalog::e2048(), SourceLocation::unknown())',
))

# === INDEX_ASSIGNMENT: Line ~3039 - 2103 (DEDUP → 2049) ===
replacements.append((
    'DryadError::new(2103, "Esperado \']\' após índice")',
    'DryadError::from_catalog(error_catalog::e2049(), SourceLocation::unknown())',
))

# === INDEX_ASSIGNMENT: Line ~3045 - 2104 ===
replacements.append((
    'DryadError::new(2104, "Esperado \'=\' para index assignment")',
    'DryadError::from_catalog(error_catalog::e2104(), SourceLocation::unknown())',
))

# === USE: Line ~3070-3073 - 4002 (DEDUP → 2116) ===
replacements.append((
    'DryadError::new(\n                 4002,\n                 "Use deve ser seguido por uma string com o caminho do módulo",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Use must be followed by a string with the module path", SourceLocation::unknown())',
))

# === IMPORT: All 4002 occurrences → 2116 ===
# Line ~3109 - "Esperado 'as' após '*'"
replacements.append((
    'DryadError::new(4002, "Esperado \'as\' após \'*\' no import")',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected \'as\' after \'*\' in import", SourceLocation::unknown())',
))

# Line ~3122-3125 - "Esperado identificador após 'as'"
replacements.append((
    'DryadError::new(\n                         4002,\n                         "Esperado identificador após \'as\' no import",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected identifier after \'as\' in import", SourceLocation::unknown())',
))

# Line ~3131 - "Esperado 'from'"
replacements.append((
    'DryadError::new(4002, "Esperado \'from\' no import")',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected \'from\' in import", SourceLocation::unknown())',
))

# Line ~3144-3147 - "Esperado string com caminho do módulo após 'from'"
replacements.append((
    'DryadError::new(\n                         4002,\n                         "Esperado string com caminho do módulo após \'from\'",\n                     )',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected string with module path after \'from\'", SourceLocation::unknown())',
))

# Line ~3169-3172 - "Esperado identificador na lista de imports"
replacements.append((
    'DryadError::new(\n                             4002,\n                             "Esperado identificador na lista de imports",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected identifier in import list", SourceLocation::unknown())',
))

# Line ~3186-3189 - "Esperado ',' ou '}'"
replacements.append((
    'DryadError::new(\n                                 4002,\n                                 "Esperado \',\' ou \'}\' na lista de imports",\n                             )',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected \',\' or \'}\' in import list", SourceLocation::unknown())',
))

# Line ~3212-3215 - "Esperado string..." (named import from)
replacements.append((
    'DryadError::new(\n                         4002,\n                         "Esperado string com caminho do módulo após \'from\'",',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Expected string with module path after \'from\'", SourceLocation::unknown()),',
))

# Line ~3225-3228 - "Import deve ser seguido..."
replacements.append((
    'DryadError::new(\n                 4002,\n                 "Import deve ser seguido por \'{\', \'*\' ou string",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2116(), "Import must be followed by \'{\', \'*\', or string", SourceLocation::unknown())',
))

# === TEMPLATE_STRING: Line ~3250-3253 - 2027 interpolation close ===
replacements.append((
    'DryadError::new(\n                             2027,\n                             "Esperado \'}\' após interpolação em template string",\n                         )',
    'DryadError::from_catalog_fmt(error_catalog::e2028(), "Expected \'}\' after interpolation in template string", SourceLocation::unknown())',
))

# === TEMPLATE_STRING: Line ~3258 - 2028 ===
replacements.append((
    '_ => return Err(DryadError::new(2028, "Token inesperado em template string")),',
    '_ => return Err(DryadError::from_catalog(error_catalog::e2028(), SourceLocation::unknown())),',
))

# === TEMPLATE_STRING: Line ~3273 - 1002 (DEDUP → 2117) ===
replacements.append((
    'DryadError::new(1002, "Template string não fechada")',
    'DryadError::from_catalog(error_catalog::e2117(), SourceLocation::unknown())',
))

# === MATCH: Line ~3287-3290 - 2034 ===
replacements.append((
    'DryadError::new(\n                 2034,\n                 "Esperado \'{\' após expressão do match",\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2036(), "Expected \'{\' after match expression", SourceLocation::unknown())',
))

# === MATCH: Line ~3309 - 2035 ===
replacements.append((
    'DryadError::new(2035, "Esperado \'=>\' após padrão do match")',
    'DryadError::from_catalog(error_catalog::e2035(), SourceLocation::unknown())',
))

# === MATCH: Line ~3339 - 2036 ===
replacements.append((
    'DryadError::new(2036, "Esperado \'}\' para fechar o match")',
    'DryadError::from_catalog(error_catalog::e2036(), SourceLocation::unknown())',
))

# === PATTERN: Line ~3390 - 2037 ===
replacements.append((
    'DryadError::new(2037, "Esperado \']\' após padrões do array")',
    'DryadError::from_catalog(error_catalog::e2037(), SourceLocation::unknown())',
))

# === PATTERN: Line ~3409 - 2038 ===
replacements.append((
    'DryadError::new(2038, "Esperado \')\' após padrões da tupla")',
    'DryadError::from_catalog(error_catalog::e2038(), SourceLocation::unknown())',
))

# === PATTERN object: Line ~3428-3431 - 2039 key ===
replacements.append((
    'DryadError::new(\n                                 2039,\n                                 "Esperado chave do objeto no padrão",\n                             )',
    'DryadError::from_catalog_fmt(error_catalog::e2041(), "Expected object key in pattern", SourceLocation::unknown())',
))

# === PATTERN object: Line ~3435-3438 - 2040 colon ===
replacements.append((
    'DryadError::new(\n                                 2040,\n                                 "Esperado \':\' após chave do objeto no padrão",\n                             )',
    'DryadError::from_catalog_fmt(error_catalog::e2041(), "Expected \':\' after object key in pattern", SourceLocation::unknown())',
))

# === PATTERN: Line ~3453 - 2041 ===
replacements.append((
    'DryadError::new(2041, "Esperado \'}\' após padrões do objeto")',
    'DryadError::from_catalog(error_catalog::e2041(), SourceLocation::unknown())',
))

# === PATTERN: Line ~3458-3461 - 2042 invalid pattern ===
replacements.append((
    'DryadError::new(\n                 2042,\n                 &format!("Padrão inválido: {:?}", self.peek()),\n             )',
    'DryadError::from_catalog_fmt(error_catalog::e2042(), &format!("Invalid pattern: {:?}", self.peek()), SourceLocation::unknown())',
))


# Apply all replacements
for old, new in replacements:
    if old in content:
        content = content.replace(old, new, 1)
        count += 1
    else:
        print(f"WARNING: Not found: {old[:80]}...")

with open('crates/dryad_parser/src/parser.rs', 'w') as f:
    f.write(content)

print(f"Applied {count} replacements")
