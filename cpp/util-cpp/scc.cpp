#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Edge {
    int64_t from;
    int64_t to;
};

class Graph {
   public:
    explicit Graph(int64_t n) : out_edges_(n), in_edges_(n) {}

    void add_edge(int64_t from, int64_t to) {
        out_edges_[from].push_back(Edge{from, to});
        in_edges_[to].push_back(Edge{from, to});
    }

    int64_t size() const { return out_edges_.size(); }

    const vector<Edge>& out_edges(int64_t from) const {
        return out_edges_[from];
    }

    const vector<Edge>& in_edges(int64_t to) const { return in_edges_[to]; }

   private:
    vector<vector<Edge>> out_edges_;
    vector<vector<Edge>> in_edges_;
};

void dfs(const Graph& g, int64_t v, vector<bool>& visited,
         vector<int64_t>& vs) {
    visited[v] = true;

    for (const auto& edge : g.out_edges(v)) {
        if (!visited[edge.to]) {
            dfs(g, edge.to, visited, vs);
        }
    }

    vs.push_back(v);
}

void rev_dfs(const Graph& g, int64_t v, vector<bool>& visited,
             vector<int64_t>& vs) {
    visited[v] = true;
    vs.push_back(v);

    for (const auto& edge : g.in_edges(v)) {
        if (!visited[edge.from]) {
            rev_dfs(g, edge.from, visited, vs);
        }
    }
}

// 強連結成分分解
vector<vector<int64_t>> scc(const Graph& g) {
    vector<bool> visited(g.size());

    vector<int64_t> vs;
    for (auto v : irange(0L, g.size())) {
        if (!visited[v]) {
            dfs(g, v, visited, vs);
        }
    }

    visited.assign(g.size(), false);
    vector<vector<int64_t>> ret;
    for (auto v : vs | reversed) {
        if (!visited[v]) {
            ret.emplace_back();
            rev_dfs(g, v, visited, ret.back());
        }
    }

    return ret;
}

// 2-sat
class TwoSat {
   public:
    explicit TwoSat(int64_t n) : g(2 * n) {}

    void add(int64_t a, bool fa, int64_t b, bool fb) {
        g.add_edge(to_v(a, !fa), to_v(b, fb));
        g.add_edge(to_v(b, !fb), to_v(a, fa));
    }

    int64_t size() const { return g.size() / 2; }

    vector<bool> solve() const {
        const auto& components = scc(g);

        vector ret(size(), false);
        vector idx(size(), components.size());
        for (auto i : irange(0uL, components.size())) {
            for (auto v : components[i]) {
                auto t = v % size();

                if (idx[t] == i) {
                    return {};
                }

                if (idx[t] == components.size()) {
                    idx[t] = i;
                    // vの否定を立てる
                    ret[t] = (v < size());
                }
            }
        }

        return ret;
    }

   private:
    // size()より小さいとfalse, size()以上だとtrue
    int64_t to_v(int64_t a, bool f) const { return a + f * size(); }

    Graph g;
};