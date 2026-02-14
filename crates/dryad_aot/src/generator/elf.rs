// crates/dryad_aot/src/generator/elf.rs
//! Gerador de arquivos ELF
//!
//! Gera arquivos objeto e executáveis no formato ELF (Linux, BSDs).

use super::Generator;
use crate::ir::IrModule;

/// Gerador de ELF
pub struct ElfGenerator {
    /// Se é executável (true) ou objeto (false)
    is_executable: bool,
    
    /// Arquitetura
    machine: u16,
}

impl ElfGenerator {
    pub fn new() -> Self {
        Self {
            is_executable: true,
            machine: 0x3E, // EM_X86_64
        }
    }
    
    /// Define como gerar objeto ao invés de executável
    pub fn set_object(mut self) -> Self {
        self.is_executable = false;
        self
    }
    
    /// Define a arquitetura
    pub fn set_machine(mut self, machine: u16) -> Self {
        self.machine = machine;
        self
    }
    
    /// Gera o ELF header
    fn generate_elf_header(&self, entry_point: u64, ph_offset: u64, ph_count: u16) -> Vec<u8> {
        let mut header = Vec::with_capacity(64);
        
        // e_ident[16]
        header.push(0x7f); // ELFMAG0
        header.push(b'E'); // ELFMAG1
        header.push(b'L'); // ELFMAG2
        header.push(b'F'); // ELFMAG3
        header.push(2);    // ELFCLASS64
        header.push(1);    // ELFDATA2LSB (Little Endian)
        header.push(1);    // EV_CURRENT
        header.push(0);    // ELFOSABI_NONE
        header.extend(&[0; 8]); // Padding
        
        // e_type: ET_EXEC (2) ou ET_REL (1)
        let e_type = if self.is_executable { 2u16 } else { 1u16 };
        header.extend(&e_type.to_le_bytes());
        
        // e_machine
        header.extend(&self.machine.to_le_bytes());
        
        // e_version
        header.extend(&1u32.to_le_bytes());
        
        // e_entry
        header.extend(&entry_point.to_le_bytes());
        
        // e_phoff
        header.extend(&ph_offset.to_le_bytes());
        
        // e_shoff (0 = sem section headers)
        header.extend(&0u64.to_le_bytes());
        
        // e_flags
        header.extend(&0u32.to_le_bytes());
        
        // e_ehsize
        header.extend(&64u16.to_le_bytes());
        
        // e_phentsize
        header.extend(&56u16.to_le_bytes());
        
        // e_phnum
        header.extend(&ph_count.to_le_bytes());
        
        // e_shentsize
        header.extend(&64u16.to_le_bytes());
        
        // e_shnum
        header.extend(&0u16.to_le_bytes());
        
        // e_shstrndx
        header.extend(&0u16.to_le_bytes());
        
        header
    }
    
    /// Gera um program header
    fn generate_program_header(
        &self,
        p_type: u32,
        p_flags: u32,
        p_offset: u64,
        p_vaddr: u64,
        p_filesz: u64,
        p_memsz: u64,
    ) -> Vec<u8> {
        let mut ph = Vec::with_capacity(56);
        
        // p_type
        ph.extend(&p_type.to_le_bytes());
        
        // p_flags
        ph.extend(&p_flags.to_le_bytes());
        
        // p_offset
        ph.extend(&p_offset.to_le_bytes());
        
        // p_vaddr
        ph.extend(&p_vaddr.to_le_bytes());
        
        // p_paddr
        ph.extend(&p_vaddr.to_le_bytes()); // Igual a vaddr
        
        // p_filesz
        ph.extend(&p_filesz.to_le_bytes());
        
        // p_memsz
        ph.extend(&p_memsz.to_le_bytes());
        
        // p_align
        ph.extend(&0x1000u64.to_le_bytes());
        
        ph
    }
}

impl Generator for ElfGenerator {
    fn generate_object(&self, _module: &IrModule, code: &[u8]) -> Result<Vec<u8>, String> {
        // ELF mínimo executável
        let entry_point = 0x400000 + 64 + 56; // Base + ELF header + 1 PHDR
        let ph_offset = 64; // Depois do ELF header
        
        // Gerar ELF header
        let elf_header = self.generate_elf_header(entry_point, ph_offset, 1);
        
        // Gerar program header para o código
        let code_offset = 64 + 56; // Depois dos headers
        let ph = self.generate_program_header(
            1,           // PT_LOAD
            5,           // PF_R | PF_X
            code_offset,
            entry_point,
            code.len() as u64,
            code.len() as u64,
        );
        
        // Montar arquivo
        let mut elf = Vec::new();
        elf.extend(elf_header);
        elf.extend(ph);
        elf.extend(code);
        
        // Alinhar para 4KB
        while elf.len() % 4096 != 0 {
            elf.push(0);
        }
        
        Ok(elf)
    }
    
    fn format_name(&self) -> &'static str {
        "ELF"
    }
    
    fn file_extension(&self) -> &'static str {
        ""
    }
}
