use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessState::Ready      => write!(f, "READY     "),
            ProcessState::Running    => write!(f, "RUNNING   "),
            ProcessState::Waiting    => write!(f, "WAITING   "),
            ProcessState::Terminated => write!(f, "TERMINATED"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub priority: u8,        // 1 (highest) - 10 (lowest)
    pub burst_time: u32,     // total CPU time needed
    pub remaining_time: u32, // time left
    pub waiting_time: u32,   // time spent waiting (aging)
    pub state: ProcessState,
    pub memory_needed: usize, // in KB
    pub resources: Vec<u32>, // resource IDs held
    pub waiting_for: Option<u32>, // resource ID it wants
}

impl Process {
    pub fn new(pid: u32, name: &str, priority: u8, burst_time: u32, memory_needed: usize) -> Self {
        Process {
            pid,
            name: name.to_string(),
            priority,
            burst_time,
            remaining_time: burst_time,
            waiting_time: 0,
            state: ProcessState::Ready,
            memory_needed,
            resources: vec![],
            waiting_for: None,
        }
    }

    // Aging: boost priority if waiting too long
    pub fn apply_aging(&mut self) {
        if self.waiting_time > 5 && self.priority > 1 {
            self.priority -= 1;
            self.waiting_time = 0;
        }
    }
}
