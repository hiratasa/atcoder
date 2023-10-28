use super::graph::{Edge, Graph};

// dinic法
// 計算量
//  * 一般に、O((見つかったOD間のパスの長さ合計) + (BFSの回数)*E)
//      ここでOD間のパスが見つかる回数は、BFS1回あたり最大で O(E) 回である.
//      1本のパスの長さは高々V, BFSの回数は高々V回なので、O(EV^2) が得られる.
//  * 全ての非零の辺容量が同一のとき, O(EV), また O(E^(3/2))
//     proof)
//      辺容量を1とする.
//      パスが見つかるとそのうえの辺は全て飽和するので、BFS1回あたりのパスの長さ合計はO(E).
//      BFSの回数は高々V回なので、O(EV).
//      さらに、フローをf流した時点で残余グラフに流せるフローを考える.
//      これ以降の経路長がf+1以上であることと、残余グラフの容量合計から、これはO(E/f)で抑えられる.
//      よって全体のフローFは最大で f+E/f <= E^(1/2)
//      BFSの回数もこれで抑えられるので、O(E^(3/2))が得られる.
//  * 全ての非零の辺容量が同一かつ多重辺がないとき, O(EV^(2/3))
//     proof)
//      同様に、BFSを何回か流した後の残余グラフを考える.
//      残余グラフ上で出発地からの距離dごとに頂点を分類し、V_dとする.
//      V_dからV_{d+1}に向かう辺の集合はカットであり、
//      また多重辺がないのでこのカットのサイズは |V_d|*|V_{d+1}| で抑えられる.
//      (正確には残余グラフには多重辺がありうるが高々2本なのでok)
//      最大フロー-最小カットの定理よりこのグラフ上に流せるフローをFとして、|V_d|*|V_{d+1}| >=F.
//      よって、|V_d| と |V_{d+1}| のいずれかは F^(1/2)以上であり、
//      d=0,1,...,D (Dはs-t距離)に対する |V_d| のうち過半数は F^(1/2) 以上.
//        (D/2) * F^(1/2) <= Σ[0<=d<=D] |V_d| = V
//      これより、 F <= (2V/D)^2
//      全体のBFSの回数を考えると、
//        (ここまでのBFS回数) + (これより後のBFS回数) <= D + (2V/D)^2 <= O(V^(2/3))
//       よって、全体で O(EV^(2/3)) が言える.
//  * 全ての非零の辺容量が同一かつ全ての頂点で入次数もしくは出次数が1のとき, O(EV^(1/2))
//     (e.g. 二部マッチング)
//     proof)
//      同様に、BFSを何回か流した後の残余グラフを考える.
//      残余グラフ上のフローを考えると、各頂点は高々1回しか通れない.
//      よってこれ以降の経路長をD以上として、残余グラフの最大フローは O(V/D) で抑えられる.
//      全体のBFSの回数を考えると、
//        (ここまでのBFS回数) + (これより後のBFS回数) <= D + V/D <= O(V^(1/2))
//       よって、全体で O(EV^(1/2)) が言える.
//
//  * もう少し特殊なケース.
//    始点からの辺の容量はばらばら、それ以外の辺の容量は全て同一のとき.
//    パスが見つかる度に始点からの辺か終点への辺のどちらかが飽和して、それらは（それ以降のBFSでも）使えない.
//    （始点から/終点への辺の逆辺は使われないため）
//    よって、(パスが見つかる回数) <= E であり、
//      O((見つかったOD間のパスの長さ合計) + (BFSの回数)*E) <= O(EV)

#[allow(dead_code)]
struct MaxFlowGraph {
    // label is edge index
    g: Graph<usize>,
    caps: Vec<usize>,
    rev: Vec<usize>,
}

#[allow(dead_code)]
impl MaxFlowGraph {
    fn new(n: usize) -> MaxFlowGraph {
        MaxFlowGraph {
            g: Graph::new(n),
            caps: vec![],
            rev: vec![],
        }
    }

    fn num_vertices(&self) -> usize {
        self.g.size()
    }

    fn num_edges(&self) -> usize {
        self.caps.len()
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let idx = self.num_edges();
        let rev_idx = self.num_edges() + 1;

        self.g.add_edge(Edge::new_with_label(from, to, idx));
        self.g.add_edge(Edge::new_with_label(to, from, rev_idx));

        // forward edge
        self.caps.push(cap);
        self.rev.push(rev_idx);

        // backward edge
        self.caps.push(0);
        self.rev.push(idx);
    }

    fn bfs(&self, src: usize, dst: usize) -> Option<Vec<usize>> {
        fn chmin(a: &mut usize, b: usize) -> bool {
            if *a > b {
                *a = b;
                true
            } else {
                false
            }
        }

        let mut q = std::collections::VecDeque::new();
        let mut costs = vec![std::usize::MAX; self.num_vertices()];

        q.push_back(src);
        costs[src] = 0;

        while let Some(v) = q.pop_front() {
            if v == dst {
                return Some(costs);
            }

            let c = costs[v];
            self.g.out_edges[v]
                .iter()
                .filter(|e| self.caps[e.label] > 0)
                .filter(|e| chmin(&mut costs[e.to], c + 1))
                .for_each(|e| q.push_back(e.to));
        }

        None
    }

    fn dfs(
        &mut self,
        src: usize,
        dst: usize,
        upper: usize,
        levels: &Vec<usize>,
        itrs: &mut Vec<usize>,
    ) -> usize {
        if src == dst {
            return upper;
        }

        let mut total_flow = 0;
        for i in itrs[src]..self.g.out_edges[src].len() {
            let e = self.g.out_edges[src][i];
            if levels[src] + 1 == levels[e.to] && self.caps[e.label] > 0 {
                let flow = self.dfs(
                    e.to,
                    dst,
                    (upper - total_flow).min(self.caps[e.label]),
                    levels,
                    itrs,
                );

                self.caps[e.label] -= flow;
                self.caps[self.rev[e.label]] += flow;

                total_flow += flow;
                if upper == total_flow {
                    // NOTE:
                    //  この場合はitrs[src]はインクリメントしないこと！
                    //  (この辺に沿ってまだ流せるかもしれない)
                    return total_flow;
                }
            }
            itrs[src] += 1;
        }

        total_flow
    }

    fn max_flow(&mut self, src: usize, dst: usize) -> usize {
        let mut total_flow = 0;
        loop {
            if let Some(levels) = self.bfs(src, dst) {
                // ここでは一回のdfsで流せるだけ流しきる方式を採用しているので、
                // 複数回呼ぶ必要はない
                total_flow += self.dfs(
                    src,
                    dst,
                    std::usize::MAX,
                    &levels,
                    &mut vec![0; self.num_vertices()],
                );
            } else {
                break;
            }
        }

        total_flow
    }
}

// 全ての頂点を組Aと組Bに分類したときのコストの最小値を求める
struct ProjectSelectionProblem {
    g: MaxFlowGraph,
    cost_offset: i64,
    src: usize,
    dst: usize,
}

#[allow(dead_code)]
impl ProjectSelectionProblem {
    fn new(n: usize) -> ProjectSelectionProblem {
        ProjectSelectionProblem {
            g: MaxFlowGraph::new(n + 2),
            cost_offset: 0,
            src: n,
            dst: n + 1,
        }
    }

    // 各頂点を各組に割り振った時のコストを設定（負も許容）
    // 同一頂点に対しては高々1回だけ呼ぶこと
    fn set_cost<C: TryInto<i64>>(&mut self, idx: usize, cost_a: C, cost_b: C) {
        let cost_a = cost_a.try_into().ok().unwrap();
        let cost_b = cost_b.try_into().ok().unwrap();

        let offset = std::cmp::max(-cost_a, -cost_b).max(0);

        // コストをどちら側にのせるか注意
        self.g.add_edge(self.src, idx, (cost_b + offset) as usize);
        self.g.add_edge(idx, self.dst, (cost_a + offset) as usize);

        self.cost_offset += offset;
    }

    // idx番目の頂点は組which(=0 or 1)に属するべきである
    fn should(&mut self, idx: usize, which: usize) {
        if which == 0 {
            self.set_cost(idx, 0, 1i64 << 60);
        } else if which == 1 {
            self.set_cost(idx, 1i64 << 60, 0);
        } else {
            unreachable!();
        }
    }

    // (idx0が組A) ⇒ (idx1が組A) の形の条件
    fn add_implication_relation(&mut self, idx0: usize, idx1: usize) {
        self.add_implication_relation_penalty(idx0, idx1, std::usize::MAX);
    }

    // (idx0が組A) ⇒ (idx1が組A) の形の条件. 破ると正のコストがかかる
    fn add_implication_relation_penalty(&mut self, idx0: usize, idx1: usize, cost: usize) {
        self.g.add_edge(idx0, idx1, cost);
    }

    // idx0とidx1が同じ組に属する の形の条件. 守ると褒章（負のコスト）がある
    fn add_reward_equality_relation(&mut self, idx0: usize, idx1: usize, reward: usize) {
        self.cost_offset += reward as i64;
        // (idx0がA) => (idx1がA)
        self.add_implication_relation_penalty(idx0, idx1, reward);
        // (idx1がA) => (idx0がA)
        self.add_implication_relation_penalty(idx1, idx0, reward);
    }

    // 最小コストを求める
    fn calc_min_cost(&mut self) -> i64 {
        let f = self.g.max_flow(self.src, self.dst);

        f as i64 - self.cost_offset
    }
}
