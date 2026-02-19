mod process;
mod scheduler;
mod memory;
mod deadlock;
mod renderer;

use process::Process;
use scheduler::Scheduler;
use memory::MemoryManager;
use deadlock::ResourceAllocGraph;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let mut scheduler = Scheduler::new(4); // time quantum = 4
    let mut memory = MemoryManager::new(256); // 256 KB RAM
    let mut rag = ResourceAllocGraph::new(vec![1, 2, 3, 4]);

    // Spawn some processes
    let processes = vec![
        Process::new(1, "SystemInit",  1, 20, 32),
        Process::new(2, "NetworkDaemon", 3, 15, 16),
        Process::new(3, "UserShell",   5, 10, 8),
        Process::new(4, "FileSystem",  2, 25, 48),
        Process::new(5, "CryptoMiner", 8, 30, 64),
        Process::new(6, "Logger",      6, 12, 8),
    ];

    for p in &processes {
        memory.allocate(p.pid, p.memory_needed);
    }

    // Simulate a deadlock scenario: P1 holds R1, wants R2; P2 holds R2, wants R1
    rag.acquire(1, 1);
    rag.acquire(2, 2);
    let mut waiting_for: HashMap<u32, u32> = HashMap::new();
    waiting_for.insert(1, 2); // P1 wants R2
    waiting_for.insert(2, 1); // P2 wants R1

    for p in processes {
        scheduler.add_process(p);
    }

    loop {
        scheduler.tick();

        // Free memory when process terminates
        for p in &scheduler.terminated {
            memory.free(p.pid);
        }

        let deadlock = rag.detect_deadlock(&waiting_for);

        renderer::render(&scheduler, &memory, deadlock.as_ref());

        // Spawn new random-ish process every 15 ticks to keep it alive
        if scheduler.tick % 15 == 0 && scheduler.tick < 200 {
            let pid = 100 + scheduler.tick;
            let p = Process::new(pid, "NewTask", (scheduler.tick % 9 + 1) as u8, 8, 4);
            memory.allocate(pid, 4);
            scheduler.add_process(p);
        }

        if scheduler.ready_queue.is_empty() && scheduler.current.is_none() && scheduler.tick > 10 {
            println!("\n\x1B[1;32mAll processes completed. System idle.\x1B[0m");
            break;
        }

        thread::sleep(Duration::from_millis(300));
    }
}
