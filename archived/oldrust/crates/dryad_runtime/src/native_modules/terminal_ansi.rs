/// Módulo de controle de terminal ANSI
/// 
/// Fornece funções nativas para manipulação avançada do terminal
/// incluindo limpeza de tela, movimentação de cursor, cores e estilos

use crate::interpreter::Value;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use std::io::{self, Write};

use crate::native_modules::NativeFunction;

/// Registra todas as funções do módulo terminal_ansi
pub fn register_terminal_ansi_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("native_clear_screen".to_string(), native_clear_screen);
    functions.insert("native_move_cursor".to_string(), native_move_cursor);
    functions.insert("native_set_color".to_string(), native_set_color);
    functions.insert("native_set_style".to_string(), native_set_style);
    functions.insert("native_reset_style".to_string(), native_reset_style);
    functions.insert("native_hide_cursor".to_string(), native_hide_cursor);
    functions.insert("native_show_cursor".to_string(), native_show_cursor);
    functions.insert("native_terminal_size".to_string(), native_terminal_size);
    functions.insert("ansi_red".to_string(), native_ansi_red);
}

/// Limpa a tela do terminal, movendo o cursor para o início
fn native_clear_screen(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    // ANSI escape sequence para limpar tela e mover cursor para home
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;
    Ok(Value::Null)
}

/// Move o cursor para uma posição específica (x, y)
fn native_move_cursor(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_move_cursor() requer exatamente 2 argumentos (x, y)".to_string()
        ));
    }

    let x = match &args[0] {
        Value::Number(n) => *n as u16,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento (x) deve ser um número".to_string()
        )),
    };

    let y = match &args[1] {
        Value::Number(n) => *n as u16,
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento (y) deve ser um número".to_string()
        )),
    };

    // ANSI escape sequence para mover cursor
    // Nota: coordenadas ANSI começam em 1,1
    print!("\x1B[{};{}H", y + 1, x + 1);
    io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;
    
    Ok(Value::Null)
}

/// Define a cor do texto e do fundo
fn native_set_color(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::ArgumentError(
            "native_set_color() requer exatamente 2 argumentos (foreground, background)".to_string()
        ));
    }

    let fg = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Primeiro argumento (foreground) deve ser uma string".to_string()
        )),
    };

    let bg = match &args[1] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Segundo argumento (background) deve ser uma string".to_string()
        )),
    };

    let fg_code = color_to_ansi_fg(fg)?;
    let bg_code = color_to_ansi_bg(bg)?;

    print!("\x1B[{};{}m", fg_code, bg_code);
    io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;
    
    Ok(Value::Null)
}

/// Define o estilo do texto
fn native_set_style(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError(
            "native_set_style() requer exatamente 1 argumento (style)".to_string()
        ));
    }

    let style = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError(
            "Argumento deve ser uma string representando o estilo".to_string()
        )),
    };

    let style_code = style_to_ansi(style)?;
    print!("\x1B[{}m", style_code);
    io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;
    
    Ok(Value::Null)
}

/// Reseta o estilo do texto para o padrão do terminal
fn native_reset_style(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    // ANSI escape sequence para reset completo
    print!("\x1B[0m");
    io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;
    Ok(Value::Null)
}

/// Oculta o cursor do terminal
fn native_hide_cursor(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    // ANSI escape sequence para ocultar cursor
    print!("\x1B[?25l");
    io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;
    Ok(Value::Null)
}

/// Mostra o cursor do terminal
fn native_show_cursor(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    // ANSI escape sequence para mostrar cursor
    print!("\x1B[?25h");
    io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;
    Ok(Value::Null)
}

/// Retorna o tamanho do terminal como uma tupla (colunas, linhas)
fn native_terminal_size(_args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    // Usando crossterm para obter o tamanho do terminal de forma cross-platform
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let fd = io::stdout().as_raw_fd();
        
        unsafe {
            let mut winsize: libc::winsize = std::mem::zeroed();
            if libc::ioctl(fd, libc::TIOCGWINSZ, &mut winsize) == 0 {
                return Ok(Value::String(format!("({}, {})", winsize.ws_col, winsize.ws_row)));
            }
        }
    }
    
    #[cfg(windows)]
    {
        use std::ptr;
        use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO};
        use winapi::um::processenv::GetStdHandle;
        use winapi::um::winbase::STD_OUTPUT_HANDLE;
        
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if handle != ptr::null_mut() {
                let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
                if GetConsoleScreenBufferInfo(handle, &mut csbi) != 0 {
                    let cols = csbi.srWindow.Right - csbi.srWindow.Left + 1;
                    let rows = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;
                    return Ok(Value::String(format!("({}, {})", cols, rows)));
                }
            }
        }
    }
    
    // Fallback: tamanho padrão
    Ok(Value::String("(80, 24)".to_string()))
}

/// Converte nome de cor para código ANSI de foreground
fn color_to_ansi_fg(color: &str) -> Result<u8, RuntimeError> {
    match color.to_lowercase().as_str() {
        "black" => Ok(30),
        "red" => Ok(31),
        "green" => Ok(32),
        "yellow" => Ok(33),
        "blue" => Ok(34),
        "magenta" => Ok(35),
        "cyan" => Ok(36),
        "white" => Ok(37),
        "bright_black" | "gray" => Ok(90),
        "bright_red" => Ok(91),
        "bright_green" => Ok(92),
        "bright_yellow" => Ok(93),
        "bright_blue" => Ok(94),
        "bright_magenta" => Ok(95),
        "bright_cyan" => Ok(96),
        "bright_white" => Ok(97),
        _ => {
            // Tentar interpretar como código hexadecimal
            if color.starts_with('#') && color.len() == 7 {
                // Para cores hexadecimais, usar código 38;2;r;g;b
                if let Ok(_rgb) = u32::from_str_radix(&color[1..], 16) {
                    // Por simplicidade, vamos mapear para cor mais próxima
                    return Ok(37); // white como fallback
                }
            }
            Err(RuntimeError::ArgumentError(
                format!("Cor '{}' não reconhecida. Use: black, red, green, yellow, blue, magenta, cyan, white, ou códigos hex #RRGGBB", color)
            ))
        }
    }
}

/// Converte nome de cor para código ANSI de background
fn color_to_ansi_bg(color: &str) -> Result<u8, RuntimeError> {
    match color.to_lowercase().as_str() {
        "black" => Ok(40),
        "red" => Ok(41),
        "green" => Ok(42),
        "yellow" => Ok(43),
        "blue" => Ok(44),
        "magenta" => Ok(45),
        "cyan" => Ok(46),
        "white" => Ok(47),
        "bright_black" | "gray" => Ok(100),
        "bright_red" => Ok(101),
        "bright_green" => Ok(102),
        "bright_yellow" => Ok(103),
        "bright_blue" => Ok(104),
        "bright_magenta" => Ok(105),
        "bright_cyan" => Ok(106),
        "bright_white" => Ok(107),
        _ => {
            // Tentar interpretar como código hexadecimal
            if color.starts_with('#') && color.len() == 7 {
                // Para cores hexadecimais, usar código 48;2;r;g;b
                return Ok(47); // white background como fallback
            }
            Err(RuntimeError::ArgumentError(
                format!("Cor de fundo '{}' não reconhecida. Use: black, red, green, yellow, blue, magenta, cyan, white, ou códigos hex #RRGGBB", color)
            ))
        }
    }
}

/// Converte nome de estilo para código ANSI
fn style_to_ansi(style: &str) -> Result<u8, RuntimeError> {
    match style.to_lowercase().as_str() {
        "reset" => Ok(0),
        "bold" => Ok(1),
        "dim" => Ok(2),
        "italic" => Ok(3),
        "underline" => Ok(4),
        "blink" => Ok(5),
        "reverse" => Ok(7),
        "strikethrough" => Ok(9),
        _ => Err(RuntimeError::ArgumentError(
            format!("Estilo '{}' não reconhecido. Use: reset, bold, dim, italic, underline, blink, reverse, strikethrough", style)
        ))
    }
}

/// Retorna o texto na cor vermelha ANSI
fn native_ansi_red(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("ansi_red espera 1 argumento".to_string()));
    }
    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("ansi_red espera string".to_string())),
    };
    Ok(Value::String(format!("\x1B[31m{}\x1B[0m", text)))
}
