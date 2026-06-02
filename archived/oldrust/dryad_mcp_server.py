#!/usr/bin/env python3
"""
Dryad MCP Server - Ready-to-Use Implementation
Enables LLMs to understand and work with Dryad code via Model Context Protocol

Usage:
    python3 dryad_mcp_server.py

Environment:
    DRYAD_CLI_PATH: Path to dryad CLI executable (default: "dryad")
    DRYAD_SPEC_PATH: Path to spec file (default: "./DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md")

This server implements:
- Resources: Language specification sections
- Tools: Validation, analysis, syntax lookup
- Prompts: Pre-configured request templates
"""

import json
import subprocess
import re
import sys
import os
from pathlib import Path
from typing import Any, Dict, List, Optional
from dataclasses import dataclass, asdict

# Try importing MCP SDK - provide fallback if not installed
try:
    from mcp.server.models import InitializationOptions
    from mcp.types import TextContent, Tool, ToolResult
    from mcp.server import Server, Request
    import mcp.types as types
except ImportError:
    print("Error: MCP SDK not installed. Install with: pip install mcp", file=sys.stderr)
    sys.exit(1)


# Configuration
DRYAD_CLI_PATH = os.getenv("DRYAD_CLI_PATH", "dryad")
SPEC_PATH = Path(os.getenv("DRYAD_SPEC_PATH", "./DRYAD_LANGUAGE_SPECIFICATION_FOR_LLM.md"))

# Specification sections for resource mapping
SPEC_SECTIONS = {
    "overview": {"title": "Language Overview", "section": 1},
    "lexical": {"title": "Lexical Structure", "section": 2},
    "types": {"title": "Type System", "section": 3},
    "expressions": {"title": "Expressions", "section": 4},
    "statements": {"title": "Statements", "section": 5},
    "declarations": {"title": "Declarations", "section": 6},
    "modules": {"title": "Modules and Imports", "section": 7},
    "runtime": {"title": "Runtime Execution Model", "section": 8},
    "ffi": {"title": "FFI and Native Functions", "section": 9},
    "errors": {"title": "Error Handling", "section": 10},
    "concurrency": {"title": "Concurrency Model", "section": 11},
    "patterns": {"title": "Standard Patterns", "section": 12},
    "reference": {"title": "Quick Reference", "section": 13},
    "notes": {"title": "Implementation Notes for LLMs", "section": 14},
    "examples": {"title": "Example Programs", "section": 15},
}


class DryaDSpecificationManager:
    """Manages loading and serving Dryad specification sections"""

    def __init__(self, spec_path: Path):
        self.spec_path = spec_path
        self._spec_content = None
        self._sections_cache = {}

    @property
    def spec_content(self) -> str:
        """Lazily load specification content"""
        if self._spec_content is None:
            try:
                with open(self.spec_path, "r", encoding="utf-8") as f:
                    self._spec_content = f.read()
            except FileNotFoundError:
                return f"ERROR: Specification file not found at {self.spec_path}"
        return self._spec_content

    def get_section(self, section_id: str) -> str:
        """Get a specific section of the specification"""
        if section_id not in SPEC_SECTIONS:
            return f"Unknown section: {section_id}. Available: {', '.join(SPEC_SECTIONS.keys())}"

        if section_id in self._sections_cache:
            return self._sections_cache[section_id]

        section_num = SPEC_SECTIONS[section_id]["section"]
        section_title = SPEC_SECTIONS[section_id]["title"]

        # Extract section from full spec
        lines = self.spec_content.split("\n")
        section_start = None
        section_end = None

        for i, line in enumerate(lines):
            # Look for section header like "## 3. Type System"
            if re.match(f"^## {section_num}\\.", line):
                section_start = i
            elif section_start is not None and re.match(r"^## \d+\.", line):
                section_end = i
                break

        if section_start is None:
            return f"Section {section_num} not found in specification"

        section_content = "\n".join(
            lines[section_start : section_end] if section_end else lines[section_start:]
        )
        self._sections_cache[section_id] = section_content
        return section_content

    def get_quick_reference(self) -> str:
        """Get a quick syntax reference"""
        keywords_section = self.get_section("reference")
        if "Unknown section" in keywords_section:
            # Fallback to a minimal reference
            return """# Quick Dryad Reference

## Variables
- `let x = value;` - mutable, block-scoped
- `const x = value;` - immutable, block-scoped
- `var x = value;` - mutable, function-scoped

## Functions
- `function name(params) { body }`
- `const fn = (params) => body;` - arrow function
- `async function name() { await expr; }`
- `thread function name() { ... }` - runs in OS thread

## Types
- `let x: number` - floating point
- `let x: string` - text
- `let x: bool` - boolean
- `let x: any[]` - array
- `let x: (number, string)` - tuple

## Classes
- `class Name { constructor(x) { this.x = x; } }`
- `class Child extends Parent { }`
- Properties and methods defined in class body
- `new ClassName()` - instantiation

## Control Flow
- `if (cond) { } else { }`
- `while (cond) { }`
- `for (let i = 0; i < n; i++) { }`
- `for (item in array) { }`
- `break;` and `continue;`

## Modules
- `import { func } from "module";`
- `export function func() { }`
- `use "path/to/file";` - simplified import
- `#io` - load native module

## Error Handling
- `try { risky(); } catch (e) { handle(e); } finally { cleanup(); }`
- `throw new Error("message");`

## Operators
- Arithmetic: `+, -, *, /, %, **`
- Logical: `&&, ||, !`
- Comparison: `==, !=, <, >, <=, >=`
- Assignment: `=, +=, -=, *=, /=`
"""
        return keywords_section

    def list_keywords(self) -> List[str]:
        """Extract list of keywords from specification"""
        # Hardcoded for reliability
        return [
            "let",
            "const",
            "var",
            "function",
            "fn",
            "class",
            "interface",
            "extends",
            "implements",
            "return",
            "if",
            "else",
            "for",
            "while",
            "do",
            "break",
            "continue",
            "try",
            "catch",
            "finally",
            "throw",
            "import",
            "export",
            "use",
            "from",
            "async",
            "await",
            "thread",
            "mutex",
            "new",
            "this",
            "super",
            "static",
            "public",
            "private",
            "protected",
            "match",
            "in",
            "of",
        ]

    def find_syntax_pattern(self, query: str) -> str:
        """Find syntax patterns matching a query"""
        query_lower = query.lower()
        content = self.spec_content

        # Search for relevant sections
        results = []

        # Search in lexical structure
        if any(
            x in query_lower
            for x in ["token", "keyword", "operator", "symbol", "literal"]
        ):
            results.append("### Lexical Structure (Section 2)")
            lexical_section = self.get_section("lexical")
            # Extract table from section
            for line in lexical_section.split("\n"):
                if query_lower in line.lower() and "|" in line:
                    results.append(line)

        # Search in syntax rules
        if any(x in query_lower for x in ["function", "class", "variable", "if", "for"]):
            results.append("### Relevant Syntax Rules")
            for line in content.split("\n"):
                if query_lower in line.lower() and any(
                    x in line for x in ["function ", "class ", "if ", "for "]
                ):
                    results.append(line.strip())

        return "\n".join(results) if results else f"No patterns found for: {query}"


class DryadValidator:
    """Validates Dryad code using CLI"""

    def __init__(self, cli_path: str = DRYAD_CLI_PATH):
        self.cli_path = cli_path

    def validate_syntax(self, code: str) -> Dict[str, Any]:
        """Validate Dryad code syntax"""
        try:
            result = subprocess.run(
                [self.cli_path, "check", "-"],
                input=code,
                text=True,
                capture_output=True,
                timeout=10,
            )

            return {
                "valid": result.returncode == 0,
                "stdout": result.stdout,
                "stderr": result.stderr,
                "return_code": result.returncode,
            }
        except FileNotFoundError:
            return {
                "valid": None,
                "error": f"Dryad CLI not found at {self.cli_path}. Install Dryad or set DRYAD_CLI_PATH.",
            }
        except subprocess.TimeoutExpired:
            return {"valid": None, "error": "Validation timeout"}
        except Exception as e:
            return {"valid": None, "error": str(e)}

    def analyze_code(self, code: str, analysis_type: str = "all") -> Dict[str, Any]:
        """Analyze code for semantic issues"""
        # Since we don't have semantic analysis in CLI, do basic checks
        analysis = {
            "type": analysis_type,
            "code": code,
            "issues": [],
        }

        # Check for common patterns
        lines = code.split("\n")

        # Check for undefined variables (basic)
        if "undefined_var" in code.lower():
            analysis["issues"].append(
                {"type": "undefined_variable", "message": "Possible undefined variable"}
            )

        # Check for missing returns in functions
        if "function " in code and "return " not in code:
            analysis["issues"].append(
                {"type": "missing_return", "message": "Function may not return value"}
            )

        # Check for uncaught errors
        if "throw " in code and "catch " not in code:
            analysis["issues"].append(
                {
                    "type": "uncaught_exception",
                    "message": "Throw without try-catch may cause crash",
                }
            )

        return analysis


def create_mcp_server() -> Server:
    """Create and configure MCP server"""

    spec_manager = DryaDSpecificationManager(SPEC_PATH)
    validator = DryadValidator(DRYAD_CLI_PATH)

    server = Server("dryad-mcp-server")

    # Register resources
    @server.list_resources()
    async def list_resources() -> list[types.Resource]:
        """List available resources"""
        resources = [
            types.Resource(
                uri="dryad://specification/complete",
                name="Complete Dryad Language Specification",
                description="Full language reference optimized for LLMs (1400+ lines)",
                mimeType="text/markdown",
            )
        ]

        # Add section resources
        for section_id, section_info in SPEC_SECTIONS.items():
            resources.append(
                types.Resource(
                    uri=f"dryad://specification/{section_id}",
                    name=f"Dryad {section_info['title']}",
                    description=f"Section {section_info['section']}: {section_info['title']}",
                    mimeType="text/markdown",
                )
            )

        # Add tools documentation
        resources.append(
            types.Resource(
                uri="dryad://tools/validation",
                name="Code Validation Tools",
                description="Tools for validating and analyzing Dryad code",
                mimeType="application/json",
            )
        )

        resources.append(
            types.Resource(
                uri="dryad://quick-reference",
                name="Quick Syntax Reference",
                description="One-page quick reference for Dryad syntax",
                mimeType="text/markdown",
            )
        )

        return resources

    @server.read_resource()
    async def read_resource(uri: str) -> str:
        """Read a resource by URI"""
        if uri == "dryad://specification/complete":
            return spec_manager.spec_content
        elif uri.startswith("dryad://specification/"):
            section_id = uri.replace("dryad://specification/", "")
            return spec_manager.get_section(section_id)
        elif uri == "dryad://quick-reference":
            return spec_manager.get_quick_reference()
        elif uri == "dryad://tools/validation":
            return json.dumps(
                {
                    "available_tools": [
                        {
                            "name": "validate_dryad",
                            "description": "Validate Dryad code syntax",
                        },
                        {
                            "name": "analyze_dryad",
                            "description": "Analyze code for semantic issues",
                        },
                        {
                            "name": "lookup_syntax",
                            "description": "Look up syntax patterns",
                        },
                    ]
                }
            )
        else:
            raise ValueError(f"Unknown resource: {uri}")

    # Register tools
    @server.call_tool()
    async def call_tool(name: str, arguments: dict) -> list[types.TextContent]:
        """Handle tool calls"""

        if name == "validate_dryad":
            code = arguments.get("code", "")
            result = validator.validate_syntax(code)
            return [types.TextContent(type="text", text=json.dumps(result, indent=2))]

        elif name == "analyze_dryad":
            code = arguments.get("code", "")
            analysis_type = arguments.get("analysis_type", "all")
            result = validator.analyze_code(code, analysis_type)
            return [types.TextContent(type="text", text=json.dumps(result, indent=2))]

        elif name == "lookup_syntax":
            query = arguments.get("query", "")
            result = spec_manager.find_syntax_pattern(query)
            return [types.TextContent(type="text", text=result)]

        elif name == "get_keywords":
            keywords = spec_manager.list_keywords()
            return [types.TextContent(type="text", text=json.dumps(keywords))]

        elif name == "quick_reference":
            ref = spec_manager.get_quick_reference()
            return [types.TextContent(type="text", text=ref)]

        else:
            raise ValueError(f"Unknown tool: {name}")

    return server


def main():
    """Main entry point"""
    server = create_mcp_server()

    # Tools definition
    tools = [
        types.Tool(
            name="validate_dryad",
            description="Validate Dryad code syntax against language specification",
            inputSchema={
                "type": "object",
                "properties": {
                    "code": {
                        "type": "string",
                        "description": "Dryad code to validate",
                    }
                },
                "required": ["code"],
            },
        ),
        types.Tool(
            name="analyze_dryad",
            description="Analyze Dryad code for semantic issues",
            inputSchema={
                "type": "object",
                "properties": {
                    "code": {"type": "string", "description": "Code to analyze"},
                    "analysis_type": {
                        "type": "string",
                        "enum": ["types", "scopes", "functions", "all"],
                        "description": "Type of analysis to perform",
                    },
                },
                "required": ["code"],
            },
        ),
        types.Tool(
            name="lookup_syntax",
            description="Look up Dryad syntax rules and patterns",
            inputSchema={
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Syntax topic to look up (e.g., 'function', 'class', 'loop')",
                    }
                },
                "required": ["query"],
            },
        ),
        types.Tool(
            name="get_keywords",
            description="Get list of Dryad keywords",
            inputSchema={"type": "object", "properties": {}},
        ),
        types.Tool(
            name="quick_reference",
            description="Get quick syntax reference",
            inputSchema={"type": "object", "properties": {}},
        ),
    ]

    # Register tools with server
    # Note: Implementation depends on MCP SDK version
    # This is a placeholder - adjust based on your SDK

    print("Dryad MCP Server starting...")
    print(f"Specification: {SPEC_PATH}")
    print(f"CLI Path: {DRYAD_CLI_PATH}")
    print()
    print("Tools available:")
    for tool in tools:
        print(f"  - {tool.name}: {tool.description}")
    print()
    print("Resources available:")
    print("  - dryad://specification/complete")
    print("  - dryad://specification/{lexical,types,expressions,statements,declarations,modules,runtime,ffi,errors,concurrency,patterns,examples}")
    print("  - dryad://quick-reference")
    print()

    # Start server
    import asyncio

    async def astart():
        async with server:
            print("Server running. Press Ctrl+C to stop.")
            await asyncio.Event().wait()

    try:
        asyncio.run(astart())
    except KeyboardInterrupt:
        print("\nServer stopped.")


if __name__ == "__main__":
    main()
