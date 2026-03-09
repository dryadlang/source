# IPE Framework - Biblioteca Gráfica Nativa para Dryad

## 📋 Visão Geral

IPE é um framework GUI nativo que permite criar interfaces gráficas simples e poderosas usando a linguagem Dryad através de FFI (Foreign Function Interface). Oferece suporte multiplataforma (Windows, Linux, macOS) com aceleração de hardware via SDL2.

## 🚀 Início Rápido

1. **Compilar a biblioteca:**
   - Windows: `cd ipe/native && build.cmd`
   - Unix: `cd ipe/native && ./build.sh`
2. **Executar demo:**
   - `dryad run ipe/tests/demo.dryad`

## 📚 Documentação

Toda a documentação, referências de API e guias de desenvolvedor foram consolidados em um único arquivo:
👉 **[DOCUMENTATION.md](./DOCUMENTATION.md)**

Para assistência via IA, utilize o arquivo:
🤖 **[llm.txt](./llm.txt)**

## 🧪 Testes

- **Suite de Integração**: `dryad run ipe/tests/test_integration.dryad`
- **Novos Recursos**: `dryad run ipe/tests/test_new_features.dryad`

## 🏗️ Estrutura

- `lib/`: Módulo Dryad (FFI bindings)
- `native/`: Código fonte C e build scripts
- `tests/`: Scripts de demonstração e testes
- `docs/`: Documentos auxiliares e referências

---

**Pedro Jesus** | Licensed under MIT
