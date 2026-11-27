#include "dryad/runtime/opcode.h"

namespace dryad {

const char* opcode_name(Opcode op) {
    switch (op) {
        case Opcode::PUSH_NULL:        return "PUSH_NULL";
        case Opcode::PUSH_TRUE:        return "PUSH_TRUE";
        case Opcode::PUSH_FALSE:       return "PUSH_FALSE";
        case Opcode::PUSH_INT:         return "PUSH_INT";
        case Opcode::PUSH_FLOAT:       return "PUSH_FLOAT";
        case Opcode::PUSH_CONST:       return "PUSH_CONST";
        case Opcode::POP:              return "POP";
        case Opcode::DUP:              return "DUP";
        case Opcode::SWAP:             return "SWAP";
        case Opcode::ADD:              return "ADD";
        case Opcode::SUB:              return "SUB";
        case Opcode::MUL:              return "MUL";
        case Opcode::DIV:              return "DIV";
        case Opcode::MOD:              return "MOD";
        case Opcode::ADD_F64:          return "ADD_F64";
        case Opcode::SUB_F64:          return "SUB_F64";
        case Opcode::MUL_F64:          return "MUL_F64";
        case Opcode::DIV_F64:          return "DIV_F64";
        case Opcode::ADD_I32:          return "ADD_I32";
        case Opcode::SUB_I32:          return "SUB_I32";
        case Opcode::MUL_I32:          return "MUL_I32";
        case Opcode::DIV_I32:          return "DIV_I32";
        case Opcode::NOT:              return "NOT";
        case Opcode::BIT_AND:          return "BIT_AND";
        case Opcode::BIT_OR:           return "BIT_OR";
        case Opcode::BIT_XOR:          return "BIT_XOR";
        case Opcode::BIT_NOT:          return "BIT_NOT";
        case Opcode::SHL:              return "SHL";
        case Opcode::SHR:              return "SHR";
        case Opcode::NEG:              return "NEG";
        case Opcode::EQ:               return "EQ";
        case Opcode::NEQ:              return "NEQ";
        case Opcode::LT:               return "LT";
        case Opcode::LTE:              return "LTE";
        case Opcode::GT:               return "GT";
        case Opcode::GTE:              return "GTE";
        case Opcode::JUMP:             return "JUMP";
        case Opcode::JUMP_IF_FALSE:    return "JUMP_IF_FALSE";
        case Opcode::JUMP_IF_TRUE:     return "JUMP_IF_TRUE";
        case Opcode::HALT:             return "HALT";
        case Opcode::CALL:             return "CALL";
        case Opcode::RET:              return "RET";
        case Opcode::CALL_VALUE:       return "CALL_VALUE";
        case Opcode::LOAD_FAST:        return "LOAD_FAST";
        case Opcode::STORE_FAST:       return "STORE_FAST";
        case Opcode::LOAD_GLOBAL:      return "LOAD_GLOBAL";
        case Opcode::MAKE_FUNCTION:    return "MAKE_FUNCTION";
        case Opcode::LOAD_CELL:        return "LOAD_CELL";
        case Opcode::STORE_CELL:       return "STORE_CELL";
        case Opcode::NEW_CELL:         return "NEW_CELL";
        case Opcode::LOAD_UPVALUE:     return "LOAD_UPVALUE";
        case Opcode::STORE_UPVALUE:    return "STORE_UPVALUE";
        case Opcode::LOAD_CELL_REF:    return "LOAD_CELL_REF";
        case Opcode::LOAD_UPVALUE_REF: return "LOAD_UPVALUE_REF";
        case Opcode::MAKE_CLOSURE:     return "MAKE_CLOSURE";
        case Opcode::STORE_GLOBAL:     return "STORE_GLOBAL";
        case Opcode::NEW_ARRAY:        return "NEW_ARRAY";
        case Opcode::ARRAY_PUSH:       return "ARRAY_PUSH";
        case Opcode::SET_ELEMENT:      return "SET_ELEMENT";
        case Opcode::GET_PROPERTY:     return "GET_PROPERTY";
        case Opcode::SET_PROPERTY:     return "SET_PROPERTY";
        case Opcode::METHOD_CALL:      return "METHOD_CALL";
        case Opcode::NEW_INSTANCE:     return "NEW_INSTANCE";
        case Opcode::CALL_GLOBAL:      return "CALL_GLOBAL";
        case Opcode::GET_ELEMENT:      return "GET_ELEMENT";
        case Opcode::GET_ELEMENT_F64:  return "GET_ELEMENT_F64";
        case Opcode::INTRINSIC_SYSCALL: return "INTRINSIC_SYSCALL";
    }
    return "UNKNOWN";
}

size_t opcode_size(Opcode op) {
    switch (op) {
        case Opcode::PUSH_INT:
        case Opcode::PUSH_FLOAT:
            return 9;
        case Opcode::PUSH_CONST:
        case Opcode::LOAD_FAST:
        case Opcode::STORE_FAST:
        case Opcode::LOAD_GLOBAL:
        case Opcode::STORE_GLOBAL:
        case Opcode::MAKE_FUNCTION:
        case Opcode::JUMP:
        case Opcode::JUMP_IF_FALSE:
        case Opcode::JUMP_IF_TRUE:
        case Opcode::GET_PROPERTY:
        case Opcode::SET_PROPERTY:
        case Opcode::LOAD_CELL:
        case Opcode::STORE_CELL:
        case Opcode::NEW_CELL:
        case Opcode::LOAD_UPVALUE:
        case Opcode::STORE_UPVALUE:
        case Opcode::LOAD_CELL_REF:
        case Opcode::LOAD_UPVALUE_REF:
            return 3;
        case Opcode::CALL:
        case Opcode::METHOD_CALL:
        case Opcode::NEW_INSTANCE:
        case Opcode::CALL_GLOBAL:
        case Opcode::MAKE_CLOSURE:
        case Opcode::INTRINSIC_SYSCALL:
            return 4;
        case Opcode::CALL_VALUE:
            return 2;
        default:
            return 1;
    }
}

} // namespace dryad
