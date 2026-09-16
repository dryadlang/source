#include "dryad/runtime/bytecode_module.h"
#include "dryad/common/utils.h"
#include <cstring>
#include <fstream>

namespace dryad {

namespace {

constexpr char kMagic[] = "DRYADC";
constexpr uint32_t kFormatVersion = 1;

void write_u32(std::vector<uint8_t>& out, uint32_t value) {
    for (int i = 0; i < 4; ++i) {
        out.push_back(static_cast<uint8_t>((value >> (i * 8)) & 0xFF));
    }
}

void write_u16(std::vector<uint8_t>& out, uint16_t value) {
    out.push_back(static_cast<uint8_t>(value & 0xFF));
    out.push_back(static_cast<uint8_t>((value >> 8) & 0xFF));
}

void write_string(std::vector<uint8_t>& out, const std::string& value) {
    write_u32(out, static_cast<uint32_t>(value.size()));
    out.insert(out.end(), value.begin(), value.end());
}

uint32_t read_u32(const std::vector<uint8_t>& data, size_t& offset) {
    if (offset + 4 > data.size()) {
        throw DryadException("dryadc: truncated u32");
    }
    uint32_t value = 0;
    for (int i = 0; i < 4; ++i) {
        value |= static_cast<uint32_t>(data[offset + i]) << (i * 8);
    }
    offset += 4;
    return value;
}

uint16_t read_u16(const std::vector<uint8_t>& data, size_t& offset) {
    if (offset + 2 > data.size()) {
        throw DryadException("dryadc: truncated u16");
    }
    uint16_t value = static_cast<uint16_t>(data[offset]) |
                     (static_cast<uint16_t>(data[offset + 1]) << 8);
    offset += 2;
    return value;
}

std::string read_string(const std::vector<uint8_t>& data, size_t& offset) {
    const uint32_t len = read_u32(data, offset);
    if (offset + len > data.size()) {
        throw DryadException("dryadc: truncated string");
    }
    std::string value(data.begin() + offset, data.begin() + offset + len);
    offset += len;
    return value;
}

void write_constant(std::vector<uint8_t>& out, const Value& value) {
    switch (value.type()) {
        case ValueType::Null:
            out.push_back(0);
            break;
        case ValueType::Boolean:
            out.push_back(1);
            out.push_back(value.as_boolean() ? 1 : 0);
            break;
        case ValueType::Integer:
            out.push_back(2);
            for (int i = 0; i < 8; ++i) {
                const int64_t v = value.as_integer();
                out.push_back(static_cast<uint8_t>((v >> (i * 8)) & 0xFF));
            }
            break;
        case ValueType::Float:
            out.push_back(3);
            for (int i = 0; i < 8; ++i) {
                const double d = value.as_float();
                int64_t bits = 0;
                std::memcpy(&bits, &d, sizeof(double));
                out.push_back(static_cast<uint8_t>((bits >> (i * 8)) & 0xFF));
            }
            break;
        case ValueType::String:
            out.push_back(4);
            write_string(out, value.as_string());
            break;
        default:
            throw DryadException("dryadc: unsupported constant type");
    }
}

Value read_constant(const std::vector<uint8_t>& data, size_t& offset) {
    if (offset >= data.size()) {
        throw DryadException("dryadc: truncated constant tag");
    }
    const uint8_t tag = data[offset++];
    switch (tag) {
        case 0:
            return Value();
        case 1: {
            if (offset >= data.size()) {
                throw DryadException("dryadc: truncated bool constant");
            }
            return Value(data[offset++] != 0);
        }
        case 2: {
            if (offset + 8 > data.size()) {
                throw DryadException("dryadc: truncated integer constant");
            }
            int64_t value = 0;
            for (int i = 0; i < 8; ++i) {
                value |= static_cast<int64_t>(data[offset + i]) << (i * 8);
            }
            offset += 8;
            return Value(value);
        }
        case 3: {
            if (offset + 8 > data.size()) {
                throw DryadException("dryadc: truncated float constant");
            }
            int64_t bits = 0;
            for (int i = 0; i < 8; ++i) {
                bits |= static_cast<int64_t>(data[offset + i]) << (i * 8);
            }
            offset += 8;
            double value = 0.0;
            std::memcpy(&value, &bits, sizeof(double));
            return Value(value);
        }
        case 4:
            return Value(read_string(data, offset));
        default:
            throw DryadException("dryadc: unknown constant tag");
    }
}

std::vector<uint8_t> serialize_program(const CompiledProgram& program) {
    std::vector<uint8_t> out;
    out.insert(out.end(), kMagic, kMagic + 6);
    write_u32(out, kFormatVersion);
    write_u32(out, static_cast<uint32_t>(program.functions.size()));

    for (const auto& fn : program.functions) {
        write_string(out, fn.metadata.name);
        write_u32(out, static_cast<uint32_t>(fn.metadata.slots.size()));
        for (const auto& slot : fn.metadata.slots) {
            write_string(out, slot.name);
            write_u16(out, slot.slot_index);
            out.push_back(static_cast<uint8_t>(slot.type));
            out.push_back(static_cast<uint8_t>(slot.confidence));
        }
        write_u16(out, fn.metadata.stack_depth);
        write_u32(out, fn.metadata.opcode_count);
        write_u32(out, static_cast<uint32_t>(fn.module.code.size()));
        out.insert(out.end(), fn.module.code.begin(), fn.module.code.end());
        write_u32(out, static_cast<uint32_t>(fn.module.constants.size()));
        for (const auto& constant : fn.module.constants) {
            write_constant(out, constant);
        }
    }

    return out;
}

CompiledProgram deserialize_program(const std::vector<uint8_t>& data) {
    if (data.size() < 10 || std::memcmp(data.data(), kMagic, 6) != 0) {
        throw DryadException("dryadc: invalid magic header");
    }
    size_t offset = 6;
    const uint32_t version = read_u32(data, offset);
    if (version != kFormatVersion) {
        throw DryadException("dryadc: unsupported format version");
    }

    CompiledProgram program;
    const uint32_t function_count = read_u32(data, offset);
    for (uint32_t i = 0; i < function_count; ++i) {
        CompiledFunction fn;
        fn.metadata.name = read_string(data, offset);
        const uint32_t slot_count = read_u32(data, offset);
        for (uint32_t s = 0; s < slot_count; ++s) {
            SlotMetadata slot;
            slot.name = read_string(data, offset);
            slot.slot_index = read_u16(data, offset);
            slot.type = static_cast<ObservedType>(data[offset++]);
            slot.confidence = static_cast<TypeConfidence>(data[offset++]);
            fn.metadata.slots.push_back(slot);
        }
        fn.metadata.stack_depth = read_u16(data, offset);
        fn.metadata.opcode_count = read_u32(data, offset);
        const uint32_t code_len = read_u32(data, offset);
        if (offset + code_len > data.size()) {
            throw DryadException("dryadc: truncated code section");
        }
        fn.module.code.assign(data.begin() + offset, data.begin() + offset + code_len);
        offset += code_len;
        const uint32_t const_count = read_u32(data, offset);
        fn.module.constants.reserve(const_count);
        for (uint32_t c = 0; c < const_count; ++c) {
            fn.module.constants.push_back(read_constant(data, offset));
        }
        program.functions.push_back(std::move(fn));
    }
    return program;
}

} // namespace

bool save_compiled_program(const CompiledProgram& program, const std::string& path) {
    try {
        const auto bytes = serialize_program(program);
        std::ofstream file(path, std::ios::binary);
        if (!file) {
            return false;
        }
        file.write(reinterpret_cast<const char*>(bytes.data()),
                   static_cast<std::streamsize>(bytes.size()));
        return static_cast<bool>(file);
    } catch (...) {
        return false;
    }
}

bool load_compiled_program(const std::string& path, CompiledProgram& program) {
    try {
        std::ifstream file(path, std::ios::binary);
        if (!file) {
            return false;
        }
        std::vector<uint8_t> bytes((std::istreambuf_iterator<char>(file)),
                                   std::istreambuf_iterator<char>());
        program = deserialize_program(bytes);
        return true;
    } catch (...) {
        return false;
    }
}

} // namespace dryad
