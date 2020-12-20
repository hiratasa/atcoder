#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
}

struct Graph {
    out_edges: Vec<Vec<Edge>>,
    in_edges: Vec<Vec<Edge>>,
}

impl Graph {
    #[allow(dead_code)]
    fn new(n: usize) -> Graph {
        Graph {
            out_edges: vec![vec![]; n],
            in_edges: vec![vec![]; n],
        }
    }

    #[allow(dead_code)]
    fn add_edge(&mut self, from: usize, to: usize) {
        self.out_edges[from].push(Edge { from, to });
        self.in_edges[to].push(Edge { from, to });
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.out_edges.len()
    }

    #[allow(dead_code)]
    fn get_out_edges(&self, from: usize) -> &Vec<Edge> {
        &self.out_edges[from]
    }

    #[allow(dead_code)]
    fn get_in_edges(&self, to: usize) -> &Vec<Edge> {
        &self.in_edges[to]
    }
}

#[allow(dead_code)]
fn dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
    visited[v] = true;

    for edge in g.get_out_edges(v) {
        if !visited[edge.to] {
            dfs(g, edge.to, visited, vs);
        }
    }

    vs.push(v);
}

#[allow(dead_code)]
fn rev_dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
    visited[v] = true;
    vs.push(v);

    for edge in g.get_in_edges(v) {
        if !visited[edge.from] {
            rev_dfs(g, edge.from, visited, vs);
        }
    }
}

// 強連結成分分解
#[allow(dead_code)]
fn scc(g: &Graph) -> Vec<Vec<usize>> {
    let mut vs = vec![];
    {
        let mut visited = vec![false; g.size()];
        for v in 0..g.size() {
            if !visited[v] {
                dfs(g, v, &mut visited, &mut vs);
            }
        }
    }

    let mut ret = vec![];
    {
        let mut visited = vec![false; g.size()];
        for &v in vs.iter().rev() {
            if !visited[v] {
                let mut component = vec![];
                rev_dfs(g, v, &mut visited, &mut component);
                ret.push(component);
            }
        }
    }

    ret
}

// 2-sat
#[allow(dead_code)]
struct TwoSat {
    g: Graph,
}

impl TwoSat {
    #[allow(dead_code)]
    fn new(n: usize) -> TwoSat {
        TwoSat {
            g: Graph::new(2 * n),
        }
    }

    // size()より小さいとfalse, size()以上だとtrue
    #[allow(dead_code)]
    fn to_v(&self, a: usize, f: bool) -> usize {
        if f {
            a + self.size()
        } else {
            a
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, a: usize, fa: bool, b: usize, fb: bool) {
        self.g.add_edge(self.to_v(a, !fa), self.to_v(b, fb));
        self.g.add_edge(self.to_v(b, !fb), self.to_v(a, fa));
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.g.size() / 2
    }

    #[allow(dead_code)]
    fn solve(&self) -> Option<Vec<bool>> {
        let components = scc(&self.g);

        let mut ret = vec![false; self.size()];
        let mut idx = vec![components.len(); self.size()];
        for i in 0..components.len() {
            for &v in &components[i] {
                let t = v % self.size();

                if idx[t] == i {
                    // negation is already appeared in same component.
                    return None;
                }

                if idx[t] == components.len() {
                    idx[t] = i;
                    // vの否定を立てる
                    ret[t] = v < self.size();
                }
            }
        }

        Some(ret)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_scc() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1);
        g.add_edge(1, 0);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        g.add_edge(4, 5);
        g.add_edge(5, 3);

        let mut components = scc(&g);
        assert!(components.len() == 3);

        for component in components.iter_mut() {
            component.sort();
        }
        assert!(components[0] == vec![0, 1]);
        assert!(components[1] == vec![2]);
        assert!(components[2] == vec![3, 4, 5]);
    }

    #[test]
    fn test_two_sat() {
        let mut sat = TwoSat::new(3);
        sat.add(0, true, 1, true);
        sat.add(1, true, 2, true);
        sat.add(0, true, 2, false);

        assert!(sat.solve() == Some(vec![false, true, false]))
    }
}
