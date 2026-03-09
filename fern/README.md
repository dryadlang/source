# 🌿 Fern - Biblioteca Matemática para Dryad

**Versão:** 0.1.0  
**Status:** ✅ Funcional e Testado  
**Linguagem:** 100% Dryad Puro  
**Licença:** MIT

---

## 📋 Visão Geral

Fern é a **primeira biblioteca matemática oficial** para a linguagem Dryad, unificando funcionalidades inspiradas nas principais bibliotecas Python para ciência de dados: NumPy, Pandas, Statsmodels, SymPy e SciPy.

### ✨ Características Principais

- 🚀 **Simplicidade**: API intuitiva similar ao Python científico
- 📦 **Unificada**: Todas as funcionalidades matemáticas em um único pacote
- ⚡ **Nativa**: 100% implementada em Dryad puro, sem dependências externas
- 🔬 **Científica**: Suporte completo para computação científica básica
- 📊 **Análise de Dados**: DataFrames estilo Pandas para manipulação de dados
- 🎯 **Modular**: Importe apenas os módulos que você precisa

### 🎯 Objetivos da Biblioteca

1. **Facilitar computação científica em Dryad**
2. **Fornecer ferramentas para análise de dados**
3. **Permitir cálculos matemáticos avançados**
4. **Servir como base para futuras bibliotecas científicas**

---

## 📦 Módulos e Funcionalidades

### 🔢 **math/arrays** - Arrays Multidimensionais (NumPy-like)

Manipulação de arrays multidimensionais com operações matemáticas.

**Funções de Criação:**

```dryad
import { zeros, ones, arange, eye } from "Fern/math/arrays";

let arr_zeros = zeros([3, 3]);        // Array 3x3 de zeros
let arr_ones = ones([2, 4]);          // Array 2x4 de uns
let seq = arange(0, 10, 1);           // [0, 1, 2, ..., 9]
let identity = eye(5);                // Matriz identidade 5x5
```

**Classe Array:**

```dryad
import { Array } from "Fern/math/arrays";

let arr = new Array([1, 2, 3, 4], [2, 2]);  // Array 2x2
let sum = arr.sum();                         // Soma de todos elementos
let mean = arr.mean();                       // Média
let min = arr.min();                         // Mínimo
let max = arr.max();                         // Máximo
arr.set([0, 1], 10);                        // Definir elemento
let val = arr.get([0, 1]);                  // Obter elemento
```

**Operações entre Arrays:**

```dryad
import { add_arrays, multiply_arrays, dot } from "Fern/math/arrays";

let a = arange(1, 5, 1);
let b = arange(5, 9, 1);
let sum_ab = add_arrays(a, b);        // Soma elemento a elemento
let prod_ab = multiply_arrays(a, b);  // Multiplicação elemento a elemento
let dot_prod = dot(a, b);             // Produto escalar
```

---

### 📊 **math/statistics** - Estatísticas (Pandas/Statsmodels-like)

Análise estatística completa de dados.

**Classe Statistics:**

```dryad
import { Statistics } from "Fern/math/statistics";

let data = [12, 15, 18, 22, 25, 28, 30, 35, 40, 45];
let stats = new Statistics(data);

// Medidas de tendência central
let media = stats.mean();           // Média aritmética
let mediana = stats.median();       // Mediana
let moda = stats.mode();            // Moda (valor mais frequente)

// Medidas de dispersão
let variancia = stats.variance();   // Variância
let desvio = stats.std_dev();      // Desvio padrão
let minimo = stats.min();          // Valor mínimo
let maximo = stats.max();          // Valor máximo

// Quartis
let q1 = stats.quartile(0.25);     // Primeiro quartil
let q2 = stats.quartile(0.5);      // Segundo quartil (mediana)
let q3 = stats.quartile(0.75);     // Terceiro quartil
let iqr = stats.iqr();             // Intervalo interquartil
```

**Funções de Correlação:**

```dryad
import { correlation, covariance } from "Fern/math/statistics";

let x = [1, 2, 3, 4, 5];
let y = [2, 4, 6, 8, 10];

let corr = correlation(x, y);      // Correlação de Pearson
let cov = covariance(x, y);        // Covariância
```

**Distribuições Estatísticas:**

```dryad
import { normal_pdf, normal_cdf, t_test } from "Fern/math/statistics";

let prob = normal_pdf(0, 0, 1);           // PDF normal padrão em x=0
let cdf = normal_cdf(1.96, 0, 1);         // CDF normal padrão
let result = t_test(sample1, sample2);     // Teste t de Student
```

---

### 🔣 **symbolic/algebra** - Álgebra Simbólica (SymPy-like)

Manipulação simbólica de expressões matemáticas.

**Criação de Expressões:**

```dryad
import { symbol, number, add, multiply, power } from "Fern/symbolic/algebra";

// Criar variável simbólica
let x = symbol("x");

// Criar expressão: f(x) = 2x² + 3x + 1
let expr = add(
    add(
        multiply(number(2), power(x, number(2))),
        multiply(number(3), x)
    ),
    number(1)
);

print("f(x) = " + expr.to_string());
```

**Avaliação de Expressões:**

```dryad
// Avaliar em x = 3
let vars = {"x": 3};
let resultado = expr.evaluate(vars);
print("f(3) = " + resultado);  // 2(3²) + 3(3) + 1 = 28
```

**Cálculo Diferencial:**

```dryad
// Derivada simbólica
let derivada = expr.derivative("x");
print("f'(x) = " + derivada.to_string());  // 4x + 3

// Avaliar derivada
print("f'(3) = " + derivada.evaluate(vars));  // 15
```

**Integração:**

```dryad
// Integração de polinômios simples
let integral = power(x, number(2)).integrate("x");
print("∫x² dx = " + integral.to_string());  // x³/3
```

**Funções Auxiliares:**

```dryad
import { solve_linear, factor_quadratic, simplify } from "Fern/symbolic/algebra";

// Resolver equação linear: 2x + 5 = 0
let solucao = solve_linear(2, 5);  // x = -2.5

// Fatoração de quadráticas
let fatores = factor_quadratic(1, -5, 6);  // (x-2)(x-3)

// Simplificação (básica)
let simplificado = simplify(expr);
```

---

### 🔬 **scientific/functions** - Funções Científicas (SciPy-like)

Funções matemáticas avançadas e cálculo numérico.

**Constantes Matemáticas:**

```dryad
import { PI, E, GOLDEN_RATIO, EULER_GAMMA } from "Fern/scientific/functions";

print("π = " + PI);                    // 3.141592653589793
print("e = " + E);                     // 2.718281828459045
print("φ = " + GOLDEN_RATIO);          // 1.618033988749895
print("γ = " + EULER_GAMMA);           // 0.5772156649015329
```

**Funções Trigonométricas:**

```dryad
import { sin, cos, tan, asin, acos } from "Fern/scientific/functions";

let seno = sin(PI/2);          // 1.0
let cosseno = cos(0);          // 1.0
let tangente = tan(PI/4);      // ~1.0
let arco_seno = asin(0.5);     // ~0.524 (30°)
let arco_cos = acos(0.5);      // ~1.047 (60°)
```

**Funções Hiperbólicas:**

```dryad
import { sinh, cosh, tanh } from "Fern/scientific/functions";

let senh = sinh(1);            // Seno hiperbólico
let cosh_val = cosh(1);        // Cosseno hiperbólico
let tanh_val = tanh(1);        // Tangente hiperbólica
```

**Funções Exponenciais e Logarítmicas:**

```dryad
import { exp, ln, log10, pow, sqrt } from "Fern/scientific/functions";

let exponencial = exp(2);      // e²
let logaritmo = ln(E);         // 1.0
let log_10 = log10(100);       // 2.0
let potencia = pow(2, 10);     // 1024
let raiz = sqrt(16);           // 4.0
```

**Funções Especiais:**

```dryad
import { factorial, gamma, erf, erfc, bessel_j0, zeta } from "Fern/scientific/functions";

let fatorial = factorial(5);           // 120
let gamma_val = gamma(5);              // 24 (para inteiros: (n-1)!)
let erro = erf(1);                     // Função erro
let erro_comp = erfc(1);               // Função erro complementar
let bessel = bessel_j0(0);             // Função de Bessel J0
let zeta_val = zeta(2);                // Função zeta de Riemann
```

**Cálculo Numérico:**

```dryad
import { integrate_numerical, derivative_numerical, find_root } from "Fern/scientific/functions";

// Integração numérica (método do trapézio)
let integral = integrate_numerical(sin, 0, PI, 100);  // ∫sin(x)dx de 0 a π

// Derivada numérica
let derivada = derivative_numerical(sin, PI/2, 0.001);  // cos(π/2) ≈ 0

// Encontrar raiz (método da bisseção)
let raiz = find_root(function(x) { return x*x - 2; }, 0, 2, 0.001);  // √2
```

---

### 📋 **data/frames** - DataFrames (Pandas-like)

Estruturas de dados tabulares para análise de dados.

**Criação de DataFrames:**

```dryad
import { DataFrame, from_dict, read_csv_simple } from "Fern/data/frames";

// A partir de dicionário
let df = from_dict({
    "produto": ["Notebook", "Mouse", "Teclado", "Monitor"],
    "preco": [2500, 50, 150, 800],
    "estoque": [10, 50, 30, 15]
});

// A partir de CSV (formato simples)
let csv_text = "nome,idade,salario\nAna,25,3000\nJoão,30,3500";
let df_csv = read_csv_simple(csv_text);
```

**Operações Básicas:**

```dryad
// Obter coluna
let precos = df.get_column("preco");

// Obter linha por índice
let primeira_linha = df.get_row(0);

// Adicionar linha
df.add_row(["Webcam", 200, 20]);

// Definir valor em coluna
let novos_precos = [2600, 55, 160, 850];
df.set_column("preco", novos_precos);
```

**Filtragem e Seleção:**

```dryad
// Filtrar linhas com condição
let produtos_caros = df.filter(function(row) {
    return row[1] > 100;  // preco > 100
});

// Ordenar por coluna
let df_ordenado = df.sort_by("preco", true);  // ascendente
```

**Estatísticas Descritivas:**

```dryad
// Estatísticas de uma coluna
let estatisticas = df.describe("preco");
print("Média: " + estatisticas["mean"]);
print("Min: " + estatisticas["min"]);
print("Max: " + estatisticas["max"]);
print("Std: " + estatisticas["std"]);
```

**Agrupamento:**

```dryad
// Agrupar e agregar
let agrupado = df.groupby("categoria", function(valores) {
    let soma = 0;
    for (let i = 0; i < valores.length; i++) {
        soma = soma + valores[i];
    }
    return soma;
});
```

**Merge/Join:**

```dryad
import { merge } from "Fern/data/frames";

let df1 = from_dict({
    "id": [1, 2, 3],
    "nome": ["Ana", "João", "Maria"]
});

let df2 = from_dict({
    "id": [1, 2, 3],
    "salario": [3000, 3500, 3200]
});

// Mesclar DataFrames
let df_completo = merge(df1, df2, "id");
```

**Visualização:**

```dryad
// Representação em string (para debug/print)
print(df.to_string());
```

---

## 🚀 Instalação

### Método 1: Instalação via Oak Package Manager (Recomendado)

**Pré-requisitos:**

- Dryad instalado e funcionando
- Oak Package Manager compilado

**Passo a Passo:**

```bash
# 1. Criar um novo projeto Dryad
oak init meu-projeto
cd meu-projeto

# 2. Instalar o Fern
oak install Fern

# 3. Gerar o arquivo de lock com mapeamento dos módulos
oak lock

# 4. Verificar instalação
oak list
```

**Estrutura gerada:**

```
meu-projeto/
├── oaklibs.json          # Configuração do projeto
├── oaklock.json          # Mapeamento de módulos
├── main.dryad            # Seu código
└── oak_modules/
    └── Fern/        # Biblioteca instalada
        └── src/
            ├── lib.dryad
            ├── math/
            ├── data/
            ├── scientific/
            └── symbolic/
```

### Método 2: Instalação Local (Desenvolvimento)

Para desenvolvimento ou testes locais:

```bash
# 1. Clone o repositório Fern
git clone https://github.com/Dryad-lang/Fern.git
cd Fern

# 2. Em seu projeto, crie um link simbólico ou copie os arquivos
# Windows (PowerShell como admin):
New-Item -ItemType SymbolicLink -Path "oak_modules\Fern" -Target "C:\caminho\para\Fern"

# Linux/Mac:
ln -s /caminho/para/Fern oak_modules/Fern

# Ou simplesmente copie:
cp -r /caminho/para/Fern oak_modules/Fern

# 3. Gere o oaklock.json
oak lock
```

### Método 3: Instalação Manual (Sem Oak)

Se você não está usando o Oak Package Manager:

```bash
# 1. Baixe os arquivos da biblioteca
# 2. Copie para uma pasta 'libs' no seu projeto
# 3. Use includes relativos no seu código
```

**Estrutura manual:**

```
meu-projeto/
├── main.dryad
└── libs/
    └── Fern/
        └── src/
            ├── math/
            ├── data/
            └── ...
```

### Verificando a Instalação

Crie um arquivo de teste `test_Fern.dryad`:

```dryad
// Teste de instalação do Fern
import { zeros, ones } from "Fern/math/arrays";
import { PI, E } from "Fern/scientific/functions";

print("✅ Fern instalado com sucesso!");
print("Teste: zeros(3) = " + zeros(3));
print("PI = " + PI);
```

Execute:

```bash
oak exec test_Fern.dryad
```

Se você ver a saída sem erros, o Fern está instalado corretamente! ✅

---

## 📚 Exemplos de Uso

### Exemplo Completo: Análise Estatística

```dryad
// Carregar módulos
// #include "Fern/src/math/statistics.dryad"
// #include "Fern/src/data/frames.dryad"

print("=== Análise Estatística com Fern ===");

// Dados de exemplo
let vendas = [120, 150, 130, 180, 160, 140, 170, 190, 165, 175];
let custos = [80, 90, 85, 110, 95, 88, 105, 120, 100, 108];

// Estatísticas básicas
let stats_vendas = new Statistics(vendas);
let stats_custos = new Statistics(custos);

print("Vendas - Média: " + stats_vendas.mean() + ", Desvio: " + stats_vendas.std_dev());
print("Custos - Média: " + stats_custos.mean() + ", Desvio: " + stats_custos.std_dev());

// Correlação
let correlacao = correlation(vendas, custos);
print("Correlação Vendas-Custos: " + correlacao);

// DataFrame
let df = from_dict({
    "mes": ["Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out"],
    "vendas": vendas,
    "custos": custos,
    "lucro": vendas.map(function(v, i) { return v - custos[i]; })
});

print("DataFrame criado:");
print(df.to_string());

print("Estatísticas descritivas de lucro:");
print(df.describe("lucro"));
```

### Exemplo: Álgebra Simbólica

```dryad
// #include "Fern/src/symbolic/algebra.dryad"

print("=== Álgebra Simbólica ===");

// f(x) = 2x^2 + 3x + 1
let x = symbol("x");
let two = number(2);
let three = number(3);
let one = number(1);

let x_squared = power(x, number(2));
let two_x_squared = multiply(two, x_squared);
let three_x = multiply(three, x);
let expr = add(add(two_x_squared, three_x), one);

print("Expressão: " + expr.to_string());

// Avaliar
let vars = {"x": 2};
print("f(2) = " + expr.evaluate(vars));

// Derivada
let deriv = expr.derivative("x");
print("f'(x) = " + deriv.to_string());
print("f'(2) = " + deriv.evaluate(vars));
```

---

## 📚 Exemplos Práticos Completos

### Exemplo 1: Análise Estatística de Vendas

```dryad
import { Statistics, correlation } from "Fern/math/statistics";
import { from_dict } from "Fern/data/frames";

print("=== Análise de Vendas Mensal ===\n");

// Dados de vendas e custos
let vendas = [120, 150, 130, 180, 160, 140, 170, 190, 165, 175];
let custos = [80, 90, 85, 110, 95, 88, 105, 120, 100, 108];

// Calcular estatísticas
let stats_vendas = new Statistics(vendas);
let stats_custos = new Statistics(custos);

print("📊 Vendas:");
print("  Média: R$ " + stats_vendas.mean());
print("  Mediana: R$ " + stats_vendas.median());
print("  Desvio Padrão: R$ " + stats_vendas.std_dev());
print("  Min: R$ " + stats_vendas.min());
print("  Max: R$ " + stats_vendas.max());

print("\n💰 Custos:");
print("  Média: R$ " + stats_custos.mean());
print("  Desvio Padrão: R$ " + stats_custos.std_dev());

// Correlação entre vendas e custos
let corr = correlation(vendas, custos);
print("\n📈 Correlação Vendas-Custos: " + corr);

// Criar DataFrame para análise
let df = from_dict({
    "mes": ["Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out"],
    "vendas": vendas,
    "custos": custos
});

// Calcular lucro
let lucros = [];
for (let i = 0; i < vendas.length; i++) {
    lucros = lucros + [vendas[i] - custos[i]];
}
df.set_column("lucro", lucros);

// Filtrar meses lucrativos (lucro > 70)
let meses_bons = df.filter(function(row) {
    return row[3] > 70;  // índice 3 = lucro
});

print("\n✅ Meses com lucro > R$ 70:");
print(meses_bons.to_string());
```

### Exemplo 2: Cálculo Numérico e Simulação

```dryad
import { sin, cos, PI, integrate_numerical, find_root } from "Fern/scientific/functions";
import { arange, Array } from "Fern/math/arrays";

print("=== Simulação Física: Movimento Harmônico ===\n");

// Parâmetros
let amplitude = 10;
let frequencia = 2;
let tempo_total = 2 * PI;

// Função de posição: x(t) = A * sin(ωt)
function posicao(t) {
    return amplitude * sin(frequencia * t);
}

// Função de velocidade: v(t) = A * ω * cos(ωt)
function velocidade(t) {
    return amplitude * frequencia * cos(frequencia * t);
}

// Gerar pontos no tempo
let n_pontos = 20;
let tempo = arange(0, tempo_total, tempo_total / n_pontos);

print("📊 Tabela de Movimento:");
print("Tempo\tPosição\tVelocidade");
for (let i = 0; i < n_pontos; i++) {
    let t = tempo.data[i];
    let x = posicao(t);
    let v = velocidade(t);
    print(t + "\t" + x + "\t" + v);
}

// Calcular deslocamento total (integral da velocidade absoluta)
let deslocamento = integrate_numerical(
    function(t) {
        let v = velocidade(t);
        if (v < 0) {
            return -v;
        }
        return v;
    },
    0,
    tempo_total,
    100
);

print("\n📏 Deslocamento total: " + deslocamento + " unidades");

// Encontrar quando passa pela origem (x = 0)
let tempo_zero = find_root(posicao, 0.1, 1.0, 0.001);
print("⏱️  Primeira passagem por x=0: t = " + tempo_zero + " s");
```

### Exemplo 3: Álgebra Simbólica - Análise de Função

```dryad
import { symbol, number, add, multiply, power, divide } from "Fern/symbolic/algebra";

print("=== Análise Simbólica de Função ===\n");

// Criar variável
let x = symbol("x");

// Definir função: f(x) = x³ - 6x² + 11x - 6
let f = add(
    add(
        power(x, number(3)),
        multiply(number(-6), power(x, number(2)))
    ),
    add(
        multiply(number(11), x),
        number(-6)
    )
);

print("📐 Função: f(x) = " + f.to_string());

// Calcular derivada primeira
let f_linha = f.derivative("x");
print("📈 Derivada: f'(x) = " + f_linha.to_string());

// Calcular derivada segunda
let f_duas_linhas = f_linha.derivative("x");
print("📊 Segunda derivada: f''(x) = " + f_duas_linhas.to_string());

// Avaliar em diversos pontos
print("\n📋 Tabela de Valores:");
print("x\tf(x)\tf'(x)\tf''(x)");

let pontos = [0, 1, 2, 3, 4];
for (let i = 0; i < pontos.length; i++) {
    let x_val = pontos[i];
    let vars = {"x": x_val};

    let y = f.evaluate(vars);
    let dy = f_linha.evaluate(vars);
    let ddy = f_duas_linhas.evaluate(vars);

    print(x_val + "\t" + y + "\t" + dy + "\t" + ddy);
}

// Análise de concavidade
print("\n🔍 Análise:");
let vars_1 = {"x": 1};
let vars_2 = {"x": 2};

let f2_1 = f_duas_linhas.evaluate(vars_1);
let f2_2 = f_duas_linhas.evaluate(vars_2);

if (f2_1 > 0) {
    print("  Em x=1: Concavidade para cima");
} else {
    print("  Em x=1: Concavidade para baixo");
}

if (f2_2 > 0) {
    print("  Em x=2: Concavidade para cima");
} else {
    print("  Em x=2: Concavidade para baixo");
}
```

### Exemplo 4: Análise de Dados com DataFrame

```dryad
import { from_dict, merge } from "Fern/data/frames";
import { Statistics, correlation } from "Fern/math/statistics";

print("=== Sistema de Análise de Produtos ===\n");

// Dados de produtos
let produtos_df = from_dict({
    "id": [1, 2, 3, 4, 5],
    "nome": ["Notebook", "Mouse", "Teclado", "Monitor", "Webcam"],
    "categoria": ["PC", "Acessorio", "Acessorio", "PC", "Acessorio"],
    "preco": [2500, 50, 150, 800, 200]
});

// Dados de vendas
let vendas_df = from_dict({
    "id": [1, 2, 3, 4, 5],
    "quantidade": [50, 200, 150, 80, 120],
    "avaliacao": [4.5, 4.2, 4.8, 4.6, 4.0]
});

// Mesclar DataFrames
let df_completo = merge(produtos_df, vendas_df, "id");

print("📊 Dataset Completo:");
print(df_completo.to_string());

// Calcular receita
let receitas = [];
for (let i = 0; i < df_completo.data.length; i++) {
    let preco = df_completo.data[i][3];     // índice do preço
    let qtd = df_completo.data[i][5];       // índice da quantidade
    receitas = receitas + [preco * qtd];
}
df_completo.set_column("receita", receitas);

print("\n💰 Análise de Receita:");
let estatisticas = df_completo.describe("receita");
print("  Total: R$ " + estatisticas["sum"]);
print("  Média por produto: R$ " + estatisticas["mean"]);
print("  Maior receita: R$ " + estatisticas["max"]);

// Filtrar produtos mais vendidos (quantidade > 100)
let top_produtos = df_completo.filter(function(row) {
    return row[5] > 100;  // quantidade
});

print("\n🏆 Produtos Mais Vendidos (qtd > 100):");
print(top_produtos.to_string());

// Correlação preço x avaliação
let precos = df_completo.get_column("preco");
let avaliacoes = df_completo.get_column("avaliacao");
let corr = correlation(precos, avaliacoes);

print("\n📈 Correlação Preço-Avaliação: " + corr);
if (corr > 0) {
    print("  ✅ Produtos mais caros tendem a ter melhor avaliação");
} else {
    print("  ⚠️  Produtos mais caros tendem a ter pior avaliação");
}

// Ordenar por receita
let df_ordenado = df_completo.sort_by("receita", false);  // descendente
print("\n📋 Top 3 Produtos por Receita:");
for (let i = 0; i < 3; i++) {
    let row = df_ordenado.get_row(i);
    print("  " + (i+1) + ". " + row[1] + ": R$ " + row[6]);
}
```

---

## 🔧 Uso com Oak Package Manager

### Importando Módulos

Após instalar o Fern, você pode importar módulos de três formas:

**1. Named Imports (Recomendado):**

```dryad
// Importar funções específicas
import { zeros, ones, arange } from "Fern/math/arrays";
import { sin, cos, PI } from "Fern/scientific/functions";
import { Statistics } from "Fern/math/statistics";

let arr = zeros(5);
let seno = sin(PI/2);
let stats = new Statistics([1, 2, 3, 4, 5]);
```

**2. Namespace Imports:**

```dryad
// Importar módulo completo com alias
import * as arrays from "Fern/math/arrays";
import * as sci from "Fern/scientific/functions";

let arr = arrays.zeros(5);
let seno = sci.sin(sci.PI/2);
```

**3. Side-effect Imports:**

```dryad
// Apenas executar o módulo (útil para inicialização)
import "Fern/lib";
```

### Estrutura do oaklock.json

Após executar `oak lock`, o arquivo `oaklock.json` mapeia todos os módulos:

```json
{
  "modules": {
    "Fern": {
      "paths": {
        "lib": "./oak_modules/Fern/src/lib.dryad",
        "math/arrays": "./oak_modules/Fern/src/math/arrays.dryad",
        "math/statistics": "./oak_modules/Fern/src/math/statistics.dryad",
        "data/frames": "./oak_modules/Fern/src/data/frames.dryad",
        "scientific/functions": "./oak_modules/Fern/src/scientific/functions.dryad",
        "symbolic/algebra": "./oak_modules/Fern/src/symbolic/algebra.dryad"
      }
    }
  }
}
```

---

## 🐛 Troubleshooting

### Erro: "oaklock.json não encontrado"

**Problema:** O Oak não consegue resolver os módulos.

**Solução:**

```bash
# Execute oak lock para gerar o arquivo de mapeamento
oak lock
```

### Erro: "Módulo não encontrado"

**Problema:** O caminho do import está incorreto.

**Soluções:**

1. Verifique o `oaklock.json` para ver os caminhos disponíveis
2. Use o formato correto: `"Fern/modulo/submodulo"`
3. Execute `oak list` para ver os módulos instalados

### Erro: "Função não definida"

**Problema:** Tentando usar uma função que não foi importada.

**Solução:**

```dryad
// ❌ Errado - função não importada
import { zeros } from "Fern/math/arrays";
let arr = ones(5);  // Erro: ones não definida

// ✅ Correto - importar todas as funções necessárias
import { zeros, ones } from "Fern/math/arrays";
let arr1 = zeros(5);
let arr2 = ones(5);
```

### Erro de Sintaxe: "Esperado '(' após 'while'"

**Problema:** Código Fern usando sintaxe antiga.

**Nota:** A versão instalada via Oak já vem com sintaxe correta. Se encontrar este erro:

1. Reinstale: `oak install Fern --force`
2. Execute: `oak lock`

### Performance Lenta em Arrays Grandes

**Limitação:** Fern v0.1.0 é otimizado para arrays pequenos a médios (até ~100 elementos).

**Recomendações:**

- Para arrays grandes, considere usar funções nativas quando disponíveis
- Versões futuras terão otimizações de performance

---

## 🗺️ Roadmap

### Versão 0.1.0 (Atual) ✅

- [x] Arrays multidimensionais básicos
- [x] Estatísticas descritivas
- [x] DataFrames simples
- [x] Álgebra simbólica básica
- [x] Funções científicas essenciais
- [x] Integração com Oak Package Manager

### Versão 0.2.0 (Planejado)

- [ ] Operações vetorizadas otimizadas
- [ ] Suporte a arrays maiores (1000+ elementos)
- [ ] Funções de agregação em DataFrames
- [ ] Plots e visualização básica
- [ ] Mais funções de distribuição estatística
- [ ] Operações de matriz (determinante, inversa)

### Versão 0.3.0 (Futuro)

- [ ] Machine Learning básico (regressão linear, k-means)
- [ ] Séries temporais
- [ ] Análise de Fourier
- [ ] Otimização numérica
- [ ] Integração com módulos nativos para performance

### Versão 1.0.0 (Visão)

- [ ] Biblioteca completa e estável
- [ ] Performance otimizada
- [ ] Documentação completa
- [ ] Exemplos abrangentes
- [ ] Testes unitários completos

---

## 🤝 Contribuindo

Fern é open-source e aceita contribuições! Áreas onde você pode ajudar:

1. **Implementação de Funcionalidades:**
   - Adicionar novas funções matemáticas
   - Otimizar algoritmos existentes
   - Implementar novos módulos

2. **Documentação:**
   - Melhorar exemplos
   - Traduzir documentação
   - Criar tutoriais

3. **Testes:**
   - Adicionar casos de teste
   - Reportar bugs
   - Validar resultados matemáticos

4. **Performance:**
   - Otimizar funções críticas
   - Reduzir uso de memória
   - Melhorar algoritmos

### Como Contribuir

```bash
# 1. Fork o repositório no GitHub

# 2. Clone seu fork
git clone https://github.com/seu-usuario/Fern.git
cd Fern

# 3. Crie uma branch para sua feature
git checkout -b feature/minha-funcionalidade

# 4. Faça suas alterações e commit
git add .
git commit -m "Adiciona funcionalidade X"

# 5. Push para seu fork
git push origin feature/minha-funcionalidade

# 6. Abra um Pull Request no GitHub
```

### Guidelines

- **Código:** Siga o estilo existente do projeto
- **Comentários:** Documente funções complexas
- **Testes:** Adicione exemplos de uso
- **Commits:** Use mensagens descritivas

---

## 📄 Licença

Fern é licenciado sob a **MIT License**.

```
MIT License

Copyright (c) 2025 Dryad Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 📞 Suporte e Comunidade

- **GitHub Issues:** [Reportar bugs e sugerir features](https://github.com/Dryad-lang/Fern/issues)
- **Discussões:** [Fórum de discussão](https://github.com/Dryad-lang/Fern/discussions)
- **Documentação:** [Wiki do projeto](https://github.com/Dryad-lang/Fern/wiki)
- **Email:** suporte@dryadlang.org

---

## 🎯 Casos de Uso

Fern é ideal para:

- 📊 **Análise de Dados:** Processar e analisar datasets
- 🔬 **Computação Científica:** Cálculos matemáticos complexos
- 📈 **Estatística:** Análise estatística de dados
- 🎓 **Educação:** Ensino de matemática e programação
- 🧪 **Pesquisa:** Prototipagem rápida de algoritmos
- 💼 **Business Intelligence:** Análise de dados empresariais

---

## 🏆 Agradecimentos

Fern foi inspirado por excelentes bibliotecas Python:

- **NumPy:** Arrays multidimensionais e álgebra linear
- **Pandas:** Análise e manipulação de dados
- **SciPy:** Algoritmos científicos
- **SymPy:** Matemática simbólica
- **Statsmodels:** Modelagem estatística

Agradecimentos especiais à comunidade Dryad e aos contribuidores!

---

<div align="center">

**🌿 Fern v0.1.0** - Primeira biblioteca matemática da linguagem Dryad 🎉

Feito com ❤️ pela comunidade Dryad

[Homepage](https://github.com/Dryad-lang/Fern) • [Documentação](https://github.com/Dryad-lang/Fern/wiki) • [Exemplos](./examples) • [Issues](https://github.com/Dryad-lang/Fern/issues)

</div>

---

## 🔧 Funcionalidades Implementadas

### Arrays Multidimensionais

- ✅ Criação de arrays (zeros, ones, arange, eye)
- ✅ Operações básicas (+, -, \*, /) elemento a elemento
- ✅ Funções de agregação (sum, mean, min, max)
- ✅ Indexação básica
- ✅ Produto escalar (dot product)

### Estatísticas

- ✅ Média, mediana, moda
- ✅ Desvio padrão, variância
- ✅ Quartis, IQR
- ✅ Correlação de Pearson
- ✅ Covariância
- ✅ Distribuições normais (PDF, CDF aproximadas)
- ✅ Teste t simples

### Álgebra Simbólica

- ✅ Expressões simbólicas básicas
- ✅ Avaliação de expressões
- ✅ Derivação simbólica (regras básicas)
- ✅ Integração simples (polinômios)
- ✅ Resolução de equações lineares
- ✅ Fatoração quadrática simples

### Funções Científicas

- ✅ Funções trigonométricas (sin, cos, tan, asin, acos)
- ✅ Funções exponenciais e logarítmicas
- ✅ Potência e raízes
- ✅ Funções hiperbólicas
- ✅ Fatorial e função gamma
- ✅ Função erro (erf)
- ✅ Função de Bessel J0 (aproximada)
- ✅ Integração numérica (trapézio)
- ✅ Derivação numérica
- ✅ Método da bisseção para raízes

### DataFrames

- ✅ Criação de DataFrames
- ✅ Seleção de colunas e linhas
- ✅ Filtragem de dados
- ✅ Estatísticas descritivas
- ✅ GroupBy e agregações
- ✅ Ordenação
- ✅ Merge/join simples

---

## 🔮 Funcionalidades Planejadas

### Arrays

- Indexação avançada
- Broadcasting
- Operações de matriz (multiplicação, inversa, etc.)
- FFT básica

### Estatísticas

- Regressão linear
- Testes de hipóteses avançados
- Análise de variância (ANOVA)
- Distribuições adicionais

### Simbólico

- Simplificação algébrica avançada
- Equações diferenciais
- Séries de Taylor
- Sistemas de equações

### Científico

- Otimização (gradiente descendente, etc.)
- Interpolação
- Integração avançada
- EDOs numéricas

---

## 🐛 Limitações Atuais

- Implementações são simplificadas para educação e prototipagem
- Precisão numérica limitada pelas capacidades da linguagem Dryad
- Performance não otimizada
- Alguns algoritmos usam aproximações

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Para contribuir:

1. Fork o repositório
2. Crie uma branch para sua feature
3. Implemente em Dryad puro
4. Teste thoroughly
5. Submit um pull request

---

## 📄 Licença

MIT License - veja LICENSE para detalhes.

---

**Fern v0.1.0** - Primeira biblioteca matemática da linguagem Dryad 🎉
