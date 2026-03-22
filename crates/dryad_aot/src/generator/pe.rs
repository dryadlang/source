// crates/dryad_aot/src/generator/pe.rs
//! Gerador de arquivos PE
//!
//! Gera executáveis no formato PE/COFF (Windows).

use super::Generator;
use crate::ir::IrModule;

const PE_MAGIC: &[u8; 4] = b"PE\0\0";
const MACHINE_X64: u16 = 0x8664;

/// Gerador de PE
pub struct PeGenerator {
    /// Subsystem: 1=native, 2=windows, 3=console
    subsystem: u16,
}

impl PeGenerator {
    pub fn new() -> Self {
        Self { subsystem: 3 } // CONSOLE por padrão
    }

    /// Define o subsystem
    pub fn set_subsystem(mut self, subsystem: u16) -> Self {
        self.subsystem = subsystem;
        self
    }

    /// Cria DOS header (64 bytes)
    fn create_dos_header() -> Vec<u8> {
        let mut header = vec![0u8; 64];
        header[0..2].copy_from_slice(b"MZ");
        header[60..64].copy_from_slice(&(64u32).to_le_bytes());
        header
    }

    /// Cria PE file header (20 bytes)
    fn create_file_header() -> Vec<u8> {
        let mut header = Vec::new();
        header.extend(&MACHINE_X64.to_le_bytes()); // Machine
        header.extend(&(1u16).to_le_bytes()); // NumberOfSections
        header.extend(&(0u32).to_le_bytes()); // TimeDateStamp
        header.extend(&(0u32).to_le_bytes()); // PointerToSymbolTable
        header.extend(&(0u32).to_le_bytes()); // NumberOfSymbols
        header.extend(&(224u16).to_le_bytes()); // SizeOfOptionalHeader
        header.extend(&(0x0002u16).to_le_bytes()); // Characteristics (EXECUTABLE_IMAGE)
        header
    }

    /// Cria PE optional header (mínimo)
    fn create_optional_header(code_size: u32) -> Vec<u8> {
        let mut header = Vec::new();
        header.extend(&(0x20bu16).to_le_bytes()); // Magic (PE32+)
        header.extend(&vec![0u8; 222]); // Placeholder para restante
        header
    }
}

impl Generator for PeGenerator {
    fn generate_object(&self, _module: &IrModule, code: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();

        // DOS header
        let dos_header = Self::create_dos_header();
        output.extend(&dos_header);

        // PE signature
        output.extend(PE_MAGIC);

        // File header
        let file_header = Self::create_file_header();
        output.extend(&file_header);

        // Optional header
        let opt_header = Self::create_optional_header(code.len() as u32);
        output.extend(&opt_header);

        // .text section header (40 bytes)
        output.extend(b".text\0\0\0"); // Name
        output.extend(&(code.len() as u32).to_le_bytes()); // VirtualSize
        output.extend(&(0x1000u32).to_le_bytes()); // VirtualAddress
        output.extend(&(code.len() as u32).to_le_bytes()); // SizeOfRawData
        output.extend(&(0x400u32).to_le_bytes()); // PointerToRawData
        output.extend(&[0u8; 12]); // Relocations, LineNumbers (unused)
        output.extend(&(0x60000020u32).to_le_bytes()); // Characteristics (CODE|EXECUTE|READ)

        // Code section
        output.extend(code);

        Ok(output)
    }

    fn format_name(&self) -> &'static str {
        "PE"
    }

    fn file_extension(&self) -> &'static str {
        ".exe"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::IrModule;
    use std::collections::HashMap;

    #[test]
    fn test_pe_header_validity() {
        let gen = PeGenerator::new();
        let code = vec![0x90; 100];
        let module = IrModule {
            name: "test".to_string(),
            functions: vec![],
            globals: vec![],
            metadata: HashMap::new(),
            next_register_id: 0,
            next_block_id: 0,
        };

        let pe_binary = gen
            .generate_object(&module, &code)
            .expect("PE generation failed");

        // Check MZ magic number
        assert_eq!(&pe_binary[0..2], b"MZ", "PE DOS magic mismatch");

        // Check minimum size
        assert!(pe_binary.len() >= 64, "PE binary too small");

        // Check PE signature
        assert_eq!(&pe_binary[64..68], PE_MAGIC, "PE signature mismatch");
    }
}
