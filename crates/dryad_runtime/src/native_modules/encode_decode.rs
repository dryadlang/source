use crate::interpreter::Value;
use crate::native_modules::NativeFunction;
use crate::errors::RuntimeError;
use crate::heap::{Heap, ManagedObject, HeapId};
use std::collections::HashMap;
use serde_json::{Value as JsonValue, Map};
use csv::{Reader, Writer};
use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::{Reader as XmlReader, Writer as XmlWriter};
use std::io::Cursor;

/// Registra todas as funções nativas do módulo encode_decode
pub fn register_encode_decode_functions(functions: &mut HashMap<String, NativeFunction>) {
    functions.insert("native_json_encode".to_string(), native_json_encode);
    functions.insert("native_json_decode".to_string(), native_json_decode);
    functions.insert("native_csv_encode".to_string(), native_csv_encode);
    functions.insert("native_csv_decode".to_string(), native_csv_decode);
    functions.insert("native_xml_encode".to_string(), native_xml_encode);
    functions.insert("native_xml_decode".to_string(), native_xml_decode);
}

// ============================================
// JSON ENCODING/DECODING
// ============================================

// native_json_encode(data) -> string
fn native_json_encode(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_json_encode: esperado 1 argumento".to_string()));
    }
    
    let json_value = runtime_value_to_json(&args[0], _heap)?;
    let json_string = serde_json::to_string(&json_value)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao codificar JSON: {}", e)))?;
    
    Ok(Value::String(json_string))
}

// native_json_decode(json_string) -> data
fn native_json_decode(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_json_decode: esperado 1 argumento".to_string()));
    }
    
    let json_string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_json_decode: argumento deve ser string".to_string())),
    };
    
    let json_value: serde_json::Value = serde_json::from_str(json_string)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao decodificar JSON: {}", e)))?;
    
    Ok(json_to_runtime_value(&json_value, _heap))
}

// ============================================
// CSV ENCODING/DECODING  
// ============================================

// native_csv_encode(array_of_arrays) -> string
fn native_csv_encode(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_csv_encode: esperado 1 argumento".to_string()));
    }
    
    let array_id = match &args[0] {
        Value::Array(id) => *id,
        _ => return Err(RuntimeError::TypeError("native_csv_encode: argumento deve ser array".to_string())),
    };
    
    let array_obj = _heap.get(array_id).ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
    let data = match array_obj {
        ManagedObject::Array(arr) => arr,
        _ => return Err(RuntimeError::TypeError("native_csv_encode: argumento deve ser array".to_string())),
    };
    
    let mut output = Vec::new();
    {
        let mut writer = Writer::from_writer(&mut output);
        
        for row in data {
            match row {
                Value::Array(ref cols_id) => {
                    let cols_obj = _heap.get(*cols_id).ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
                    if let ManagedObject::Array(cols) = cols_obj {
                        let string_cols: Vec<String> = cols.iter()
                            .map(|v| runtime_value_to_string(v, _heap))
                            .collect();
                        writer.write_record(&string_cols)
                            .map_err(|e| RuntimeError::IoError(format!("Erro ao escrever CSV: {}", e)))?;
                    } else {
                        return Err(RuntimeError::TypeError("native_csv_encode: cada elemento deve ser um array".to_string()));
                    }
                },
                _ => return Err(RuntimeError::TypeError("native_csv_encode: cada elemento deve ser um array".to_string())),
            }
        }
        
        writer.flush()
            .map_err(|e| RuntimeError::IoError(format!("Erro ao finalizar CSV: {}", e)))?;
    }
    
    let csv_string = String::from_utf8(output)
        .map_err(|e| RuntimeError::IoError(format!("Erro de codificação UTF-8: {}", e)))?;
    
    Ok(Value::String(csv_string))
}

// native_csv_decode(csv_string) -> array_of_arrays
fn native_csv_decode(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_csv_decode: esperado 1 argumento".to_string()));
    }
    
    let csv_string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_csv_decode: argumento deve ser string".to_string())),
    };
    
    let mut reader = Reader::from_reader(csv_string.as_bytes());
    let mut result = Vec::new();
    
    for record_result in reader.records() {
        let record = record_result
            .map_err(|e| RuntimeError::IoError(format!("Erro ao ler CSV: {}", e)))?;
        
        let row: Vec<Value> = record.iter()
            .map(|field| Value::String(field.to_string()))
            .collect();
        
        let row_id = _heap.allocate(ManagedObject::Array(row));
        result.push(Value::Array(row_id));
    }
    
    let result_id = _heap.allocate(ManagedObject::Array(result));
    Ok(Value::Array(result_id))
}

// ============================================
// XML ENCODING/DECODING
// ============================================

// native_xml_encode(data, root_name?) -> string
fn native_xml_encode(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::ArgumentError("native_xml_encode: esperado 1 ou 2 argumentos".to_string()));
    }
    
    let root_name = if args.len() == 2 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("native_xml_encode: segundo argumento deve ser string".to_string())),
        }
    } else {
        "root".to_string()
    };
    
    let mut output = Vec::new();
    {
        let mut writer = XmlWriter::new(Cursor::new(&mut output));
        write_runtime_value_as_xml(&mut writer, &args[0], &root_name, _heap)?;
    }
    
    let xml_string = String::from_utf8(output)
        .map_err(|e| RuntimeError::IoError(format!("Erro de codificação UTF-8: {}", e)))?;
    
    Ok(Value::String(xml_string))
}

// native_xml_decode(xml_string) -> data
fn native_xml_decode(args: &[Value], _manager: &crate::native_modules::NativeModuleManager, _heap: &mut crate::heap::Heap) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentError("native_xml_decode: esperado 1 argumento".to_string()));
    }
    
    let xml_string = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("native_xml_decode: argumento deve ser string".to_string())),
    };
    
    let mut reader = XmlReader::from_str(xml_string);
    reader.trim_text(true);
    
    parse_xml_to_runtime_value(&mut reader, _heap)
        .map_err(|e| RuntimeError::IoError(format!("Erro ao analisar XML: {}", e)))
}

// ============================================
// HELPER FUNCTIONS
// ============================================

fn runtime_value_to_json(value: &Value, heap: &Heap) -> Result<JsonValue, RuntimeError> {
    match value {
        Value::Number(n) => Ok(JsonValue::Number(serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0)))),
        Value::String(s) => Ok(JsonValue::String(s.clone())),
        Value::Bool(b) => Ok(JsonValue::Bool(*b)),
        Value::Null => Ok(JsonValue::Null),
        Value::Array(id) => {
            let obj = heap.get(*id).ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
            if let ManagedObject::Array(arr) = obj {
                let json_array: Result<Vec<_>, _> = arr.iter()
                    .map(|v| runtime_value_to_json(v, heap))
                    .collect();
                Ok(JsonValue::Array(json_array?))
            } else {
                Err(RuntimeError::TypeError("Expected array in heap".to_string()))
            }
        }
        Value::Object(id) => {
            let obj = heap.get(*id).ok_or_else(|| RuntimeError::HeapError("Object reference not found".to_string()))?;
            if let ManagedObject::Object { properties, .. } = obj {
                let mut json_obj = Map::new();
                for (key, val) in properties {
                    json_obj.insert(key.clone(), runtime_value_to_json(&val, heap)?);
                }
                Ok(JsonValue::Object(json_obj))
            } else {
                Err(RuntimeError::TypeError("Expected object in heap".to_string()))
            }
        }
        _ => Err(RuntimeError::TypeError("Tipo não suportado para JSON".to_string())),
    }
}

pub fn json_to_runtime_value(value: &JsonValue, heap: &mut Heap) -> Value {
    match value {
        JsonValue::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        JsonValue::String(s) => Value::String(s.clone()),
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Null => Value::Null,
        JsonValue::Array(arr) => {
            let runtime_array: Vec<Value> = arr.iter()
                .map(|v| json_to_runtime_value(v, heap))
                .collect();
            let id = heap.allocate(ManagedObject::Array(runtime_array));
            Value::Array(id)
        }
        JsonValue::Object(obj) => {
            let mut runtime_obj = HashMap::new();
            for (key, val) in obj {
                runtime_obj.insert(key.clone(), json_to_runtime_value(val, heap));
            }
            let id = heap.allocate(ManagedObject::Object { 
                properties: runtime_obj, 
                methods: HashMap::new() 
            });
            Value::Object(id)
        }
    }
}

fn runtime_value_to_string(value: &Value, heap: &Heap) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(id) => {
            if let Some(ManagedObject::Array(arr)) = heap.get(*id) {
                let elements: Vec<String> = arr.iter().map(|v| runtime_value_to_string(v, heap)).collect();
                format!("[{}]", elements.join(", "))
            } else {
                "[Array]".to_string()
            }
        }
        _ => value.to_string(),
    }
}

fn write_runtime_value_as_xml<W: std::io::Write>(writer: &mut XmlWriter<W>, value: &Value, tag_name: &str, heap: &Heap) -> Result<(), RuntimeError> {
    match value {
        Value::Number(n) => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            let text_string = n.to_string();
            let text = BytesText::new(&text_string);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::Text(text)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        Value::String(s) => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            let text = BytesText::new(s);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::Text(text)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        Value::Bool(b) => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            let text_string = b.to_string();
            let text = BytesText::new(&text_string);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::Text(text)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        Value::Null => {
            let start = BytesStart::new(tag_name);
            let end = BytesEnd::new(tag_name);
            
            writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
        }
        Value::Array(id) => {
            let obj = heap.get(*id).ok_or_else(|| RuntimeError::HeapError("Array reference not found".to_string()))?;
            if let ManagedObject::Array(arr) = obj {
                let start = BytesStart::new(tag_name);
                let end = BytesEnd::new(tag_name);
                
                writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
                
                for (i, item) in arr.iter().enumerate() {
                    write_runtime_value_as_xml(writer, item, &format!("item_{}", i), heap)?;
                }
                
                writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            }
        }
        Value::Object(id) => {
            let obj = heap.get(*id).ok_or_else(|| RuntimeError::HeapError("Object reference not found".to_string()))?;
            if let ManagedObject::Object { properties, .. } = obj {
                let start = BytesStart::new(tag_name);
                let end = BytesEnd::new(tag_name);
                
                writer.write_event(Event::Start(start)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
                
                for (key, value) in properties.iter() {
                    write_runtime_value_as_xml(writer, value, key, heap)?;
                }
                
                writer.write_event(Event::End(end)).map_err(|e| RuntimeError::IoError(e.to_string()))?;
            }
        }
        _ => {
            return Err(RuntimeError::TypeError("Tipo não suportado para XML".to_string()));
        }
    }
    Ok(())
}

fn parse_xml_to_runtime_value<R: std::io::BufRead>(reader: &mut XmlReader<R>, heap: &mut Heap) -> Result<Value, String> {
    let mut buf = Vec::new();
    let mut result = HashMap::new();
    let mut text_content = String::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = std::str::from_utf8(e.name().as_ref()).unwrap_or("unknown").to_string();
                let child_value = parse_xml_to_runtime_value(reader, heap)?;
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
        let id = heap.allocate(ManagedObject::Object { 
            properties: result, 
            methods: HashMap::new() 
        });
        Ok(Value::Object(id))
    } else if !text_content.trim().is_empty() {
        if let Ok(num) = text_content.trim().parse::<f64>() {
            Ok(Value::Number(num))
        } else if text_content.trim() == "true" {
            Ok(Value::Bool(true))
        } else if text_content.trim() == "false" {
            Ok(Value::Bool(false))
        } else {
            Ok(Value::String(text_content.trim().to_string()))
        }
    } else {
        Ok(Value::Null)
    }
}