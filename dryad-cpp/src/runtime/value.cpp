#include "dryad/runtime/value.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/class.h"
#include "dryad/common/utils.h"
#include <sstream>

namespace dryad {

Value::Value() : type_(ValueType::Null), integer_(0), string_(nullptr), array_(nullptr), object_(nullptr), function_(nullptr), class_(nullptr), instance_(nullptr) {}

Value::Value(bool b) : type_(ValueType::Boolean), boolean_(b), string_(nullptr), array_(nullptr), object_(nullptr), function_(nullptr), class_(nullptr), instance_(nullptr) {}

Value::Value(int64_t i) : type_(ValueType::Integer), integer_(i), string_(nullptr), array_(nullptr), object_(nullptr), function_(nullptr), class_(nullptr), instance_(nullptr) {}

Value::Value(double d) : type_(ValueType::Float), float_(d), string_(nullptr), array_(nullptr), object_(nullptr), function_(nullptr), class_(nullptr), instance_(nullptr) {}

Value::Value(const std::string& s) : type_(ValueType::String), integer_(0), string_(new std::string(s)), array_(nullptr), object_(nullptr), function_(nullptr), class_(nullptr), instance_(nullptr) {}

Value::Value(const char* s) : type_(ValueType::String), integer_(0), string_(new std::string(s)), array_(nullptr), object_(nullptr), function_(nullptr), class_(nullptr), instance_(nullptr) {}

Value::Value(std::shared_ptr<Function> func) : type_(ValueType::Function), integer_(0), string_(nullptr), array_(nullptr), object_(nullptr), function_(new std::shared_ptr<Function>(std::move(func))), class_(nullptr), instance_(nullptr) {}

Value::Value(std::shared_ptr<DryadClass> klass) : type_(ValueType::Class), integer_(0), string_(nullptr), array_(nullptr), object_(nullptr), function_(nullptr), class_(new std::shared_ptr<DryadClass>(std::move(klass))), instance_(nullptr) {}

Value::Value(std::shared_ptr<ClassInstance> inst) : type_(ValueType::Instance), integer_(0), string_(nullptr), array_(nullptr), object_(nullptr), function_(nullptr), class_(nullptr), instance_(new std::shared_ptr<ClassInstance>(std::move(inst))) {}

Value::~Value() {
    clear_data();
}

void Value::clear_data() {
    if (type_ == ValueType::String) {
        delete string_;
    } else if (type_ == ValueType::Function) {
        delete function_;
    } else if (type_ == ValueType::Class) {
        delete class_;
    } else if (type_ == ValueType::Instance) {
        delete instance_;
    }
    string_ = nullptr;
    array_ = nullptr;
    object_ = nullptr;
    function_ = nullptr;
    class_ = nullptr;
    instance_ = nullptr;
}

Value::Value(const Value& other) : type_(other.type_), integer_(0), string_(nullptr), array_(nullptr), object_(nullptr), function_(nullptr) {
    switch (type_) {
        case ValueType::Null:
            break;
        case ValueType::Boolean:
            boolean_ = other.boolean_;
            break;
        case ValueType::Integer:
            integer_ = other.integer_;
            break;
        case ValueType::Float:
            float_ = other.float_;
            break;
        case ValueType::String:
            string_ = new std::string(*other.string_);
            break;
        case ValueType::Array:
            array_ = other.array_;
            break;
        case ValueType::Object:
            object_ = other.object_;
            break;
        case ValueType::Function:
            function_ = new std::shared_ptr<Function>(*other.function_);
            break;
        case ValueType::Class:
            class_ = new std::shared_ptr<DryadClass>(*other.class_);
            break;
        case ValueType::Instance:
            instance_ = new std::shared_ptr<ClassInstance>(*other.instance_);
            break;
    }
}

Value& Value::operator=(const Value& other) {
    if (this != &other) {
        clear_data();
        type_ = other.type_;
        
        switch (type_) {
            case ValueType::Null:
                break;
            case ValueType::Boolean:
                boolean_ = other.boolean_;
                break;
            case ValueType::Integer:
                integer_ = other.integer_;
                break;
            case ValueType::Float:
                float_ = other.float_;
                break;
            case ValueType::String:
                string_ = new std::string(*other.string_);
                break;
            case ValueType::Array:
                array_ = other.array_;
                break;
            case ValueType::Object:
                object_ = other.object_;
                break;
            case ValueType::Function:
                function_ = new std::shared_ptr<Function>(*other.function_);
                break;
            case ValueType::Class:
                class_ = new std::shared_ptr<DryadClass>(*other.class_);
                break;
            case ValueType::Instance:
                instance_ = new std::shared_ptr<ClassInstance>(*other.instance_);
                break;
        }
    }
    return *this;
}

Value::Value(Value&& other) noexcept : type_(other.type_), integer_(other.integer_), string_(other.string_), array_(other.array_), object_(other.object_), function_(other.function_), class_(other.class_), instance_(other.instance_) {
    other.type_ = ValueType::Null;
    other.string_ = nullptr;
    other.array_ = nullptr;
    other.object_ = nullptr;
    other.function_ = nullptr;
    other.class_ = nullptr;
    other.instance_ = nullptr;
}

Value& Value::operator=(Value&& other) noexcept {
    if (this != &other) {
        clear_data();
        type_ = other.type_;
        integer_ = other.integer_;
        string_ = other.string_;
        array_ = other.array_;
        object_ = other.object_;
        function_ = other.function_;
        class_ = other.class_;
        instance_ = other.instance_;
        
        other.type_ = ValueType::Null;
        other.string_ = nullptr;
        other.array_ = nullptr;
        other.object_ = nullptr;
        other.function_ = nullptr;
        other.class_ = nullptr;
        other.instance_ = nullptr;
    }
    return *this;
}

bool Value::as_boolean() const {
    if (type_ != ValueType::Boolean) {
        throw DryadException("Value is not a boolean");
    }
    return boolean_;
}

int64_t Value::as_integer() const {
    if (type_ != ValueType::Integer) {
        throw DryadException("Value is not an integer");
    }
    return integer_;
}

double Value::as_float() const {
    if (type_ == ValueType::Float) {
        return float_;
    } else if (type_ == ValueType::Integer) {
        return static_cast<double>(integer_);
    }
    throw DryadException("Value is not a number");
}

const std::string& Value::as_string() const {
    if (type_ != ValueType::String) {
        throw DryadException("Value is not a string");
    }
    return *string_;
}

std::shared_ptr<Function> Value::as_function() const {
    if (type_ != ValueType::Function) {
        throw DryadException("Value is not a function");
    }
    return *function_;
}

std::shared_ptr<DryadClass> Value::as_class() const {
    if (type_ != ValueType::Class) {
        throw DryadException("Value is not a class");
    }
    return *class_;
}

std::shared_ptr<ClassInstance> Value::as_instance() const {
    if (type_ != ValueType::Instance) {
        throw DryadException("Value is not an instance");
    }
    return *instance_;
}

void Value::array_push(const Value& value) {
    if (type_ != ValueType::Array) {
        throw DryadException("Value is not an array");
    }
    array_->push_back(value);
}

Value Value::array_get(size_t index) const {
    if (type_ != ValueType::Array) {
        throw DryadException("Value is not an array");
    }
    if (index >= array_->size()) {
        throw DryadException("Array index out of bounds");
    }
    return (*array_)[index];
}

void Value::array_set(size_t index, const Value& value) {
    if (type_ != ValueType::Array) {
        throw DryadException("Value is not an array");
    }
    if (index >= array_->size()) {
        throw DryadException("Array index out of bounds");
    }
    (*array_)[index] = value;
}

size_t Value::array_length() const {
    if (type_ != ValueType::Array) {
        throw DryadException("Value is not an array");
    }
    return array_->size();
}

void Value::object_set(const std::string& key, const Value& value) {
    if (type_ != ValueType::Object) {
        throw DryadException("Value is not an object");
    }
    (*object_)[key] = value;
}

Value Value::object_get(const std::string& key) const {
    if (type_ != ValueType::Object) {
        throw DryadException("Value is not an object");
    }
    auto it = object_->find(key);
    if (it == object_->end()) {
        return Value();
    }
    return it->second;
}

bool Value::object_has(const std::string& key) const {
    if (type_ != ValueType::Object) {
        throw DryadException("Value is not an object");
    }
    return object_->find(key) != object_->end();
}

bool Value::is_truthy() const {
    switch (type_) {
        case ValueType::Null:
            return false;
        case ValueType::Boolean:
            return boolean_;
        case ValueType::Integer:
            return integer_ != 0;
        case ValueType::Float:
            return float_ != 0.0;
        case ValueType::String:
            return !string_->empty();
        case ValueType::Array:
            return !array_->empty();
        case ValueType::Object:
            return !object_->empty();
        case ValueType::Function:
            return true;
        case ValueType::Class:
            return true;
        case ValueType::Instance:
            return true;
    }
    return false;
}

std::string Value::to_string() const {
    std::stringstream ss;
    switch (type_) {
        case ValueType::Null:
            return "null";
        case ValueType::Boolean:
            return boolean_ ? "true" : "false";
        case ValueType::Integer:
            return std::to_string(integer_);
        case ValueType::Float:
            ss << float_;
            return ss.str();
        case ValueType::String:
            return *string_;
        case ValueType::Array:
            return "[Array]";
        case ValueType::Object:
            return "[Object]";
        case ValueType::Function:
            return "[Function]";
        case ValueType::Class:
            return "[Class " + (*class_)->name + "]";
        case ValueType::Instance:
            return "[Instance of " + (*instance_)->klass->name + "]";
    }
    return "unknown";
}

Value Value::create_array() {
    Value v;
    v.type_ = ValueType::Array;
    v.array_ = std::make_shared<std::vector<Value>>();
    return v;
}

Value Value::create_object() {
    Value v;
    v.type_ = ValueType::Object;
    v.object_ = std::make_shared<std::unordered_map<std::string, Value>>();
    return v;
}

} // namespace dryad
