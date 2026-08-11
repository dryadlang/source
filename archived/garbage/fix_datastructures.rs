// Funções DataStructures corrigidas

    fn register_data_structures_functions(&mut self) {
        // === HashMap Functions (usando Object como base) ===
        self.functions.insert("hashmap_new".to_string(), |_args| {
            Ok(Value::Object { 
                properties: HashMap::new(),
                methods: HashMap::new()
            })
        });

        self.functions.insert("hashmap_set".to_string(), |args| {
            if args.len() != 3 {
                return Err(DryadError::new(3004, "hashmap_set espera 3 argumentos (map, key, value)"));
            }
            
            let mut map = match &args[0] {
                Value::Object { properties, .. } => properties.clone(),
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um objeto/mapa")),
            };

            let key = args[1].to_string();
            let value = args[2].clone();
            
            map.insert(key, value);
            Ok(Value::Object { 
                properties: map,
                methods: HashMap::new()
            })
        });

        self.functions.insert("hashmap_get".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "hashmap_get espera 2 argumentos (map, key)"));
            }
            
            let map = match &args[0] {
                Value::Object { properties, .. } => properties,
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um objeto/mapa")),
            };

            let key = args[1].to_string();
            
            match map.get(&key) {
                Some(value) => Ok(value.clone()),
                None => Ok(Value::Null),
            }
        });

        self.functions.insert("hashmap_has".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "hashmap_has espera 2 argumentos (map, key)"));
            }
            
            let map = match &args[0] {
                Value::Object { properties, .. } => properties,
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um objeto/mapa")),
            };

            let key = args[1].to_string();
            Ok(Value::Bool(map.contains_key(&key)))
        });

        self.functions.insert("hashmap_remove".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "hashmap_remove espera 2 argumentos (map, key)"));
            }
            
            let mut map = match &args[0] {
                Value::Object { properties, .. } => properties.clone(),
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um objeto/mapa")),
            };

            let key = args[1].to_string();
            let removed_value = map.remove(&key).unwrap_or(Value::Null);
            
            // Retorna uma tupla com o novo mapa e o valor removido
            Ok(Value::Tuple(vec![Value::Object { 
                properties: map,
                methods: HashMap::new()
            }, removed_value]))
        });

        self.functions.insert("hashmap_keys".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "hashmap_keys espera 1 argumento (map)"));
            }
            
            let map = match &args[0] {
                Value::Object { properties, .. } => properties,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um objeto/mapa")),
            };

            let keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
            Ok(Value::Array(keys))
        });

        self.functions.insert("hashmap_values".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "hashmap_values espera 1 argumento (map)"));
            }
            
            let map = match &args[0] {
                Value::Object { properties, .. } => properties,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um objeto/mapa")),
            };

            let values: Vec<Value> = map.values().cloned().collect();
            Ok(Value::Array(values))
        });

        self.functions.insert("hashmap_size".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "hashmap_size espera 1 argumento (map)"));
            }
            
            let map = match &args[0] {
                Value::Object { properties, .. } => properties,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um objeto/mapa")),
            };

            Ok(Value::Number(map.len() as f64))
        });

        self.functions.insert("hashmap_clear".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "hashmap_clear espera 1 argumento (map)"));
            }
            
            match &args[0] {
                Value::Object { .. } => Ok(Value::Object { 
                    properties: HashMap::new(),
                    methods: HashMap::new()
                }),
                _ => Err(DryadError::new(3002, "Argumento deve ser um objeto/mapa")),
            }
        });

        // === Set Functions (usando Array como base) ===
        self.functions.insert("set_new".to_string(), |_args| {
            Ok(Value::Array(Vec::new()))
        });

        self.functions.insert("set_add".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "set_add espera 2 argumentos (set, value)"));
            }
            
            let mut set = match &args[0] {
                Value::Array(arr) => arr.clone(),
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array/set")),
            };

            let value = &args[1];
            
            // Verifica se o valor já existe no set
            if !set.iter().any(|v| values_equal(v, &value)) {
                set.push(value.clone());
            }
            
            Ok(Value::Array(set))
        });

        self.functions.insert("set_has".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "set_has espera 2 argumentos (set, value)"));
            }
            
            let set = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array/set")),
            };

            let value = &args[1];
            Ok(Value::Bool(set.iter().any(|v| values_equal(v, value))))
        });

        self.functions.insert("set_remove".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "set_remove espera 2 argumentos (set, value)"));
            }
            
            let mut set = match &args[0] {
                Value::Array(arr) => arr.clone(),
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array/set")),
            };

            let value = &args[1];
            set.retain(|v| !values_equal(v, value));
            
            Ok(Value::Array(set))
        });

        self.functions.insert("set_size".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "set_size espera 1 argumento (set)"));
            }
            
            let set = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/set")),
            };

            Ok(Value::Number(set.len() as f64))
        });

        self.functions.insert("set_union".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "set_union espera 2 argumentos (set1, set2)"));
            }
            
            let set1 = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array/set")),
            };

            let set2 = match &args[1] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Segundo argumento deve ser um array/set")),
            };

            let mut result = set1.clone();
            
            for value in set2 {
                if !result.iter().any(|v| values_equal(v, value)) {
                    result.push(value.clone());
                }
            }
            
            Ok(Value::Array(result))
        });

        self.functions.insert("set_intersection".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "set_intersection espera 2 argumentos (set1, set2)"));
            }
            
            let set1 = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array/set")),
            };

            let set2 = match &args[1] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Segundo argumento deve ser um array/set")),
            };

            let mut result = Vec::new();
            
            for value in set1 {
                if set2.iter().any(|v| values_equal(v, value)) {
                    result.push(value.clone());
                }
            }
            
            Ok(Value::Array(result))
        });

        // === Stack Functions (usando Array como base) ===
        self.functions.insert("stack_new".to_string(), |_args| {
            Ok(Value::Array(Vec::new()))
        });

        self.functions.insert("stack_push".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "stack_push espera 2 argumentos (stack, value)"));
            }
            
            let mut stack = match &args[0] {
                Value::Array(arr) => arr.clone(),
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array/stack")),
            };

            stack.push(args[1].clone());
            Ok(Value::Array(stack))
        });

        self.functions.insert("stack_pop".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "stack_pop espera 1 argumento (stack)"));
            }
            
            let mut stack = match &args[0] {
                Value::Array(arr) => arr.clone(),
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/stack")),
            };

            let popped = stack.pop().unwrap_or(Value::Null);
            Ok(Value::Tuple(vec![Value::Array(stack), popped]))
        });

        self.functions.insert("stack_peek".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "stack_peek espera 1 argumento (stack)"));
            }
            
            let stack = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/stack")),
            };

            Ok(stack.last().cloned().unwrap_or(Value::Null))
        });

        self.functions.insert("stack_size".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "stack_size espera 1 argumento (stack)"));
            }
            
            let stack = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/stack")),
            };

            Ok(Value::Number(stack.len() as f64))
        });

        self.functions.insert("stack_is_empty".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "stack_is_empty espera 1 argumento (stack)"));
            }
            
            let stack = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/stack")),
            };

            Ok(Value::Bool(stack.is_empty()))
        });

        // === Queue Functions (usando Array como base) ===
        self.functions.insert("queue_new".to_string(), |_args| {
            Ok(Value::Array(Vec::new()))
        });

        self.functions.insert("queue_enqueue".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "queue_enqueue espera 2 argumentos (queue, value)"));
            }
            
            let mut queue = match &args[0] {
                Value::Array(arr) => arr.clone(),
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array/queue")),
            };

            queue.push(args[1].clone()); // Adiciona no final
            Ok(Value::Array(queue))
        });

        self.functions.insert("queue_dequeue".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "queue_dequeue espera 1 argumento (queue)"));
            }
            
            let mut queue = match &args[0] {
                Value::Array(arr) => arr.clone(),
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/queue")),
            };

            let dequeued = if !queue.is_empty() {
                Some(queue.remove(0)) // Remove do início
            } else {
                None
            };

            Ok(Value::Tuple(vec![Value::Array(queue), dequeued.unwrap_or(Value::Null)]))
        });

        self.functions.insert("queue_front".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "queue_front espera 1 argumento (queue)"));
            }
            
            let queue = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/queue")),
            };

            Ok(queue.first().cloned().unwrap_or(Value::Null))
        });

        self.functions.insert("queue_size".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "queue_size espera 1 argumento (queue)"));
            }
            
            let queue = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/queue")),
            };

            Ok(Value::Number(queue.len() as f64))
        });

        self.functions.insert("queue_is_empty".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "queue_is_empty espera 1 argumento (queue)"));
            }
            
            let queue = match &args[0] {
                Value::Array(arr) => arr,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array/queue")),
            };

            Ok(Value::Bool(queue.is_empty()))
        });

        // === Array Utility Functions ===
        self.functions.insert("array_reverse".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "array_reverse espera 1 argumento (array)"));
            }
            
            let mut arr = match &args[0] {
                Value::Array(a) => a.clone(),
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array")),
            };

            arr.reverse();
            Ok(Value::Array(arr))
        });

        self.functions.insert("array_sort".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "array_sort espera 1 argumento (array)"));
            }
            
            let mut arr = match &args[0] {
                Value::Array(a) => a.clone(),
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array")),
            };

            // Ordenação simples por string representation
            arr.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
            Ok(Value::Array(arr))
        });

        self.functions.insert("array_unique".to_string(), |args| {
            if args.len() != 1 {
                return Err(DryadError::new(3004, "array_unique espera 1 argumento (array)"));
            }
            
            let arr = match &args[0] {
                Value::Array(a) => a,
                _ => return Err(DryadError::new(3002, "Argumento deve ser um array")),
            };

            let mut unique = Vec::new();
            for value in arr {
                if !unique.iter().any(|v| values_equal(v, value)) {
                    unique.push(value.clone());
                }
            }

            Ok(Value::Array(unique))
        });

        self.functions.insert("array_concat".to_string(), |args| {
            if args.len() != 2 {
                return Err(DryadError::new(3004, "array_concat espera 2 argumentos (array1, array2)"));
            }
            
            let arr1 = match &args[0] {
                Value::Array(a) => a,
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array")),
            };

            let arr2 = match &args[1] {
                Value::Array(a) => a,
                _ => return Err(DryadError::new(3002, "Segundo argumento deve ser um array")),
            };

            let mut result = arr1.clone();
            result.extend(arr2.clone());
            Ok(Value::Array(result))
        });

        self.functions.insert("array_slice".to_string(), |args| {
            if args.len() < 2 || args.len() > 3 {
                return Err(DryadError::new(3004, "array_slice espera 2 ou 3 argumentos (array, start, [end])"));
            }
            
            let arr = match &args[0] {
                Value::Array(a) => a,
                _ => return Err(DryadError::new(3002, "Primeiro argumento deve ser um array")),
            };

            let start = match &args[1] {
                Value::Number(n) => *n as usize,
                _ => return Err(DryadError::new(3002, "Start deve ser um número")),
            };

            let end = if args.len() == 3 {
                match &args[2] {
                    Value::Number(n) => (*n as usize).min(arr.len()),
                    _ => return Err(DryadError::new(3002, "End deve ser um número")),
                }
            } else {
                arr.len()
            };

            if start > arr.len() {
                return Ok(Value::Array(Vec::new()));
            }

            let result = arr[start..end.min(arr.len())].to_vec();
            Ok(Value::Array(result))
        });
    }

    // Função auxiliar para comparar valores profundamente
    fn values_equal(v1: &Value, v2: &Value) -> bool {
        match (v1, v2) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Null, Value::Null) => true,
            (Value::Array(a1), Value::Array(a2)) => {
                a1.len() == a2.len() && a1.iter().zip(a2.iter()).all(|(v1, v2)| values_equal(v1, v2))
            }
            _ => false,
        }
    }
