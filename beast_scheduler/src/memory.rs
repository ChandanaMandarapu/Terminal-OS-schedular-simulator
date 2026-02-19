#[derive(Clone, Debug)]
pub struct MemoryBlock {
    pub start: usize,
    pub size: usize,
    pub pid: Option<u32>, // None = free
}

pub struct MemoryManager {
    pub total: usize,
    pub blocks: Vec<MemoryBlock>,
}

impl MemoryManager {
    pub fn new(total_kb: usize) -> Self {
        MemoryManager {
            total: total_kb,
            blocks: vec![MemoryBlock { start: 0, size: total_kb, pid: None }],
        }
    }

    // First-fit allocation
    pub fn allocate(&mut self, pid: u32, size: usize) -> bool {
        let idx = self.blocks.iter().position(|b| b.pid.is_none() && b.size >= size);
        if let Some(i) = idx {
            let remaining = self.blocks[i].size - size;
            let start = self.blocks[i].start;
            self.blocks[i] = MemoryBlock { start, size, pid: Some(pid) };
            if remaining > 0 {
                self.blocks.insert(i + 1, MemoryBlock {
                    start: start + size,
                    size: remaining,
                    pid: None,
                });
            }
            true
        } else {
            false
        }
    }

    pub fn free(&mut self, pid: u32) {
        for block in &mut self.blocks {
            if block.pid == Some(pid) {
                block.pid = None;
            }
        }
        self.merge_free_blocks();
    }

    fn merge_free_blocks(&mut self) {
        let mut i = 0;
        while i + 1 < self.blocks.len() {
            if self.blocks[i].pid.is_none() && self.blocks[i + 1].pid.is_none() {
                let merged_size = self.blocks[i].size + self.blocks[i + 1].size;
                self.blocks[i].size = merged_size;
                self.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    pub fn fragmentation_percent(&self) -> f32 {
        let free: usize = self.blocks.iter().filter(|b| b.pid.is_none()).map(|b| b.size).sum();
        let free_blocks = self.blocks.iter().filter(|b| b.pid.is_none()).count();
        if free == 0 { return 0.0; }
        // Fragmentation = 1 - largest_free / total_free
        let largest = self.blocks.iter().filter(|b| b.pid.is_none()).map(|b| b.size).max().unwrap_or(0);
        if free_blocks <= 1 { 0.0 } else { (1.0 - largest as f32 / free as f32) * 100.0 }
    }
}
