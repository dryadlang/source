use crate::interpreter::RuntimeValue;
use crate::errors::RuntimeError;
use std::collections::HashMap;
use serde_json::{Value, Map};
use csv::{Reader, Writer};
use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::{Reader as XmlReader, Writer as XmlWriter};
use std::io::Cursor;

/// Registra todas as funções nativas do módulo encode_decode
pub fn register_encode_decode_functions(functions: &mut HashMap<String, fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>>) {
    functions.insert("native_json_encode".to_string(), native_json_encode as fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>);
    functions.insert("native_json_decode".to_string(), native_json_decode as fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>);
    functions.insert("native_csv_encode".to_string(), native_csv_encode as fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>);
    functions.insert("native_csv_decode".to_string(), native_csv_decode as fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>);
    functions.insert("native_xml_encode".to_string(), native_xml_encode as fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>);
    functions.insert("native_xml_decode".to_string(), native_xml_decode as fn(&[RuntimeValue]) -> Result<RuntimeValue, RuntimeError>);
}

// ============================================
// JSON ENCODING/DECODING
// ============================================

// native_json_encode(data) -> string
fn native_json_encode(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_json_encode: esperado 1 argumento".to_string()));
    }
    
    let json_value = runtime_value_to_json(&args[0])?;
    let json_string = serde_json::to_string(&json_value)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao codificar JSON: {}", e)))?;
    
    Ok(RuntimeValue::String(json_string))
}

// native_json_decode(json_string) -> data
fn native_json_decode(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_json_decode: esperado 1 argumento".to_string()));
    }
    
    let json_string = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_json_decode: argumento deve ser string".to_string())),
    };
    
    let json_value: Value = serde_json::from_str(json_string)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao decodificar JSON: {}", e)))?;
    
    Ok(json_to_runtime_value(&json_value))
}

// ============================================
// CSV ENCODING/DECODING  
// ============================================

// native_csv_encode(array_of_arrays) -> string
fn native_csv_encode(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_csv_encode: esperado 1 argumento".to_string()));
    }
    
    let data = match &args[0] {
        RuntimeValue::Array(arr) => arr,
        _ => return Err(RuntimeError::TypeError("native_csv_encode: argumento deve ser array".to_string())),
    };
    
    let mut output = Vec::new();
    {
        let mut writer = Writer::from_writer(&mut output);
        
        for row in data {
            match row {
                RuntimeValue::Array(cols) => {
                    let string_cols: Vec<String> = cols.iter()
                        .map(|v| runtime_value_to_string(v))
                        .collect();
                    writer.write_record(&string_cols)
                        .map_err(|e| RuntimeError::IoError(format!("Erro ao escrever CSV: {}", e)))?;
                },
                _ => return Err(RuntimeError::TypeError("native_csv_encode: cada elemento deve ser um array".to_string())),
            }
        }
        
        writer.flush()
            .map_err(|e| RuntimeError::IoError(format!("Erro ao finalizar CSV: {}", e)))?;
    }
    
    let csv_string = String::from_utf8(output)
        .map_err(|e| RuntimeError::IoError(format!("Erro de codificação UTF-8: {}", e)))?;
    
    Ok(RuntimeValue::String(csv_string))
}

// native_csv_decode(csv_string) -> array_of_arrays
fn native_csv_decode(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_csv_decode: esperado 1 argumento".to_string()));
    }
    
    let csv_string = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_csv_decode: argumento deve ser string".to_string())),
    };
    
    let mut reader = Reader::from_reader(csv_string.as_bytes());
    let mut result = Vec::new();
    
    for record_result in reader.records() {
        let record = record_result
            .map_err(|e| RuntimeError::IoError(format!("Erro ao ler CSV: {}", e)))?;
        
        let row: Vec<RuntimeValue> = record.iter()
            .map(|field| RuntimeValue::String(field.to_string()))
            .collect();
        
        result.push(RuntimeValue::Array(row));
    }
    
    Ok(RuntimeValue::Array(result))
}

// ============================================
// XML ENCODING/DECODING
// ============================================

// native_xml_encode(data, root_name?) -> string
fn native_xml_encode(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::ArgumentError("native_xml_encode: esperado 1 ou 2 argumentos".to_string()));
    }
    
    let root_name = if args.len() == 2 {
        match &args[1] {
            RuntimeValue::String(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("native_xml_encode: segundo argumento deve ser string".to_string())),
        }
    } else {
        "root".to_string()
    };
    
    let mut output = Vec::new();
    {
        let mut writer = XmlWriter::new(Cursor::new(&mut output));
        write_runtime_value_as_xml(&mut writer, &args[0], &root_name)?;
    }
    
    let xml_string = String::from_utf8(output)
        .map_err(|e| RuntimeError::IoError(format!("Erro de codificação UTF-8: {}", e)))?;
    
    Ok(RuntimeValue::String(xml_string))
}

// native_xml_decode(xml_string) -> data
fn native_xml_decode(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_xml_decode: esperado 1 argumento".to_string()));
    }
    
    let xml_string = match &args[0] {
        RuntimeValue::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_xml_decode: argumento deve ser string".to_string())),
    };
    
    let mut reader = XmlReader::from_str(xml_string);
    reader.trim_text(true);
    
    parse_xml_to_runtime_value(&mut reader)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao analisar XML: {}", e)))
}

// ============================================
// HELPER FUNCTIONS
// ============================================

fn runtime_value_to_json(value: &RuntimeValue) -> Result<Value, RuntimeError> {
    match value {
        RuntimeValue::Number(n) => Ok(Value::Number(serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0)))),
        RuntimeValue::String(s) => Ok(Value::String(s.clone())),
        RuntimeValue::Bool(b) => Ok(Value::Bool(*b)),
        RuntimeValue::Null => Ok(Value::Null),
        RuntimeValue::Array(arr) => {
            let json_array: Result<Vec<_>, _> = arr.iter()
                .map(runtime_value_to_json)
                .collect();
            Ok(Value::Array(json_array?))
        }
        RuntimeValue::Object { properties, methods: _ } => {
            let mut json_obj = Map::new();
            for (key, val) in properties {
                json_obj.insert(key.clone(), runtime_value_to_json(val)?);
            }
            Ok(Value::Object(json_obj))
        }
        _ => Err(RuntimeError::TypeError("Tipo não suportado para JSON".to_string())),
    }
}

pub fn json_to_runtime_value(value: &Value) -> RuntimeValue {
    match value {
        Value::Number(n) => RuntimeValue::Number(n.as_f64().unwrap_or(0.0)),
        Value::String(s) => RuntimeValue::String(s.clone()),
        Value::Bool(b) => RuntimeValue::Bool(*b),
        Value::Null => RuntimeValue::Null,
        Value::Array(arr) => {
            let runtime_array: Vec<RuntimeValue> = arr.iter()
                .map(json_to_runtime_value)
                .collect();
            RuntimeValue::Array(runtime_array)
        }
        Value::Object(obj) => {
            let mut runtime_obj = HashMap::new();
            for (key, val) in obj {
                runtime_obj.insert(key.clone(), json_to_runtime_value(val));
            }
            RuntimeValue::Object { 
                properties: runtime_obj, 
                methods: HashMap::new() 
            }
        }
    }
}

fn runtime_value_to_string(value: &RuntimeValue) -> String {
    match value {
        RuntimeValue::Number(n) => n.to_string(),
        RuntimeValue::String(s) => s.clone(),
        RuntimeValue::Bool(b) => b.to_string(),
        RuntimeValue::Null => "null".to_string(),
        _ => format!("{:?}", value),
    }
}

fn write_runtime_value_as_xml<W: std::io::Write>(writer: &mut XmlWriter<W>, value: &RuntimeValue, tag_name: &str) -> Result<(), RuntimeError> {
    match value {
        RuntimeValue::Number(n) => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            let text_string = n.to_string();
            let text = BytesText::new(&text_string);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::Text(text)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        RuntimeValue::String(s) => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            let text = BytesText::new(s);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::Text(text)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        RuntimeValue::Bool(b) => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            let text_string = b.to_string();
            let text = BytesText::new(&text_string);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::Text(text)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        RuntimeValue::Null => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        RuntimeValue::Array(arr) => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            
            for (i, item) in arr.iter().enumerate() {
                write_runtime_value_as_xml(writer, item, &format!("item_{}", i))?;
            }
            
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        RuntimeValue::Object { properties, methods: _ } => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            
            for (key, value) in properties.iter() {
                write_runtime_value_as_xml(writer, value, key)?;
            }
            
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        _ => {
            return Err(RuntimeError::TypeError("Tipo não suportado para XML".to_string()));
        }
    }
    Ok(())
}

fn parse_xml_to_runtime_value<R: std::io::BufRead>(reader: &mut XmlReader<R>) -> Result<RuntimeValue, String> {
    let mut buf = Vec::new();
    let mut result = HashMap::new();
    let mut text_content = String::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = std::str::from_utf8(e.name().as_ref()).unwrap_or("unknown").to_string();
                let child_value = parse_xml_to_runtime_value(reader)?;
                result.insert(name, child_value);
            }
            Ok(Event::End(_)) => {
                break;
            }
            Ok(Event::Text(e)) => {
                text_content.push_str(&e.unescape().unwrap_or_default());
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Erro XML: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    if !result.is_empty() {
        Ok(RuntimeValue::Object { 
            properties: result, 
            methods: HashMap::new() 
        })
    } else if !text_content.trim().is_empty() {
        if let Ok(num) = text_content.trim().parse::<f64>() {
            Ok(RuntimeValue::Number(num))
        } else if text_content.trim() == "true" {
            Ok(RuntimeValue::Bool(true))
        } else if text_content.trim() == "false" {
            Ok(RuntimeValue::Bool(false))
        } else {
            Ok(RuntimeValue::String(text_content.trim().to_string()))
        }
    } else {
        Ok(RuntimeValue::Null)
    }
}