// crates/dryad_aot/src/backend/register_allocator.rs
//! Register allocator for x86_64
//!
//! Maps virtual registers (RegisterId) to physical x86_64 registers.
//! Uses linear scan allocation with spilling support.

use crate::ir::RegisterId;
use std::collections::{HashMap, HashSet};

/// Physical x86_64 register numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PhysicalReg {
    /// RAX - Accumulator (caller-saved, return value)
    Rax,
    /// RCX - Counter (caller-saved)
    Rcx,
    /// RDX - Data (caller-saved)
    Rdx,
    /// RSI - Source index (caller-saved, parameter 2)
    Rsi,
    /// RDI - Destination index (caller-saved, parameter 1)
    Rdi,
    /// R8 - (caller-saved, parameter 5)
    R8,
    /// R9 - (caller-saved, parameter 6)
    R9,
    /// R10 - (caller-saved)
    R10,
    /// R11 - (caller-saved)
    R11,
    /// RBX - (callee-saved - preserved by callee)
    Rbx,
    /// R12 - (callee-saved)
    R12,
    /// R13 - (callee-saved)
    R13,
    /// R14 - (callee-saved)
    R14,
    /// R15 - (callee-saved)
    R15,
}

impl PhysicalReg {
    /// Returns register encoding (0-15)
    pub fn encoding(self) -> u8 {
        match self {
            PhysicalReg::Rax => 0,
            PhysicalReg::Rcx => 1,
            PhysicalReg::Rdx => 2,
            PhysicalReg::Rsi => 3,
            PhysicalReg::Rdi => 4,
            PhysicalReg::R8 => 8,
            PhysicalReg::R9 => 9,
            PhysicalReg::R10 => 10,
            PhysicalReg::R11 => 11,
            PhysicalReg::Rbx => 3,
            PhysicalReg::R12 => 4,
            PhysicalReg::R13 => 5,
            PhysicalReg::R14 => 6,
            PhysicalReg::R15 => 7,
        }
    }

    /// Returns true if this is a callee-saved register
    pub fn is_callee_saved(self) -> bool {
        matches!(
            self,
            PhysicalReg::Rbx
                | PhysicalReg::R12
                | PhysicalReg::R13
                | PhysicalReg::R14
                | PhysicalReg::R15
        )
    }

    /// Returns true if this is a caller-saved register
    pub fn is_caller_saved(self) -> bool {
        !self.is_callee_saved()
    }

    /// All caller-saved registers available for allocation
    pub fn caller_saved_regs() -> &'static [PhysicalReg] {
        &[
            PhysicalReg::Rax,
            PhysicalReg::Rcx,
            PhysicalReg::Rdx,
            PhysicalReg::Rsi,
            PhysicalReg::Rdi,
            PhysicalReg::R8,
            PhysicalReg::R9,
            PhysicalReg::R10,
            PhysicalReg::R11,
        ]
    }

    /// All callee-saved registers (preserved across calls)
    pub fn callee_saved_regs() -> &'static [PhysicalReg] {
        &[
            PhysicalReg::Rbx,
            PhysicalReg::R12,
            PhysicalReg::R13,
            PhysicalReg::R14,
            PhysicalReg::R15,
        ]
    }

    /// All allocatable registers (excluding RSP, RBP)
    pub fn all_allocatable() -> &'static [PhysicalReg] {
        &[
            PhysicalReg::Rax,
            PhysicalReg::Rcx,
            PhysicalReg::Rdx,
            PhysicalReg::Rsi,
            PhysicalReg::Rdi,
            PhysicalReg::R8,
            PhysicalReg::R9,
            PhysicalReg::R10,
            PhysicalReg::R11,
            PhysicalReg::Rbx,
            PhysicalReg::R12,
            PhysicalReg::R13,
            PhysicalReg::R14,
            PhysicalReg::R15,
        ]
    }
}

/// Live range of a virtual register
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveRange {
    /// Virtual register ID
    pub vreg: RegisterId,
    /// Start position (instruction index)
    pub start: usize,
    /// End position (instruction index, inclusive)
    pub end: usize,
}

impl LiveRange {
    pub fn new(vreg: RegisterId, start: usize, end: usize) -> Self {
        Self { vreg, start, end }
    }

    /// Returns true if ranges overlap
    pub fn overlaps(&self, other: &LiveRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

/// Register allocation result
#[derive(Debug, Clone)]
pub struct AllocationResult {
    /// Maps virtual register to physical register (or None if spilled)
    pub alloc: HashMap<RegisterId, Option<PhysicalReg>>,
    /// Maps spilled virtual registers to stack offsets
    pub spill_offsets: HashMap<RegisterId, i32>,
    /// Total stack space needed for spills
    pub total_spill_size: i32,
}

impl AllocationResult {
    /// Get physical register for virtual register (None if spilled)
    pub fn get_phys(&self, vreg: RegisterId) -> Option<PhysicalReg> {
        self.alloc.get(&vreg).copied().flatten()
    }

    /// Get spill offset for virtual register (None if allocated to register)
    pub fn get_spill_offset(&self, vreg: RegisterId) -> Option<i32> {
        self.spill_offsets.get(&vreg).copied()
    }

    /// Returns true if register is spilled
    pub fn is_spilled(&self, vreg: RegisterId) -> bool {
        self.spill_offsets.contains_key(&vreg)
    }
}

/// Register allocator using linear scan algorithm
pub struct LinearScanAllocator;

impl LinearScanAllocator {
    /// Allocate registers for virtual registers with given live ranges
    pub fn allocate(live_ranges: &[LiveRange]) -> AllocationResult {
        let mut alloc = HashMap::new();
        let mut spill_offsets = HashMap::new();
        let mut next_spill_offset = 0i32;

        // Sort by start position (typical for linear scan)
        let mut sorted_ranges = live_ranges.to_vec();
        sorted_ranges.sort_by_key(|r| (r.start, r.end));

        // Allocate each range
        let mut active_allocations: Vec<(PhysicalReg, LiveRange)> = Vec::new();

        for range in sorted_ranges {
            active_allocations.retain(|(_, r)| r.end >= range.start);

            // Find best available physical register
            if let Some(preg) = Self::find_available_register(&active_allocations) {
                alloc.insert(range.vreg, Some(preg));
                active_allocations.push((preg, range));
            } else {
                // Spill: allocate stack space
                spill_offsets.insert(range.vreg, next_spill_offset);
                alloc.insert(range.vreg, None);
                next_spill_offset += 8; // 8 bytes per spilled register
            }
        }

        AllocationResult {
            alloc,
            spill_offsets,
            total_spill_size: next_spill_offset,
        }
    }

    /// Find an available physical register
    fn find_available_register(active: &[(PhysicalReg, LiveRange)]) -> Option<PhysicalReg> {
        // Prefer caller-saved registers for simpler code
        for preg in PhysicalReg::caller_saved_regs() {
            if active.iter().all(|(p, _)| p != preg) {
                return Some(*preg);
            }
        }

        // Fall back to callee-saved registers
        for preg in PhysicalReg::callee_saved_regs() {
            if active.iter().all(|(p, _)| p != preg) {
                return Some(*preg);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_reg_encoding() {
        assert_eq!(PhysicalReg::Rax.encoding(), 0);
        assert_eq!(PhysicalReg::Rcx.encoding(), 1);
        assert_eq!(PhysicalReg::Rdx.encoding(), 2);
        assert_eq!(PhysicalReg::Rdi.encoding(), 4);
        assert_eq!(PhysicalReg::R8.encoding(), 8);
    }

    #[test]
    fn test_callee_saved_classification() {
        assert!(PhysicalReg::Rbx.is_callee_saved());
        assert!(PhysicalReg::R12.is_callee_saved());
        assert!(!PhysicalReg::Rax.is_callee_saved());
        assert!(!PhysicalReg::Rdi.is_callee_saved());
    }

    #[test]
    fn test_live_range_overlap() {
        let r1 = LiveRange::new(0, 0, 5);
        let r2 = LiveRange::new(1, 3, 8);
        let r3 = LiveRange::new(2, 6, 10);
        let r4 = LiveRange::new(3, 10, 15);

        assert!(r1.overlaps(&r2)); // [0,5] overlaps [3,8]
        assert!(r2.overlaps(&r3)); // [3,8] overlaps [6,10]
        assert!(!r1.overlaps(&r4)); // [0,5] does not overlap [10,15]
    }

    #[test]
    fn test_allocate_single_register() {
        let ranges = vec![LiveRange::new(0, 0, 5)];
        let result = LinearScanAllocator::allocate(&ranges);

        assert!(result.get_phys(0).is_some());
        assert!(!result.is_spilled(0));
    }

    #[test]
    fn test_allocate_non_overlapping_registers() {
        let ranges = vec![
            LiveRange::new(0, 0, 5),
            LiveRange::new(1, 6, 10),
            LiveRange::new(2, 11, 15),
        ];
        let result = LinearScanAllocator::allocate(&ranges);

        assert!(result.get_phys(0).is_some());
        assert!(result.get_phys(1).is_some());
        assert!(result.get_phys(2).is_some());
        assert_eq!(result.total_spill_size, 0);
    }

    #[test]
    fn test_allocate_overlapping_requires_different_registers() {
        let ranges = vec![LiveRange::new(0, 0, 10), LiveRange::new(1, 3, 8)];
        let result = LinearScanAllocator::allocate(&ranges);

        let phys0 = result.get_phys(0);
        let phys1 = result.get_phys(1);

        // Both should be allocated
        assert!(phys0.is_some());
        assert!(phys1.is_some());

        // Should be different registers
        assert_ne!(phys0, phys1);
    }

    #[test]
    fn test_spill_when_many_overlapping() {
        // More registers than available physical registers
        let ranges = vec![
            LiveRange::new(0, 0, 20),
            LiveRange::new(1, 0, 20),
            LiveRange::new(2, 0, 20),
            LiveRange::new(3, 0, 20),
            LiveRange::new(4, 0, 20),
            LiveRange::new(5, 0, 20),
            // ... create more to force spilling
        ];

        let mut full_ranges = ranges;
        for i in 6..20 {
            full_ranges.push(LiveRange::new(i as u32, 0, 20));
        }

        let result = LinearScanAllocator::allocate(&full_ranges);

        // Some should be spilled
        let spilled_count = full_ranges
            .iter()
            .filter(|r| result.is_spilled(r.vreg))
            .count();
        assert!(spilled_count > 0);
    }

    #[test]
    fn test_allocation_result_getters() {
        let ranges = vec![LiveRange::new(0, 0, 5)];
        let result = LinearScanAllocator::allocate(&ranges);

        assert!(result.get_phys(0).is_some());
        assert_eq!(result.get_spill_offset(0), None);
        assert!(!result.is_spilled(0));
    }

    #[test]
    fn test_spill_offset_unique() {
        let ranges = vec![
            LiveRange::new(0, 0, 20),
            LiveRange::new(1, 0, 20),
            LiveRange::new(2, 0, 20),
            LiveRange::new(3, 0, 20),
            LiveRange::new(4, 0, 20),
            LiveRange::new(5, 0, 20),
            LiveRange::new(6, 0, 20),
            LiveRange::new(7, 0, 20),
            LiveRange::new(8, 0, 20),
            LiveRange::new(9, 0, 20),
            LiveRange::new(10, 0, 20),
            LiveRange::new(11, 0, 20),
            LiveRange::new(12, 0, 20),
            LiveRange::new(13, 0, 20),
            LiveRange::new(14, 0, 20),
            LiveRange::new(15, 0, 20),
        ];

        let result = LinearScanAllocator::allocate(&ranges);

        let spilled: Vec<_> = (0..16).filter(|i| result.is_spilled(*i)).collect();

        if spilled.len() > 1 {
            // All spilled offsets should be unique
            let offsets: Vec<_> = spilled
                .iter()
                .map(|i| result.get_spill_offset(*i))
                .collect();
            assert_eq!(offsets.len(), offsets.iter().collect::<HashSet<_>>().len());
        }
    }
}
