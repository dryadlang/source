#ifndef DRYAD_RUNTIME_VALUE_H
#define DRYAD_RUNTIME_VALUE_H

#include <cstdint>
#include <string>
#include <memory>
#include <vector>
#include <unordered_map>

namespace dryad {

class Function;
class DryadClass;
class ClassInstance;

enum class ValueType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    Array,
    Object,
    Function,
    Class,
    Instance
};

class Value {
public:
    Value();
    explicit Value(bool b);
    explicit Value(int64_t i);
    explicit Value(double d);
    explicit Value(const std::string& s);
    explicit Value(const char* s);
    explicit Value(std::shared_ptr<Function> func);
    explicit Value(std::shared_ptr<DryadClass> klass);
    explicit Value(std::shared_ptr<ClassInstance> instance);
    
    ~Value();
    
    Value(const Value& other);
    Value& operator=(const Value& other);
    Value(Value&& other) noexcept;
    Value& operator=(Value&& other) noexcept;
    
    ValueType type() const { return type_; }
    bool is_null() const { return type_ == ValueType::Null; }
    bool is_boolean() const { return type_ == ValueType::Boolean; }
    bool is_integer() const { return type_ == ValueType::Integer; }
    bool is_float() const { return type_ == ValueType::Float; }
    bool is_number() const { return is_integer() || is_float(); }
    bool is_string() const { return type_ == ValueType::String; }
    bool is_array() const { return type_ == ValueType::Array; }
    bool is_object() const { return type_ == ValueType::Object; }
    bool is_function() const { return type_ == ValueType::Function; }
    bool is_class() const { return type_ == ValueType::Class; }
    bool is_instance() const { return type_ == ValueType::Instance; }
    
    bool as_boolean() const;
    int64_t as_integer() const;
    double as_float() const;
    const std::string& as_string() const;
    std::shared_ptr<Function> as_function() const;
    std::shared_ptr<DryadClass> as_class() const;
    std::shared_ptr<ClassInstance> as_instance() const;
    
    void array_push(const Value& value);
    Value array_get(size_t index) const;
    void array_set(size_t index, const Value& value);
    size_t array_length() const;
    
    void object_set(const std::string& key, const Value& value);
    Value object_get(const std::string& key) const;
    bool object_has(const std::string& key) const;
    
    bool is_truthy() const;
    std::string to_string() const;
    
    static Value create_array();
    static Value create_object();
    
private:
    ValueType type_;
    
    union {
        bool boolean_;
        int64_t integer_;
        double float_;
    };
    
    std::string* string_;
    std::shared_ptr<std::vector<Value>> array_;
    std::shared_ptr<std::unordered_map<std::string, Value>> object_;
    std::shared_ptr<Function>* function_;
    std::shared_ptr<DryadClass>* class_;
    std::shared_ptr<ClassInstance>* instance_;
    
    void clear_data();
};

} // namespace dryad

#endif // DRYAD_RUNTIME_VALUE_H
