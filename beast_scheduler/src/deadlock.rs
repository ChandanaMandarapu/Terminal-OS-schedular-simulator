use std::collections::{HashMap, HashSet};

pub struct ResourceAllocGraph {
    pub resources: HashMap<u32, Option<u32>>, // resource_id -> held by pid
}

impl ResourceAllocGraph {
    pub fn new(resource_ids: Vec<u32>) -> Self {
        let mut resources = HashMap::new();
        for id in resource_ids {
            resources.insert(id, None);
        }
        ResourceAllocGraph { resources }
    }

    pub fn acquire(&mut self, pid: u32, resource_id: u32) -> bool {
        if let Some(holder) = self.resources.get_mut(&resource_id) {
            if holder.is_none() {
                *holder = Some(pid);
                return true;
            }
        }
        false
    }

    pub fn release(&mut self, resource_id: u32) {
        if let Some(holder) = self.resources.get_mut(&resource_id) {
            *holder = None;
        }
    }

    // Detect cycle = deadlock using DFS
    pub fn detect_deadlock(
        &self,
        waiting_for: &HashMap<u32, u32>, // pid -> resource_id
    ) -> Option<Vec<u32>> {
        // Build wait-for graph: pid -> pid (who holds what pid wants)
        let mut wait_graph: HashMap<u32, u32> = HashMap::new();
        for (pid, res_id) in waiting_for {
            if let Some(Some(holder)) = self.resources.get(res_id) {
                if holder != pid {
                    wait_graph.insert(*pid, *holder);
                }
            }
        }

        // DFS cycle detection
        let mut visited: HashSet<u32> = HashSet::new();
        let mut rec_stack: HashSet<u32> = HashSet::new();
        let mut cycle_path: Vec<u32> = vec![];

        for &start in wait_graph.keys() {
            if Self::dfs(start, &wait_graph, &mut visited, &mut rec_stack, &mut cycle_path) {
                return Some(cycle_path);
            }
        }
        None
    }

    fn dfs(
        node: u32,
        graph: &HashMap<u32, u32>,
        visited: &mut HashSet<u32>,
        rec_stack: &mut HashSet<u32>,
        path: &mut Vec<u32>,
    ) -> bool {
        if rec_stack.contains(&node) {
            path.push(node);
            return true;
        }
        if visited.contains(&node) { return false; }
        visited.insert(node);
        rec_stack.insert(node);
        path.push(node);
        if let Some(&next) = graph.get(&node) {
            if Self::dfs(next, graph, visited, rec_stack, path) {
                return true;
            }
        }
        rec_stack.remove(&node);
        path.pop();
        false
    }
}
