use std::collections::{HashMap, VecDeque};

pub struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>,
    directed: bool,
}

pub struct BfsFunc<'a> {
    vertex_processing_early: Box<dyn FnMut(i32) + 'a>,
    vertex_processing_late: Box<dyn FnMut(i32) + 'a>,
    edge_processing_in_tree: Box<dyn FnMut(i32, i32) + 'a>,
    edge_processing_out_tree: Box<dyn FnMut(i32, i32) + 'a>,
}

impl<'a> BfsFunc<'a> {
    pub fn new() -> BfsFunc<'a> {
        BfsFunc {
            vertex_processing_early: Box::new(|_| {}),
            vertex_processing_late: Box::new(|_| {}),
            edge_processing_in_tree: Box::new(|_, _| {}),
            edge_processing_out_tree: Box::new(|_, _| {}),
        }
    }
}

enum BfsState {
    Undiscovered,
    Discovered,
    Processed,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adjacency_list: HashMap::new(),
            directed: true,
        }
    }

    pub fn add_vertex(&mut self, id: i32) {
        if !self.adjacency_list.contains_key(&id) {
            self.adjacency_list.insert(id, vec![]);
        }
    }

    pub fn add_edge(&mut self, from: i32, to: i32) -> Result<(), &str> {
        if self.adjacency_list.contains_key(&from) {
            if self.adjacency_list.contains_key(&to) {
                let list = self.adjacency_list.get_mut(&from).unwrap();
                list.push(to);
                if self.directed {
                    let list = self.adjacency_list.get_mut(&to).unwrap();
                    list.push(from);
                }
                return Ok(());
            }
            return Err("\"to\" vertex doesn't exist in the graph");
        }
        return Err("\"from\" vertex doesn't exist in the graph");
    }

    pub fn bfs(&self, root: i32, mut bfs_functions: BfsFunc) -> Result<(), &str> {
        if !self.adjacency_list.contains_key(&root) {
            return Err("Root vertex doesn't exist in the graph");
        }

        let mut states = HashMap::new();
        self.bfs_init(&mut states);

        states.insert(root, BfsState::Discovered);
        let mut for_processing = VecDeque::new();
        for_processing.push_back(root);

        while for_processing.len() != 0 {
            let curr_vertex = for_processing.pop_front().unwrap();
            (bfs_functions.vertex_processing_early)(curr_vertex);
            states.insert(curr_vertex, BfsState::Processed);
            let al = self.adjacency_list.get(&curr_vertex).unwrap();
            for &adj_vertex in al {
                match states.get(&adj_vertex).unwrap() {
                    BfsState::Undiscovered => {
                        for_processing.push_back(adj_vertex);
                        states.insert(adj_vertex, BfsState::Discovered);
                        (bfs_functions.edge_processing_in_tree)(curr_vertex, adj_vertex);
                    }
                    BfsState::Discovered => {
                        (bfs_functions.edge_processing_out_tree)(curr_vertex, adj_vertex);
                    }
                    BfsState::Processed => {}
                }
            }
            (bfs_functions.vertex_processing_late)(curr_vertex);
        }
        Ok(())
    }

    fn bfs_init(&self, states: &mut HashMap<i32, BfsState>) {
        for id in self.adjacency_list.keys() {
            states.insert(*id, BfsState::Undiscovered);
        }
    }

    pub fn furthest_from_root(&self, root: i32) -> Result<(i32, i32), &str> {
        if !self.adjacency_list.contains_key(&root) {
            return Err("Root vertex doesn't exist in the graph");
        }

        let mut furthest_vertex = root;
        let mut max_height = 0;
        let mut heights: HashMap<i32, i32> = HashMap::new();
        heights.insert(root, 1);
        let mut bfs_functions = BfsFunc::new();
        let f = |from, to| {
            let parent_height = *heights.get(&from).unwrap();
            heights.insert(to, parent_height + 1);
            if parent_height + 1 > max_height {
                furthest_vertex = to;
                max_height = parent_height + 1;
            }
        };
        bfs_functions.edge_processing_in_tree = Box::new(f);
        self.bfs(root, bfs_functions).unwrap();

        Ok((furthest_vertex, max_height))
    }
}

struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // Initialize graph
        let mut g = Graph::new();
        for id in 0..n {
            g.add_vertex(id);
        }
        for edge in edges {
            g.add_edge(edge[0], edge[1]).unwrap();
        }
        // Find two leaves furthest from each other
        let (leaf_vertex_1, _) = g.furthest_from_root(0).unwrap();
        let (leaf_vertex_2, graph_diameter) = g.furthest_from_root(leaf_vertex_1).unwrap();
        // Perform BFS from leaf 1 to leaf 2 to create parents map
        let mut parents: HashMap<i32, i32> = HashMap::new();
        let mut bfs_functions = BfsFunc::new();
        let f = |from, to| {
            parents.insert(to, from);
        };
        bfs_functions.edge_processing_in_tree = Box::new(f);
        g.bfs(leaf_vertex_1, bfs_functions).unwrap();
        // Find center point between the two leaves
        let mut result = vec![];
        let min_height = graph_diameter / 2;
        let mut vertex = leaf_vertex_2;
        for i in 0..=min_height {
            if graph_diameter % 2 == 0 && i == min_height - 1 {
                result.push(vertex);
            } else if i == min_height {
                result.push(vertex);
            }
            vertex = *parents.get(&vertex).unwrap();
        }
        result
    }
}

fn main() {
    let n = 4;
    let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
    let result = Solution::find_min_height_trees(n, edges);
    assert_eq!(result, vec![1]);
    let n = 6;
    let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
    let result = Solution::find_min_height_trees(n, edges);
    assert_eq!(result, vec![3, 4]);
    let n = 3;
    let edges = vec![vec![0, 1], vec![0, 2]];
    let result = Solution::find_min_height_trees(n, edges);
    assert_eq!(result, vec![0]);
}
