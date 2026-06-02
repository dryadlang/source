use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::value::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionMode {
    Running,
    Stepping,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugCommand {
    SetBreakpoints { file: String, lines: Vec<usize> },
    Continue,
    Step,
    Pause,
    GetVariables,
    GetHeap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugEvent {
    BreakpointHit { file: String, line: usize },
    StepComplete { file: String, line: usize },
    Paused,
    Variables(HashMap<String, String>),
    Heap(Vec<String>),
    Error(String),
}

pub struct DebugState {
    pub breakpoints: HashSet<(String, usize)>,
    pub execution_mode: ExecutionMode,
    pub last_location: (String, usize),
    pub command_queue: Vec<DebugCommand>,
    pub event_queue: Vec<DebugEvent>,
}

impl DebugState {
    pub fn new() -> Self {
        Self {
            breakpoints: HashSet::new(),
            execution_mode: ExecutionMode::Running,
            last_location: (String::new(), 0),
            command_queue: Vec::new(),
            event_queue: Vec::new(),
        }
    }

    pub fn set_breakpoints(&mut self, file: String, lines: Vec<usize>) {
        // Clear old breakpoints for this file (simple implementation)
        self.breakpoints.retain(|(f, _)| f != &file);
        for line in lines {
            self.breakpoints.insert((file.clone(), line));
        }
    }

    pub fn should_pause(&self, file: &str, line: usize) -> bool {
        match self.execution_mode {
            ExecutionMode::Paused => true,
            ExecutionMode::Stepping => true,
            ExecutionMode::Running => self.breakpoints.contains(&(file.to_string(), line)),
        }
    }
}

pub type SharedDebugState = Arc<Mutex<DebugState>>;
