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
        case Opcode::LOAD_FAST:        return "LOAD_FAST";
        case Opcode::STORE_FAST:       return "STORE_FAST";
        case Opcode::GET_ELEMENT:      return "GET_ELEMENT";
        case Opcode::GET_ELEMENT_F64:  return "GET_ELEMENT_F64";
        case Opcode::INTRINSIC_SYSCALL: return "INTRINSIC_SYSCALL";
    }
    return "UNKNOWN";
}

} // namespace dryad
