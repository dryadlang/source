# IPE — TODO

Resumo
- Checklist para acompanhar o status da implementação do *Ipe* (FFI native + wrapper Dryad + demo).

## ✅ Feito
- [x] `ipe/lib/ipe.dryad` — wrapper Dryad que usa `ffi_load_library` / `ffi_call`.
- [x] `ipe/native/ipe_helper.c` — implementação nativa (exports: ipe_init, ipe_window_create, ipe_clear_background, ipe_draw_rect, ipe_process_events, ipe_is_window_open, ipe_window_close).
- [x] `ipe/native/Makefile` presente; `ipe.dll` já existe no repositório.
- [x] `ipe/tests/demo.dryad` — demo/example que demonstra a API do ipe.
- [x] Correções no runtime Dryad necessárias para FFI e execução (pattern matches, getters/setters, Expr::Spread, Value::Result, etc.).

## ⚠️ Observações atuais
- `ipe/tests/demo.dryad` atualmente falha na análise sintática (E2003: "Expected ';' after declaration") — precisa correção de sintaxe antes do run.
- Embora `ipe.dll` esteja no repositório, é recomendado rebuild local e validação dos símbolos exportados.

## ⬜ Pendente — Prioridade ALTA
- [ ] Compilar e validar `ipe.dll` localmente (`ipe/native`).
- [ ] Corrigir sintaxe em `ipe/tests/demo.dryad` e executar a demo end‑to‑end.
- [ ] Teste de integração automatizado (load library → ipe_init → createWindow → isOpen → close).
- [ ] Adicionar job de CI (Windows runner) para build da DLL e testes de integração.

## ⬜ Pendente — Prioridade MÉDIA
- [ ] Testes unitários para FFI (strings, ponteiros, null returns, símbolo inexistente).
- [ ] Melhorar mensagens de erro/validação em `ffi_load_library` / `ffi_call`.
- [ ] Tornar o build nativo multiplataforma (sugerir CMake ou script build.rs).

## ⬜ Pendente — Prioridade BAIXA (Nice-to-have)
- [ ] Harness headless / mock para testes de janela em CI.
- [ ] Documentação do módulo `ipe` (API, exemplos, pré-requisitos do sistema).
- [ ] Publicar exemplo Oak/Ipe e empacotar como módulo opcional.

## Próximos passos imediatos (recomendado)
1. Executar `make` em `ipe/native` e verificar símbolos exportados (alta prioridade).
2. Corrigir `ipe/tests/demo.dryad` (parser error) e reexecutar demo com `cargo run --bin dryad run ipe/tests/demo.dryad`.
3. Escrever teste de integração automático que verifica as chamadas FFI básicas.

## Comandos úteis
- cd ipe/native && mingw32-make OR make
- cargo run --bin dryad run ipe/tests/demo.dryad
- cargo test -p dryad_runtime

---

Notas
- Marque itens como concluídos quando o DLL for reconstruído e o demo rodar sem erros.
- Se preferir, adiciono testes automáticos e o job de CI em seguida.
