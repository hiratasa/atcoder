#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Edge {
    int64_t u;
    int64_t cap;
    int64_t cost;
    int64_t rev;
};

class Graph {
   public:
    explicit Graph(int64_t n) : n(n), edges(n) {}

    void add_edge(int64_t v, int64_t u, int64_t cap, int64_t cost) {
        assert(cost >= 0);
        edges[v].push_back(Edge{u, cap, cost, (int64_t)edges[u].size()});
        edges[u].push_back(Edge{v, 0, -cost, (int64_t)edges[v].size() - 1});
    }

    int64_t min_cost_flow(int64_t s, int64_t t, int64_t f) {
        int64_t total_cost = 0;

        while (f > 0) {
            vector<int64_t> costs(n, numeric_limits<int64_t>::max());
            vector<Edge*> parents(n, nullptr);
            vector<int64_t> h(n, 0L);
            priority_queue<pair<int64_t, int64_t>> q;

            costs[s] = 0;
            q.emplace(0L, s);

            while (!q.empty()) {
                auto [cost, v] = q.top();
                cost = -cost;
                q.pop();

                if (cost > costs[v]) {
                    continue;
                }

                for (auto&& edge : edges[v]) {
                    if (edge.cap == 0) {
                        continue;
                    }

                    auto new_cost = cost + edge.cost + h[v] - h[edge.u];

                    if (new_cost < costs[edge.u]) {
                        costs[edge.u] = new_cost;
                        parents[edge.u] = &edge;
                        q.emplace(-new_cost, edge.u);
                    }
                }
            }

            if (costs[t] == numeric_limits<int64_t>::max()) {
                return -1;
            }

            for (auto v : irange(0L, n)) {
                h[v] += costs[v];
            }

            total_cost += h[t];

            int64_t u = t;
            while (parents[u] != nullptr) {
                auto* edge = parents[u];
                auto* rev_edge = &edges[u][edge->rev];

                --edge->cap;
                ++rev_edge->cap;

                u = rev_edge->u;
            }
            --f;
        }

        return total_cost;
    }

   private:
    // num vertex
    int64_t n;
    vector<vector<Edge>> edges;
};

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector mat(n, vector<bool>(m));
    vector<pair<int64_t, int64_t>> idxs;
    for (auto i : irange(0L, n)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, m)) {
            if (s[j] == '#') {
                continue;
            }

            mat[i][j] = true;
            if (s[j] == 'o') {
                idxs.emplace_back(i, j);
            }
        }
    }

    Graph g(n * m + 2);
    int64_t src = n * m, dst = n * m + 1;

    auto IDX = [&](int64_t i, int64_t j) { return i * m + j; };

    for (auto [i, j] : idxs) {
        g.add_edge(src, IDX(i, j), 1, i + j);
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m)) {
            if (mat[i][j]) {
                g.add_edge(IDX(i, j), dst, 1, n + m - i - j);
            }
        }
    }

    for (auto i : irange(0L, n - 1)) {
        for (auto j : irange(0L, m)) {
            if (mat[i][j] && mat[i + 1][j]) {
                g.add_edge(IDX(i, j), IDX(i + 1, j), idxs.size(), 0);
            }
        }
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m - 1)) {
            if (mat[i][j] && mat[i][j + 1]) {
                g.add_edge(IDX(i, j), IDX(i, j + 1), idxs.size(), 0);
            }
        }
    }

    auto c = g.min_cost_flow(src, dst, idxs.size());

    assert(c >= 0);

    cout << -c + (n + m) * idxs.size() << endl;
}