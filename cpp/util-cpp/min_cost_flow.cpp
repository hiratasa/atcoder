#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

// clang-format off
// 流れ
// 1. コスト関数が負のコストを含む場合は全て0以上になるように補正する
//    これはたとえば以下のようにすれば実現できる:
//   1a. ベルマンフォード法を使って h0[v] = 出発地からの距離を求め、
//       d'(v=>u) := d(v=>u) + h0[v] - h0[u] とする(これは0以上になる)
//   1b. その他の方法でも0以上いなればOK
//       (ACL1 C問題では各辺のコストを-1としてhをマンハッタン距離を負にしたものとすると、
//        解説と同じ形になる？)
//   注: 上記のh式を使うと、s-tコストはh0[s]-h0[t]だけ増える
//       この対処のために、以下のいずれかを行う:
//        a) 最後に flow * (h0[s] - h0[t]) を引く
//        b) min_cost_flow()関数の中のhの初期値としてh0を使い、
//           total_costに足し上げるときに h0[s]を引く
// 2. コストとして d(v=>u) + h[v] - h[u] を使ってダイクストラ法を行う
// 3. h[v] += costs[v] で更新;
//    ここでcosts[v]=true_costs[v]+h[s]-h[v]なので、これをh[v]に足すとtrue_costs[v]+h[s]になる
//    (ここでtrue_costsといっているのは残余グラフ上でのhを考慮しないコスト)
//    h[s]=h0[s]なので、h[v]=h0[s]+true_costs[v]になっている
// 4. 得られたs-t経路に沿ってフローを流す
// 5. 2-4の繰り返し
// 6. 返す時にh0[s]に注意（これは元々0にしておくのがいいかも）
// clang-format on

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

    // h0の計算用の補助; うえのコメント参照
    // ここでは負閉路がないことを前提している. 負閉路の検出が必要なら要改修
    vector<int64_t> bellman_ford(int64_t s) const {
        vector<int64_t> costs(n, numeric_limits<int32_t>::max());

        costs[s] = 0;

        for (auto _ : irange(0L, n - 1)) {
            for (auto v : irange(0L, n)) {
                for (auto&& edge : edges[v]) {
                    if (edge.cap == 0) {
                        continue;
                    }

                    auto new_cost = costs[v] + edge.cost;

                    if (new_cost < costs[edge.u]) {
                        costs[edge.u] = new_cost;
                    }
                }
            }
        }

        return costs;
    }

    // 流せるだけ流す場合はfを消して返り値で流量も返すとか
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

            // 以下、1ずつ流してるので注意
            // TODO:
            // まとめて流したほうがよい場合はトレースして流せる量を計算したうえで、
            //  - total_costにcosts[t]の流量倍
            //  - capの増減幅も変える
            //  - fの減少幅も変える
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