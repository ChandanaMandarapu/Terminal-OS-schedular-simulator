use crate::process::{Process, ProcessState};
use std::collections::VecDeque;

pub struct Scheduler {
    pub ready_queue: VecDeque<Process>,
    pub terminated: Vec<Process>,
    pub current: Option<Process>,
    pub time_quantum: u32,
    pub tick: u32,
    pub quantum_counter: u32,
}

impl Scheduler {
    pub fn new(quantum: u32) -> Self {
        Scheduler {
            ready_queue: VecDeque::new(),
            terminated: vec![],
            current: None,
            time_quantum: quantum,
            tick: 0,
            quantum_counter: 0,
        }
    }

    pub fn add_process(&mut self, p: Process) {
        self.ready_queue.push_back(p);
    }

    pub fn tick(&mut self) {
        self.tick += 1;

        // Age waiting processes
        for p in &mut self.ready_queue {
            p.waiting_time += 1;
            p.apply_aging();
        }

        // Sort by priority (lower number = higher priority)
        let mut sorted: Vec<Process> = self.ready_queue.drain(..).collect();
        sorted.sort_by_key(|p| p.priority);
        self.ready_queue = sorted.into();

        if let Some(ref mut current) = self.current {
            current.remaining_time = current.remaining_time.saturating_sub(1);
            self.quantum_counter += 1;

            if current.remaining_time == 0 {
                let mut done = self.current.take().unwrap();
                done.state = ProcessState::Terminated;
                self.terminated.push(done);
                self.quantum_counter = 0;
            } else if self.quantum_counter >= self.time_quantum {
                let mut preempted = self.current.take().unwrap();
                preempted.state = ProcessState::Ready;
                self.ready_queue.push_back(preempted);
                self.quantum_counter = 0;
            }
        }

        if self.current.is_none() {
            if let Some(mut next) = self.ready_queue.pop_front() {
                next.state = ProcessState::Running;
                self.current = Some(next);
                self.quantum_counter = 0;
            }
        }
    }
}
