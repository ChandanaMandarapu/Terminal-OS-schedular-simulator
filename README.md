# 🦀 Beast Rust OS Scheduler Simulator

A terminal based operating system scheduler simulator written in Rust.

This project simulates how an operating system handles process scheduling, memory allocation and deadlock detection in real time.

Everything is shown live in the terminal with colors and continuous updates.

---

## What this project does

This simulator demonstrates core operating system concepts:

Process scheduling using priority and round robin  
Memory management using first fit allocation  
Memory fragmentation calculation  
Deadlock detection using resource allocation graph  
Priority aging to prevent starvation  
Live terminal rendering of system state  

The system runs in a loop and updates every few milliseconds to simulate real CPU ticks.

---

## Concepts implemented

### Process management

Each process has:

Process ID  
Name  
Priority level  
Burst time  
Remaining execution time  
Waiting time  
Memory requirement  
State like Ready Running Waiting or Terminated  

Priority aging is applied when a process waits too long.

---

### Scheduler

The scheduler uses:

Priority based sorting  
Round robin with time quantum  
Preemption  
Automatic termination when burst time finishes  

Higher priority processes run first.  
Lower priority processes eventually get boosted due to aging.

---

### Memory management

Total memory is simulated as 256 KB.

First fit allocation strategy is used.

When a process terminates, memory is freed and adjacent free blocks are merged.

Fragmentation percentage is calculated and displayed.

---

### Deadlock detection

A resource allocation graph is built.

Cycles are detected using depth first search.

If a cycle is found, deadlock is detected and shown in red in the terminal.

---

## Terminal output

Add your screenshot below.

Save your image as output.png in the project root.

![Simulator Output](Screenshot (480).png)
Screenshot (481).png

---

## How to run

Make sure Rust is installed.

Clone the repository.

Open terminal inside the project folder.

Run:

cargo run

The simulator will start automatically.

Press Ctrl C to stop.

---

## Why this project matters

This project demonstrates understanding of:

Operating system scheduling  
Memory allocation strategies  
Deadlock detection algorithms  
Terminal rendering  
Rust ownership and struct modeling  

It is a strong systems programming portfolio project.

---

Built with Rust ❤️
