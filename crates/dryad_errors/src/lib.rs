// crates/dryad_errors/src/lib.rs
use std::fmt;
use std::path::PathBuf;

pub mod error_catalog;
pub mod error_urls;
#[cfg(test)]
mod tests;

/// Source code location information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: Option<PathBuf>,
    pub line: usize,
    pub column: usize,
    pub position: usize,
    pub source_line: Option<String>, // Source code line for context
}

impl SourceLocation {
    pub fn new(file: Option<PathBuf>, line: usize, column: usize, position: usize) -> Self {
        Self {
            file,
            line,
            column,
            position,
            source_line: None,
        }
    }

    pub fn with_source_line(mut self, source_line: String) -> Self {
        self.source_line = Some(source_line);
        self
    }

    pub fn unknown() -> Self {
        Self {
            file: None,
            line: 0,
            column: 0,
            position: 0,
            source_line: None,
        }
    }
}

/// Stack trace frame
#[derive(Debug, Clone, PartialEq)]
pub struct StackFrame {
    pub function_name: String,
    pub location: SourceLocation,
    pub context: Option<String>, // Additional context (ex: "within class method", "in for loop")
}

impl StackFrame {
    pub fn new(function_name: String, location: SourceLocation) -> Self {
        Self {
            function_name,
            location,
            context: None,
        }
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

/// Complete stack trace
#[derive(Debug, Clone, PartialEq)]
pub struct StackTrace {
    pub frames: Vec<StackFrame>,
}

impl StackTrace {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn push_frame(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }

    pub fn from_frames(frames: Vec<StackFrame>) -> Self {
        Self { frames }
    }
}

/// Debug context information
#[derive(Debug, Clone, PartialEq)]
pub struct DebugContext {
    pub variables: Option<std::collections::HashMap<String, String>>, // Local variables
    pub suggestions: Vec<String>,                                     // Suggestions for fix
    pub help_url: Option<String>,                                     // Link to documentation
    pub related_code: Vec<String>,                                    // Code related to the error
}

impl DebugContext {
    pub fn new() -> Self {
        Self {
            variables: None,
            suggestions: Vec::new(),
            help_url: None,
            related_code: Vec::new(),
        }
    }

    pub fn with_variables(mut self, variables: std::collections::HashMap<String, String>) -> Self {
        self.variables = Some(variables);
        self
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    pub fn with_help_url(mut self, url: String) -> Self {
        self.help_url = Some(url);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WarningSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DryadError {
    Lexer {
        code: u16,
        message: String,
        location: SourceLocation,
        debug_context: Option<DebugContext>,
    },
    Parser {
        code: u16,
        message: String,
        location: SourceLocation,
        expected: Vec<String>, // Expected tokens
        found: String,         // Found token
        debug_context: Option<DebugContext>,
    },
    Runtime {
        code: u16,
        message: String,
        location: SourceLocation,
        stack_trace: StackTrace,
        debug_context: Option<DebugContext>,
    },
    Type {
        code: u16,
        message: String,
        location: SourceLocation,
        expected_type: String,
        found_type: String,
        debug_context: Option<DebugContext>,
    },
    Io {
        code: u16,
        message: String,
        location: SourceLocation,
        operation: String, // "read", "write", "open", etc.
        path: Option<PathBuf>,
        debug_context: Option<DebugContext>,
    },
    Module {
        code: u16,
        message: String,
        location: SourceLocation,
        module_name: String,
        debug_context: Option<DebugContext>,
    },
    Syntax {
        code: u16,
        message: String,
        location: SourceLocation,
        syntax_help: Option<String>,
        debug_context: Option<DebugContext>,
    },
    Warning {
        code: u16,
        message: String,
        location: SourceLocation,
        severity: WarningSeverity,
        debug_context: Option<DebugContext>,
    },
    System {
        code: u16,
        message: String,
        location: SourceLocation,
        system_info: Option<String>,
        debug_context: Option<DebugContext>,
    },
}

impl fmt::Display for DryadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DryadError::Lexer {
                code,
                message,
                location,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: Lexer Error - {}", code, message)?;
                write_location_info(f, location)?;
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::Parser {
                code,
                message,
                location,
                expected,
                found,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: Parser Error - {}", code, message)?;
                write_location_info(f, location)?;
                if !expected.is_empty() {
                    writeln!(f, "   📝 Expected: {}", expected.join(", "))?;
                }
                if !found.is_empty() {
                    writeln!(f, "   ❌ Found: {}", found)?;
                }
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::Runtime {
                code,
                message,
                location,
                stack_trace,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: Runtime Error - {}", code, message)?;
                write_location_info(f, location)?;
                write_stack_trace(f, stack_trace)?;
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::Type {
                code,
                message,
                location,
                expected_type,
                found_type,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: Type Error - {}", code, message)?;
                write_location_info(f, location)?;
                writeln!(f, "   📝 Expected type: {}", expected_type)?;
                writeln!(f, "   ❌ Found type: {}", found_type)?;
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::Io {
                code,
                message,
                location,
                operation,
                path,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: I/O Error - {}", code, message)?;
                write_location_info(f, location)?;
                writeln!(f, "   🔧 Operation: {}", operation)?;
                if let Some(path) = path {
                    writeln!(f, "   📁 File: {}", path.display())?;
                }
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::Module {
                code,
                message,
                location,
                module_name,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: Module Error - {}", code, message)?;
                write_location_info(f, location)?;
                writeln!(f, "   📦 Module: {}", module_name)?;
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::Syntax {
                code,
                message,
                location,
                syntax_help,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: Syntax Error - {}", code, message)?;
                write_location_info(f, location)?;
                if let Some(help) = syntax_help {
                    writeln!(f, "   💡 Hint: {}", help)?;
                }
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::Warning {
                code,
                message,
                location,
                severity,
                debug_context,
            } => {
                let icon = match severity {
                    WarningSeverity::Low => "⚠️",
                    WarningSeverity::Medium => "🟡",
                    WarningSeverity::High => "🟠",
                };
                writeln!(f, "\n{} W{}: Warning - {}", icon, code, message)?;
                write_location_info(f, location)?;
                write_debug_context(f, debug_context.as_ref())?;
            }
            DryadError::System {
                code,
                message,
                location,
                system_info,
                debug_context,
            } => {
                writeln!(f, "\n🚨 E{}: System Error - {}", code, message)?;
                write_location_info(f, location)?;
                if let Some(info) = system_info {
                    writeln!(f, "   🖥️  System: {}", info)?;
                }
                write_debug_context(f, debug_context.as_ref())?;
            }
        }
        Ok(())
    }
}

fn write_location_info(f: &mut fmt::Formatter<'_>, location: &SourceLocation) -> fmt::Result {
    if let Some(file) = &location.file {
        writeln!(
            f,
            "   📍 Location: {}:{}:{}",
            file.display(),
            location.line,
            location.column
        )?;
    } else {
        writeln!(
            f,
            "   📍 Location: line {}, column {}",
            location.line, location.column
        )?;
    }

    if let Some(source_line) = &location.source_line {
        writeln!(f, "   📄 Code:")?;
        writeln!(f, "      {}", source_line)?;

        // Show visual pointer to the error
        let pointer = format!("{:width$}^", "", width = location.column.saturating_sub(1));
        writeln!(f, "      {}", pointer)?;
    }

    Ok(())
}

fn write_stack_trace(f: &mut fmt::Formatter<'_>, stack_trace: &StackTrace) -> fmt::Result {
    if !stack_trace.frames.is_empty() {
        writeln!(f, "   📚 Stack Trace:")?;
        for (i, frame) in stack_trace.frames.iter().enumerate() {
            let prefix = if i == 0 { "   ┌─" } else { "   ├─" };
            write!(f, "{} {}", prefix, frame.function_name)?;

            if let Some(file) = &frame.location.file {
                write!(
                    f,
                    " ({}:{}:{})",
                    file.display(),
                    frame.location.line,
                    frame.location.column
                )?;
            } else {
                write!(
                    f,
                    " (line {}:{})",
                    frame.location.line, frame.location.column
                )?;
            }

            if let Some(context) = &frame.context {
                write!(f, " - {}", context)?;
            }
            writeln!(f)?;
        }
    }
    Ok(())
}

fn write_debug_context(
    f: &mut fmt::Formatter<'_>,
    debug_context: Option<&DebugContext>,
) -> fmt::Result {
    if let Some(ctx) = debug_context {
        if let Some(variables) = &ctx.variables {
            writeln!(f, "   🔍 Local variables:")?;
            for (name, value) in variables {
                writeln!(f, "      {} = {}", name, value)?;
            }
        }

        if !ctx.suggestions.is_empty() {
            writeln!(f, "   💡 Suggestions:")?;
            for suggestion in &ctx.suggestions {
                writeln!(f, "      • {}", suggestion)?;
            }
        }

        if let Some(help_url) = &ctx.help_url {
            writeln!(f, "   📖 Documentation: {}", help_url)?;
        }

        if !ctx.related_code.is_empty() {
            writeln!(f, "   🔗 Related code:")?;
            for code in &ctx.related_code {
                writeln!(f, "      {}", code)?;
            }
        }
    }
    Ok(())
}

impl std::error::Error for DryadError {}

impl DryadError {
    // Simplified construction methods for compatibility
    pub fn new(code: u16, msg: &str) -> Self {
        let location = SourceLocation::unknown();
        match code {
            1000..=1999 => DryadError::Lexer {
                code,
                message: msg.into(),
                location,
                debug_context: None,
            },
            2000..=2999 => DryadError::Parser {
                code,
                message: msg.into(),
                location,
                expected: Vec::new(),
                found: String::new(),
                debug_context: None,
            },
            3000..=3999 => DryadError::Runtime {
                code,
                message: msg.into(),
                location,
                stack_trace: StackTrace::new(),
                debug_context: None,
            },
            4000..=4999 => DryadError::Type {
                code,
                message: msg.into(),
                location,
                expected_type: "unknown".into(),
                found_type: "unknown".into(),
                debug_context: None,
            },
            5000..=5999 => DryadError::Io {
                code,
                message: msg.into(),
                location,
                operation: "unknown".into(),
                path: None,
                debug_context: None,
            },
            6000..=6999 => DryadError::Module {
                code,
                message: msg.into(),
                location,
                module_name: "unknown".into(),
                debug_context: None,
            },
            7000..=7999 => DryadError::Syntax {
                code,
                message: msg.into(),
                location,
                syntax_help: None,
                debug_context: None,
            },
            8000..=8999 => DryadError::Warning {
                code,
                message: msg.into(),
                location,
                severity: WarningSeverity::Medium,
                debug_context: None,
            },
            _ => DryadError::System {
                code,
                message: msg.into(),
                location,
                system_info: None,
                debug_context: None,
            },
        }
    }

    // Specific methods for creating errors with context
    pub fn lexer(code: u16, message: &str, location: SourceLocation) -> Self {
        DryadError::Lexer {
            code,
            message: message.into(),
            location,
            debug_context: None,
        }
    }

    pub fn parser(
        code: u16,
        message: &str,
        location: SourceLocation,
        expected: Vec<String>,
        found: String,
    ) -> Self {
        DryadError::Parser {
            code,
            message: message.into(),
            location,
            expected,
            found,
            debug_context: None,
        }
    }

    pub fn runtime(
        code: u16,
        message: &str,
        location: SourceLocation,
        stack_trace: StackTrace,
    ) -> Self {
        DryadError::Runtime {
            code,
            message: message.into(),
            location,
            stack_trace,
            debug_context: None,
        }
    }

    pub fn type_error(
        code: u16,
        message: &str,
        location: SourceLocation,
        expected_type: String,
        found_type: String,
    ) -> Self {
        DryadError::Type {
            code,
            message: message.into(),
            location,
            expected_type,
            found_type,
            debug_context: None,
        }
    }

    pub fn io_error(
        code: u16,
        message: &str,
        location: SourceLocation,
        operation: String,
        path: Option<PathBuf>,
    ) -> Self {
        DryadError::Io {
            code,
            message: message.into(),
            location,
            operation,
            path,
            debug_context: None,
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            DryadError::Lexer { code, .. }
            | DryadError::Parser { code, .. }
            | DryadError::Runtime { code, .. }
            | DryadError::Type { code, .. }
            | DryadError::Io { code, .. }
            | DryadError::Module { code, .. }
            | DryadError::Syntax { code, .. }
            | DryadError::Warning { code, .. }
            | DryadError::System { code, .. } => *code,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            DryadError::Lexer { message, .. }
            | DryadError::Parser { message, .. }
            | DryadError::Runtime { message, .. }
            | DryadError::Type { message, .. }
            | DryadError::Io { message, .. }
            | DryadError::Module { message, .. }
            | DryadError::Syntax { message, .. }
            | DryadError::Warning { message, .. }
            | DryadError::System { message, .. } => message,
        }
    }

    pub fn location(&self) -> &SourceLocation {
        match self {
            DryadError::Lexer { location, .. }
            | DryadError::Parser { location, .. }
            | DryadError::Runtime { location, .. }
            | DryadError::Type { location, .. }
            | DryadError::Io { location, .. }
            | DryadError::Module { location, .. }
            | DryadError::Syntax { location, .. }
            | DryadError::Warning { location, .. }
            | DryadError::System { location, .. } => location,
        }
    }

    // Adds debug context to the error
    pub fn with_debug_context(mut self, debug_context: DebugContext) -> Self {
        match &mut self {
            DryadError::Lexer {
                debug_context: ctx, ..
            }
            | DryadError::Parser {
                debug_context: ctx, ..
            }
            | DryadError::Runtime {
                debug_context: ctx, ..
            }
            | DryadError::Type {
                debug_context: ctx, ..
            }
            | DryadError::Io {
                debug_context: ctx, ..
            }
            | DryadError::Module {
                debug_context: ctx, ..
            }
            | DryadError::Syntax {
                debug_context: ctx, ..
            }
            | DryadError::Warning {
                debug_context: ctx, ..
            }
            | DryadError::System {
                debug_context: ctx, ..
            } => {
                *ctx = Some(debug_context);
            }
        }
        self
    }

    /// Automatically adds suggestions and documentation URL based on the error code
    pub fn with_auto_context(self) -> Self {
        let code = self.code();
        let suggestions = crate::error_urls::get_error_suggestions(code);
        let help_url = crate::error_urls::get_error_documentation_url(code);

        let debug_context = DebugContext::new().with_help_url(help_url);

        let debug_context = suggestions
            .into_iter()
            .fold(debug_context, |ctx, suggestion| {
                ctx.with_suggestion(suggestion)
            });

        self.with_debug_context(debug_context)
    }

    pub fn from_catalog(def: ErrorDef, location: SourceLocation) -> Self {
        Self::from_catalog_fmt(def, def.message, location)
    }

    pub fn from_catalog_fmt(def: ErrorDef, message: &str, location: SourceLocation) -> Self {
        match def.category {
            ErrorCategory::Lexer => DryadError::Lexer {
                code: def.code,
                message: message.into(),
                location,
                debug_context: None,
            },
            ErrorCategory::Parser => DryadError::Parser {
                code: def.code,
                message: message.into(),
                location,
                expected: Vec::new(),
                found: String::new(),
                debug_context: None,
            },
            ErrorCategory::Runtime => DryadError::Runtime {
                code: def.code,
                message: message.into(),
                location,
                stack_trace: StackTrace::new(),
                debug_context: None,
            },
            ErrorCategory::Type => DryadError::Type {
                code: def.code,
                message: message.into(),
                location,
                expected_type: "unknown".into(),
                found_type: "unknown".into(),
                debug_context: None,
            },
            ErrorCategory::Io => DryadError::Io {
                code: def.code,
                message: message.into(),
                location,
                operation: "unknown".into(),
                path: None,
                debug_context: None,
            },
            ErrorCategory::Module => DryadError::Module {
                code: def.code,
                message: message.into(),
                location,
                module_name: "unknown".into(),
                debug_context: None,
            },
            ErrorCategory::Syntax => DryadError::Syntax {
                code: def.code,
                message: message.into(),
                location,
                syntax_help: None,
                debug_context: None,
            },
            ErrorCategory::Warning => DryadError::Warning {
                code: def.code,
                message: message.into(),
                location,
                severity: WarningSeverity::Medium,
                debug_context: None,
            },
            ErrorCategory::System => DryadError::System {
                code: def.code,
                message: message.into(),
                location,
                system_info: None,
                debug_context: None,
            },
        }
    }
}

/// Error category for the error catalog
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Lexer,
    Parser,
    Runtime,
    Type,
    Io,
    Module,
    Syntax,
    Warning,
    System,
}

/// Error definition from the centralized error catalog
#[derive(Debug, Clone, Copy)]
pub struct ErrorDef {
    pub code: u16,
    pub category: ErrorCategory,
    pub message: &'static str,
    pub suggestion: Option<&'static str>,
}
