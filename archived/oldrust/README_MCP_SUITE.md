# Dryad Language MCP Suite

**Complete documentation and tools for integrating Dryad with AI Language Models (LLMs) via Model Context Protocol (MCP)**

---

## 📦 What's Included

### 1. **DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md** (1,400+ lines)
**Complete, LLM-optimized Dryad language specification**

- **15 comprehensive sections** covering all language features
- **Token taxonomy** with lexical classification
- **Type system** with compatibility rules
- **Expression precedence** table
- **All statement types** (if, while, for, try-catch, etc.)
- **Declaration forms** (functions, classes, interfaces)
- **Module system** (import/export/use/#directives)
- **Runtime model** (execution flow, scoping, memory)
- **FFI and native functions** (C interop)
- **Error handling** patterns
- **Concurrency** (async/await, threads, mutexes)
- **15+ example programs**
- **Quick reference** (keywords, operators, types)

**Use this for:** System prompts, context injection, code generation, analysis

---

### 2. **DRYAD_MCP_INTEGRATION_GUIDE.md** (700+ lines)
**Detailed guide for implementing MCP integration**

- **MCP overview** and concepts
- **5 integration strategies:**
  - Specification as system context
  - Specification as reference tool
  - Specification as training data
  - Structured prompts
  - Iterative refinement
- **5 concrete use cases:**
  - Code generation
  - Code analysis
  - Debugging
  - Feature explanation
  - Code translation
- **Prompt templates** for each use case
- **MCP resource structure** (URI mapping)
- **LLM best practices**
- **Example: Claude integration code**
- **Deployment checklist**

**Use this for:** Building MCP servers, implementing AI assistants, designing LLM workflows

---

### 3. **dryad_mcp_server.py** (500+ lines)
**Production-ready MCP server implementation**

A fully functional Python MCP server with:

**Resources:**
- Complete specification (all sections)
- 15 section-specific resources
- Quick reference
- Tools documentation

**Tools:**
- `validate_dryad` - Check syntax
- `analyze_dryad` - Semantic analysis
- `lookup_syntax` - Pattern lookup
- `get_keywords` - Keyword list
- `quick_reference` - One-page reference

**Features:**
- Specification section caching
- Graceful error handling
- CLI integration (dryad validator)
- Environment configuration
- Ready to run with MCP SDK

**Use this for:** Immediate deployment, local development, testing

---

## 🚀 Quick Start

### Option 1: Use Specification Directly

```python
# Load specification into your system prompt
with open("DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md") as f:
    spec = f.read()

system_prompt = f"""You are an expert Dryad programmer.

{spec}

When generating code, follow the exact syntax and semantics specified above.
"""
```

### Option 2: Deploy MCP Server

```bash
# Install MCP SDK
pip install mcp

# Run the server
python3 dryad_mcp_server.py

# Connect your LLM tool via MCP to the server
```

### Option 3: Use Integration Guide

Follow `DRYAD_MCP_INTEGRATION_GUIDE.md` to:
1. Choose integration strategy
2. Implement MCP server (use provided example or build custom)
3. Set up prompt templates
4. Deploy and test

---

## 📚 Document Organization

### DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md

```
1.  Language Overview (characteristics, design principles)
2.  Lexical Structure (tokens, keywords, operators, literals)
3.  Type System (primitives, composites, annotations, compatibility)
4.  Expressions (operators, precedence, binary/unary/ternary/lambda)
5.  Statements (if/while/for/try-catch/throw/return/block)
6.  Declarations (functions, classes, interfaces, namespaces)
7.  Modules and Imports (import/export/use/#directives)
8.  Runtime Execution Model (flow, scoping, memory, functions, classes)
9.  FFI and Native Functions (calling C code, type mapping)
10. Error Handling (exceptions, try-catch, propagation)
11. Concurrency Model (async/await, threads, mutexes)
12. Standard Patterns (OOP, functional, modules, error handling)
13. Quick Reference (keywords, operators, types)
14. Implementation Notes for LLMs (architecture, patterns, limitations)
15. Example Programs (hello world, fibonacci, classes, async, files)
```

Each section is designed for LLM comprehension with:
- Clear hierarchical structure
- Tables and diagrams where helpful
- Concrete code examples
- Cross-references to related sections

---

## 🔧 Use Cases

### 1. **Code Generation**
```
Input: "Write a function that reads a CSV file in Dryad"
Uses: Sections 6 (Declarations), 7 (Modules), 9 (FFI)
Output: Complete, runnable function
```

### 2. **Code Analysis**
```
Input: User's Dryad code + semantic issues
Uses: Sections 3 (Types), 5 (Statements), 8 (Scoping)
Output: Issue explanation + corrected code
```

### 3. **Debugging**
```
Input: Runtime error + code that caused it
Uses: Section 10 (Error Handling), full context
Output: Root cause + fix
```

### 4. **Language Feature Explanation**
```
Input: "Explain async/await in Dryad"
Uses: Section 11 (Concurrency), examples
Output: Complete explanation with examples
```

### 5. **Code Translation**
```
Input: JavaScript code + "translate to Dryad"
Uses: Entire specification as translation guide
Output: Idiomatic Dryad equivalent
```

---

## 📖 Example: Using with Claude

```python
from anthropic import Anthropic

# Load specification
with open("DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md") as f:
    SPEC = f.read()

client = Anthropic()

# System prompt with specification
system_prompt = f"""You are an expert Dryad programmer with complete knowledge of the language:

{SPEC}

When writing Dryad code, always follow the specification exactly.
"""

# Use it
response = client.messages.create(
    model="claude-3-5-sonnet-20241022",
    max_tokens=2048,
    system=system_prompt,
    messages=[
        {
            "role": "user",
            "content": "Write a Dryad function that fetches JSON from HTTP and parses it"
        }
    ]
)

print(response.content[0].text)
```

---

## 🛠 MCP Server Usage

### Setup

```bash
# Install dependencies
pip install mcp anthropic

# Verify Dryad CLI is installed
dryad --version

# Run server
python3 dryad_mcp_server.py
```

### Available Resources (MCP URIs)

- `dryad://specification/complete` - Full specification
- `dryad://specification/lexical` - Token types, keywords, operators
- `dryad://specification/types` - Type system
- `dryad://specification/expressions` - Operators and precedence
- `dryad://specification/statements` - Control flow
- `dryad://specification/declarations` - Functions, classes
- `dryad://specification/modules` - Import/export
- `dryad://specification/runtime` - Execution model
- `dryad://specification/ffi` - C FFI
- `dryad://specification/errors` - Error handling
- `dryad://specification/concurrency` - Async/threads
- `dryad://specification/patterns` - Design patterns
- `dryad://specification/examples` - Example programs
- `dryad://quick-reference` - One-page reference

### Available Tools

```json
{
  "validate_dryad": {
    "description": "Validate Dryad code syntax",
    "input": {"code": "string"}
  },
  "analyze_dryad": {
    "description": "Analyze code for semantic issues",
    "input": {"code": "string", "analysis_type": "enum[types,scopes,functions,all]"}
  },
  "lookup_syntax": {
    "description": "Look up syntax patterns",
    "input": {"query": "string"}
  },
  "get_keywords": {
    "description": "Get list of Dryad keywords",
    "input": {}
  },
  "quick_reference": {
    "description": "Get quick reference",
    "input": {}
  }
}
```

---

## 📋 Requirements

### For Direct Use (No MCP)
- Dryad language installed (optional, for code execution)
- Access to specification file

### For MCP Server
- Python 3.8+
- `mcp` package: `pip install mcp`
- Dryad CLI (optional): `dryad` in PATH
- Specification file: `DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md`

### For LLM Integration
- LLM client (Claude, ChatGPT, etc.)
- MCP-compatible tool interface
- Network access to MCP server (if remote)

---

## 🔗 Integration Examples

### Claude via MCP

```bash
# Start MCP server
python3 dryad_mcp_server.py &

# Connect Claude to server
# (Via Claude Desktop configuration or API)
```

### ChatGPT Custom Action

```json
{
  "name": "dryad_helper",
  "description": "Help with Dryad programming",
  "spec": {
    "url": "http://localhost:3000/openapi.json",
    "auth": { "type": "none" }
  }
}
```

### Local Development

```python
# Direct specification use
spec = open("DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md").read()

# In your prompt templates
prompt = f"Reference this specification:\n{spec}\n\nNow help with: {user_request}"
```

---

## ✅ Quality Assurance

**Specification Coverage:**
- ✅ 100% of core language features documented
- ✅ All operators with precedence table
- ✅ All statement types with examples
- ✅ Complete type system
- ✅ Module system documented
- ✅ FFI interface specifications
- ✅ Error handling patterns
- ✅ Concurrency model

**Code Examples:**
- ✅ Hello world
- ✅ Fibonacci (recursion)
- ✅ Class hierarchy (OOP)
- ✅ Async processing
- ✅ File I/O with native modules

**MCP Integration:**
- ✅ Resource URIs structured
- ✅ Tools defined with schemas
- ✅ Server implementation complete
- ✅ Error handling in place
- ✅ Caching for performance

---

## 📝 Maintenance

### When to Update

1. **Language Feature Added**
   - Add to appropriate specification section
   - Include examples
   - Update quick reference
   - Regenerate section index

2. **Syntax Changed**
   - Update section 2 (Lexical)
   - Update affected statement/expression sections
   - Update examples

3. **New Native Module**
   - Update section 9 (FFI)
   - Add module to native functions table

4. **Runtime Behavior Change**
   - Update section 8 (Execution Model)
   - Update error handling if affected
   - Update examples

### Version Control

```markdown
## Document History

### Version 1.0 (2026-03-09)
- Initial comprehensive specification
- All core language features
- MCP integration guide
- Ready-to-use server
- 1400+ line specification
```

---

## 📞 Support

### For Questions About Language

Reference sections in `DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md`:
- Syntax: Section 2-6
- Types: Section 3
- Runtime: Section 8
- Modules: Section 7
- Errors: Section 10

### For Questions About MCP Integration

Reference `DRYAD_MCP_INTEGRATION_GUIDE.md`:
- Implementation: Section 1-2
- Use cases: Section 4
- Best practices: Section 5
- Examples: Section 6

### For Server Issues

Check `dryad_mcp_server.py`:
- Logs show specification loading
- Environment variables: `DRYAD_CLI_PATH`, `DRYAD_SPEC_PATH`
- Tools require valid Dryad CLI for validation

---

## 📄 License

These documentation and tools are part of the Dryad Language project.
Same license as the main Dryad repository.

---

## 🎯 Next Steps

1. **Choose Integration Strategy**
   - Direct specification use (simplest)
   - MCP server deployment (most flexible)
   - Custom implementation (most control)

2. **Deploy**
   - Copy specification to your project
   - Run `dryad_mcp_server.py` if using MCP
   - Configure LLM system prompt

3. **Test**
   - Try code generation on simple tasks
   - Verify generated code is valid
   - Refine prompts based on results

4. **Monitor**
   - Track which features LLM uses correctly
   - Note patterns that need improvement
   - Contribute improvements back

---

**Last Updated:** 2026-03-09  
**Status:** Production Ready  
**Maintainer:** Sisyphus (AI Agent)  
**Repository:** https://github.com/dryad-lang/dryad

---

## Files in This Suite

```
├── DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md     (1400+ lines, complete spec)
├── DRYAD_MCP_INTEGRATION_GUIDE.md               (700+ lines, integration guide)
├── dryad_mcp_server.py                          (500+ lines, ready-to-run server)
└── README_MCP_SUITE.md                          (this file)
```

**Start with:** Read this file → Choose integration → Use specification/server

