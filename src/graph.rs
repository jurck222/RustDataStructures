pub struct Graph {
    num_vertices: usize,
    adj: Vec<Vec<usize>>,
}
impl Graph {
    pub fn new(v: usize) -> Graph {
        Graph {
            num_vertices: v,
            adj: vec![vec![]; v],
        }
    }
    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        self.adj[v1].push(v2);
        self.adj[v2].push(v1);
    }
    pub fn print_graph(&self) {
        println!("Adjacency list of graph:");
        for i in 0..self.num_vertices {
            println!("[{}] -> {:?}", i, self.adj[i]);
        }
    }

    pub fn dfs(&self) {
        println!("Starting DFS of the graph …");
        let mut visited = vec![false; self.num_vertices];
        for i in 0..self.num_vertices {
            if visited[i] == false {
                self.dfs_rec(i, &mut visited);
            }
        }
    }
    fn dfs_rec(&self, v: usize, visited: &mut Vec<bool>) {
        visited[v] = true;
        println!("Visited : {}", v);
        for i in &self.adj[v] {
            if visited[*i] == false {
                self.dfs_rec(*i, visited);
            }
        }
    }
}
