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

int main() {
    int64_t n, m;
    cin >> n >> m;

    Graph g(n);

    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;

        g.add_edge(a, b);
    }

    const auto& ans = scc(g);

    cout << ans.size() << endl;

    for (const auto& s : ans) {
        cout << s.size();

        for (auto ss : s) {
            cout << " " << ss;
        }

        cout << "\n";
    }
}