# Dryad MCP (Model Context Protocol) Integration Guide

**Purpose:** Enable LLMs and AI assistants to understand and work with Dryad code through structured context injection.

**Document Version:** 1.0  
**Target:** LLM Developers, MCP Tool Builders, AI Assistance Tool Creators

---

## Overview

This guide explains how to use `DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md` as a Model Context Protocol (MCP) resource to enable LLMs to:

1. **Generate** valid Dryad code
2. **Understand** Dryad code semantics
3. **Refactor** and optimize Dryad programs
4. **Debug** runtime errors
5. **Suggest** language features and idioms
6. **Translate** code from other languages to Dryad

---

## What is Model Context Protocol (MCP)?

MCP is a protocol for LLMs to receive structured context. Instead of unstructured instructions, MCP provides:
- **Documents** (specifications, manuals, guides)
- **Tools** (code analysis, generation, execution)
- **Resources** (code examples, templates, reference materials)
- **Prompts** (pre-configured request templates)

The Dryad specification can be served as an MCP **Document** or **Resource**.

---

## MCP Server Implementation (Example)

### 1. Serving as Document Resource

```typescript
// dryad-mcp-server.ts - Example MCP Server in TypeScript

import { MCPServer } from "@modelcontextprotocol/sdk/server/index.js";

const server = new MCPServer({
  name: "dryad-language-server",
  version: "1.0.0",
});

// Register the Dryad specification as a resource
server.resource.add({
  uri: "dryad://specification/language-reference",
  name: "Dryad Language Specification for LLMs",
  description: "Complete Dryad language syntax, semantics, type system, and execution model",
  mimeType: "text/markdown",
  async readContent() {
    return await fs.readFile("DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md", "utf-8");
  },
});

// Register sections as separate resources for targeted access
server.resource.add({
  uri: "dryad://specification/type-system",
  name: "Dryad Type System",
  mimeType: "text/markdown",
  async readContent() {
    // Extract Section 3 from specification
    return extractSection("Type System");
  },
});

server.resource.add({
  uri: "dryad://specification/operators",
  name: "Dryad Operators and Expressions",
  mimeType: "text/markdown",
  async readContent() {
    return extractSection("Expressions and Operators");
  },
});

// Tool: Validate Dryad Code
server.tool.add({
  name: "validate_dryad",
  description: "Check if code is valid Dryad syntax",
  inputSchema: {
    type: "object",
    properties: {
      code: { type: "string", description: "Dryad code to validate" },
    },
    required: ["code"],
  },
  async execute(args: any) {
    const { code } = args;
    // Call dryad parser
    const result = await parseDryad(code);
    return {
      valid: result.ok,
      errors: result.errors || [],
      ast: result.ast || null,
    };
  },
});

// Tool: Analyze Dryad Code
server.tool.add({
  name: "analyze_dryad",
  description: "Analyze Dryad code for semantic issues",
  inputSchema: {
    type: "object",
    properties: {
      code: { type: "string" },
      analysis: {
        type: "array",
        enum: ["types", "scopes", "functions", "classes", "errors"],
      },
    },
  },
  async execute(args: any) {
    // Semantic analysis
    return performAnalysis(args.code, args.analysis);
  },
});

server.connect(process.stdin, process.stdout);
```

### 2. System Prompt for LLM (When Using MCP)

```
You have access to comprehensive Dryad language documentation via MCP.

When working with Dryad code:
1. First check: dryad://specification/language-reference for the complete specification
2. For type questions: check dryad://specification/type-system
3. For expression help: check dryad://specification/operators
4. Use validate_dryad tool to check syntax
5. Use analyze_dryad tool for semantic issues

Key things to remember:
- Dryad is dynamically typed but supports optional type annotations
- It's a tree-walking interpreter (not bytecode VM)
- Uses JavaScript-like syntax but with class-based OOP
- Supports async/await, threads, and mutexes
- Native modules available via # directive (e.g., #io, #crypto)
- Scope chain: block-scoped for let/const
```

---

## Integration Strategies

### Strategy 1: Specification as System Context

**When to use:** Building a general-purpose Dryad assistant

```markdown
**System Prompt:**

You are an expert Dryad programmer. You have complete knowledge of Dryad syntax, types, and semantics:

[INSERT ENTIRE SPECIFICATION HERE]

When generating Dryad code:
1. Follow the exact syntax rules specified above
2. Ensure type annotations are valid
3. Use appropriate error handling patterns
4. Follow the module system for imports/exports
5. Use native modules (#io, #crypto, etc.) when appropriate

When analyzing Dryad code:
1. Check against the grammar rules
2. Verify type compatibility
3. Ensure scoping rules are followed
4. Identify common patterns and idioms
```

### Strategy 2: Specification as Reference Tool

**When to use:** Implementing an MCP server with modular access

```python
# dryad_mcp_tools.py - Python MCP Tools

class DryaDSpecification:
    def __init__(self):
        self.spec = load_specification()
    
    def get_section(self, section_name):
        """Get a specific section of the specification"""
        return self.spec.extract_section(section_name)
    
    def find_syntax_rule(self, keyword):
        """Find syntax rules for a keyword or operator"""
        # Search specification for rules matching keyword
        pass
    
    def get_type_rules(self, type_name):
        """Get type compatibility and coercion rules"""
        pass
    
    def get_examples(self, feature):
        """Get code examples for a feature"""
        pass

# MCP Tool registration
mcp_tools = {
    "lookup_syntax": {
        "description": "Look up Dryad syntax rules",
        "function": lambda q: dryad_spec.find_syntax_rule(q),
    },
    "check_type": {
        "description": "Check type compatibility",
        "function": lambda t1, t2: dryad_spec.check_type_compatibility(t1, t2),
    },
    "get_examples": {
        "description": "Get code examples",
        "function": lambda f: dryad_spec.get_examples(f),
    },
}
```

### Strategy 3: Specification as Training Data

**When to use:** Fine-tuning LLMs on Dryad

1. Convert specification to structured JSON format
2. Create training examples from specification sections
3. Include code examples and counter-examples
4. Mark error cases and their explanations
5. Train on both specification understanding and code generation

```json
{
  "training_examples": [
    {
      "specification_section": "Type System - Primitive Types",
      "example_valid": "let x: number = 42;",
      "example_invalid": "let x: integer = 42;",
      "explanation": "Dryad uses 'number', not 'integer'",
      "category": "type_annotation"
    },
    {
      "specification_section": "Statements - Variable Declaration",
      "example_valid": "let x = 1; let y = 2;",
      "example_invalid": "let x, y = 1, 2;",
      "explanation": "Multiple declarations must have separate let keywords or use destructuring",
      "category": "syntax"
    }
  ]
}
```

---

## Specific Use Cases

### Use Case 1: Code Generation

**Prompt Template:**
```
Task: Generate Dryad code for [task description]

Requirements:
- Must follow Dryad syntax exactly (see specification)
- Use appropriate type annotations
- Handle errors with try-catch
- Use native modules if needed: [#modules to use]
- Include comments explaining logic

Reference: dryad://specification/language-reference

Generated code:
```

**Example:**
```
Task: Create a function that reads a CSV file, parses it, and returns array of objects

Generated code:
```

### Use Case 2: Code Analysis and Suggestions

**Prompt Template:**
```
Analyze this Dryad code for issues:

[CODE]

Check for:
1. Type errors (per Dryad type system)
2. Scoping issues (let/const/var rules)
3. Missing error handling
4. Performance concerns
5. Non-idiomatic patterns

Reference: Entire Dryad specification
```

### Use Case 3: Debugging and Error Explanation

**Prompt Template:**
```
User got this Dryad runtime error:

[ERROR MESSAGE AND CODE]

Explain:
1. What caused the error
2. Why it happens
3. How to fix it
4. Show corrected code

Reference: Error Handling section of specification
```

### Use Case 4: Language Feature Explanation

**Prompt Template:**
```
Explain the Dryad feature: [FEATURE]

Include:
1. Syntax rules
2. Example usage
3. Common patterns
4. Common mistakes
5. Performance notes

Reference: dryad://specification/[relevant-section]
```

### Use Case 5: Code Translation

**Prompt Template:**
```
Translate this [SOURCE_LANGUAGE] code to Dryad:

[SOURCE CODE]

Guidelines:
1. Follow Dryad syntax and idioms
2. Use equivalent Dryad features
3. Include appropriate error handling
4. Add type annotations where helpful
5. Explain any feature differences

Reference: Full Dryad specification
```

---

## Document Sections for Different Tasks

| Task | Relevant Sections | MCP URI |
|------|-------------------|---------|
| **Generate Code** | Lexical Structure, Expressions, Statements, Declarations | `dryad://spec/syntax` |
| **Type Checking** | Type System (3.1-3.4) | `dryad://spec/types` |
| **Function Design** | Declarations (6.1), Execution Model (8.3) | `dryad://spec/functions` |
| **Error Handling** | Error Handling (10.1-10.4), Try-Catch (5.9) | `dryad://spec/errors` |
| **Module System** | Modules and Imports (7.1-7.3) | `dryad://spec/modules` |
| **OOP Design** | Declarations (6.2-6.3), Execution Model (8.4) | `dryad://spec/oop` |
| **Performance** | Execution Model (8), Implementation Notes (14.3-14.4) | `dryad://spec/performance` |
| **Concurrency** | Concurrency Model (11.1-11.3) | `dryad://spec/concurrency` |
| **Native Integration** | FFI (9.1-9.4) | `dryad://spec/ffi` |

---

## LLM Prompt Best Practices

### 1. Include Relevant Section Only

**Don't:** Ask LLM to remember entire 1400+ line specification
**Do:** Include only relevant sections for the task

```markdown
For this task, here's the relevant spec:

## Type System (Relevant for your task)
[Include only 3-5 key paragraphs]

## Expressions (Relevant for your task)
[Include precedence table]
```

### 2. Use Structured Requests

```markdown
Task: Generate Dryad code

Input: [specific requirements]

Constraints:
- Must use only features from spec section 5 (Statements)
- Type annotations required (per section 3)
- No undefined behavior allowed

Reference: [link to specification section]

Please generate:
```

### 3. Verify Against Specification

**When LLM generates code:**
1. Check syntax against lexical structure (section 2)
2. Check types against type system (section 3)
3. Check patterns against standard patterns (section 12)
4. Check for unsupported features (section 14.3)

### 4. Iterative Refinement

```
Round 1: Generate initial code
Round 2: Check against spec - find issues
Round 3: Ask LLM to fix specific issues per spec
Round 4: Validate against specification rules
```

---

## MCP Resources JSON Structure

```json
{
  "mcp_server": {
    "name": "dryad-assistant",
    "version": "1.0",
    "resources": [
      {
        "uri": "dryad://specification/complete",
        "title": "Complete Dryad Language Specification",
        "description": "Full language reference optimized for LLMs",
        "mimeType": "text/markdown",
        "updateInterval": "PT24H"
      },
      {
        "uri": "dryad://specification/lexical",
        "title": "Lexical Structure",
        "section": 2,
        "topics": ["tokens", "keywords", "operators", "comments"]
      },
      {
        "uri": "dryad://specification/types",
        "title": "Type System",
        "section": 3,
        "topics": ["primitives", "composites", "annotations", "compatibility"]
      },
      {
        "uri": "dryad://specification/syntax",
        "title": "Expressions and Statements",
        "sections": [4, 5],
        "topics": ["operators", "precedence", "statements", "control-flow"]
      },
      {
        "uri": "dryad://specification/declarations",
        "title": "Functions, Classes, Interfaces",
        "section": 6,
        "topics": ["functions", "classes", "interfaces", "methods"]
      },
      {
        "uri": "dryad://specification/modules",
        "title": "Module System",
        "section": 7,
        "topics": ["imports", "exports", "native-directives"]
      },
      {
        "uri": "dryad://specification/runtime",
        "title": "Execution Model",
        "section": 8,
        "topics": ["execution-flow", "scoping", "memory", "functions"]
      },
      {
        "uri": "dryad://specification/ffi",
        "title": "FFI and Native Functions",
        "section": 9,
        "topics": ["foreign-functions", "type-mapping", "modules"]
      },
      {
        "uri": "dryad://specification/errors",
        "title": "Error Handling",
        "section": 10,
        "topics": ["error-types", "propagation", "try-catch", "exceptions"]
      },
      {
        "uri": "dryad://specification/patterns",
        "title": "Standard Patterns",
        "section": 12,
        "topics": ["oop", "functional", "modules", "error-handling"]
      },
      {
        "uri": "dryad://specification/reference",
        "title": "Quick Reference",
        "section": 13,
        "topics": ["keywords", "operators", "types"]
      },
      {
        "uri": "dryad://specification/examples",
        "title": "Example Programs",
        "section": 15,
        "topics": ["hello-world", "fibonacci", "classes", "async", "files"]
      }
    ],
    "tools": [
      {
        "name": "validate_dryad",
        "description": "Validate Dryad code syntax",
        "input": "code: string"
      },
      {
        "name": "analyze_dryad",
        "description": "Analyze code for semantic issues",
        "input": "code: string, analysis_type: string"
      },
      {
        "name": "lookup_syntax",
        "description": "Look up syntax rules",
        "input": "query: string"
      },
      {
        "name": "check_type_compatibility",
        "description": "Check if types are compatible",
        "input": "type1: string, type2: string"
      }
    ]
  }
}
```

---

## Deployment Checklist

- [ ] Deploy `DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md` to static hosting
- [ ] Set up MCP server with specification resource
- [ ] Implement validation tools (syntax, semantic analysis)
- [ ] Create LLM system prompts that reference specification
- [ ] Add tool definitions for code analysis
- [ ] Test with sample code generation tasks
- [ ] Document MCP endpoints for LLM integration
- [ ] Set up monitoring for tool usage
- [ ] Create error feedback loop to improve specification

---

## Example: Claude Integration

```python
# dryad_claude_assistant.py

from anthropic import Anthropic
import subprocess
import json

client = Anthropic()
conversation_history = []

# Load Dryad specification
with open("DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md", "r") as f:
    DRYAD_SPEC = f.read()

SYSTEM_PROMPT = f"""You are an expert Dryad programmer.

You have complete knowledge of the Dryad programming language:

{DRYAD_SPEC}

When writing Dryad code:
1. Follow the exact syntax and semantics specified
2. Use appropriate type annotations
3. Include proper error handling
4. Use idiomatic Dryad patterns
5. Reference the specification when needed

When analyzing Dryad code:
1. Check syntax against the lexical structure
2. Verify type compatibility
3. Ensure correct scoping
4. Identify pattern violations
5. Suggest improvements per specification
"""

def generate_dryad_code(user_request):
    """Generate Dryad code based on user request"""
    conversation_history.append({
        "role": "user",
        "content": user_request
    })
    
    response = client.messages.create(
        model="claude-3-5-sonnet-20241022",
        max_tokens=2048,
        system=SYSTEM_PROMPT,
        messages=conversation_history
    )
    
    assistant_message = response.content[0].text
    conversation_history.append({
        "role": "assistant",
        "content": assistant_message
    })
    
    return assistant_message

def validate_generated_code(code):
    """Validate generated code using dryad CLI"""
    try:
        result = subprocess.run(
            ["dryad", "check", "-"],
            input=code,
            text=True,
            capture_output=True,
            timeout=5
        )
        return {
            "valid": result.returncode == 0,
            "output": result.stdout,
            "errors": result.stderr
        }
    except Exception as e:
        return {"valid": False, "errors": str(e)}

# Example usage
if __name__ == "__main__":
    # Generate code
    code = generate_dryad_code(
        "Write a function that reads a JSON file and returns parsed data"
    )
    print("Generated Code:")
    print(code)
    print()
    
    # Validate
    validation = validate_generated_code(code)
    print("Validation:")
    print(json.dumps(validation, indent=2))
```

---

## Document Maintenance

### When to Update Specification

1. **New language feature added** → Add to appropriate section + examples
2. **Syntax change** → Update section 2 (Lexical) and section 5 (Statements)
3. **Type system change** → Update section 3 (Type System)
4. **New native module** → Update section 9 (FFI)
5. **Runtime behavior change** → Update section 8 (Execution Model)

### Version Control

```markdown
# Document History

## Version 1.0 (2026-03-09)
- Initial comprehensive specification
- All core language features documented
- 1400+ lines of LLM-optimized content
- Ready for MCP integration

## Version 1.1 (TBD)
- [Future updates tracked here]
```

---

## Contact and Support

For questions about using this specification with LLMs:
- Check the relevant section of `DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md`
- Review example use cases in this guide
- Test MCP tool implementations
- File issues with specific LLM behavior

---

**Document Created:** 2026-03-09  
**Last Updated:** 2026-03-09  
**Maintenance:** Update whenever Dryad language changes  
**Audience:** LLM Developers, MCP Tool Builders, AI Assistants  
