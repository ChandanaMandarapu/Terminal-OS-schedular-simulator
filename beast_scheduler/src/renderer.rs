use crate::scheduler::Scheduler;
use crate::memory::MemoryManager;
use crate::process::ProcessState;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

pub fn render(scheduler: &Scheduler, memory: &MemoryManager, deadlock: Option<&Vec<u32>>) {
    clear_screen();

    // Header
    println!("\x1B[1;36m╔══════════════════════════════════════════════════════════╗");
    println!("║         🦀 BEAST RUST OS SCHEDULER SIMULATOR 🦀          ║");
    println!("╚══════════════════════════════════════════════════════════╝\x1B[0m");
    println!("\x1B[33m  SYSTEM TICK: {:<10}\x1B[0m", scheduler.tick);

    // CPU
    println!("\n\x1B[1;35m[ CPU ]\x1B[0m");
    match &scheduler.current {
        Some(p) => println!(
            "  \x1B[32mRunning: PID={} | {} | Priority={} | Remaining={}t | Quantum={}/{}\x1B[0m",
            p.pid, p.name, p.priority, p.remaining_time,
            scheduler.quantum_counter, scheduler.time_quantum
        ),
        None => println!("  \x1B[90mIDLE\x1B[0m"),
    }

    // Ready Queue
    println!("\n\x1B[1;35m[ READY QUEUE (priority sorted) ]\x1B[0m");
    if scheduler.ready_queue.is_empty() {
        println!("  \x1B[90m(empty)\x1B[0m");
    }
    for p in &scheduler.ready_queue {
        println!(
            "  PID={} | {:<12} | Priority=\x1B[33m{}\x1B[0m | Remaining={}t | Waiting={}t",
            p.pid, p.name, p.priority, p.remaining_time, p.waiting_time
        );
    }

    // Terminated
    println!("\n\x1B[1;35m[ TERMINATED ]\x1B[0m");
    if scheduler.terminated.is_empty() {
        println!("  \x1B[90m(none yet)\x1B[0m");
    }
    for p in &scheduler.terminated {
        println!("  \x1B[90mPID={} | {} | DONE\x1B[0m", p.pid, p.name);
    }

    // Memory Map
    println!("\n\x1B[1;35m[ MEMORY MAP — {}KB total ]\x1B[0m", memory.total);
    print!("  ");
    for block in &memory.blocks {
        let bar_len = (block.size * 40 / memory.total).max(1);
        let bar: String = "█".repeat(bar_len);
        match block.pid {
            Some(pid) => print!("\x1B[32m{}\x1B[0m", bar), // green = used
            None => print!("\x1B[90m{}\x1B[0m", bar),      // grey = free
        }
    }
    println!();
    println!("  Fragmentation: \x1B[31m{:.1}%\x1B[0m", memory.fragmentation_percent());
    println!("  \x1B[32m█\x1B[0m = Used  \x1B[90m█\x1B[0m = Free");

    // Deadlock
    println!("\n\x1B[1;35m[ DEADLOCK DETECTOR ]\x1B[0m");
    match deadlock {
        Some(cycle) => {
            print!("  \x1B[1;31m⚠ DEADLOCK DETECTED! Cycle: ");
            for (i, pid) in cycle.iter().enumerate() {
                if i > 0 { print!(" → "); }
                print!("P{}", pid);
            }
            println!("\x1B[0m");
        }
        None => println!("  \x1B[32m✓ No deadlock detected\x1B[0m"),
    }

    println!("\n\x1B[90m  Press Ctrl+C to exit\x1B[0m");
}
