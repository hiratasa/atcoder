#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <queue>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

using namespace std;

void bfs(const vector<vector<pair<int, int>>>& e, const vector<vector<int>>& c,
         int s, vector<int>* level) {
    level->assign(e.size(), -1);

    queue<int> q;

    q.push(s);
    (*level)[s] = 0;
    while (!q.empty()) {
        auto d = q.front();

        q.pop();

        for (int i = 0; i < e[d].size(); ++i) {
            const auto& ee = e[d][i];
            if (c[d][i] <= 0) {
                continue;
            }

            auto a = ee.first;
            if ((*level)[a] >= 0) {
                continue;
            }

            q.push(a);
            (*level)[a] = (*level)[d] + 1;
        }
    }
}

bool dfs(const vector<vector<pair<int, int>>>& e, const vector<int>& level,
         int s, int t, vector<vector<int>>* c, vector<int>* itr) {
    if (s == t) {
        return true;
    }

    for (int i = (*itr)[s]; i < e[s].size(); ++i) {
        if ((*c)[s][i] <= 0) {
            continue;
        }

        const auto& ee = e[s][i];
        auto a = ee.first;
        if (level[a] <= level[s]) {
            continue;
        }

        auto reached = dfs(e, level, a, t, c, itr);
        if (reached) {
            (*c)[s][i] -= 1;
            (*c)[a][ee.second] += 1;
            return true;
        }
    }

    return false;
}

// min flow
// dinic
int64_t solve(int m, const vector<int>& ls, const vector<int>& rs) {
    int n = ls.size();

    int num_nodes = n + m + 2;
    vector<vector<pair<int, int>>> e(num_nodes);
    vector<vector<int>> c(num_nodes);
    int s = 0;
    int t = 1 + n + m;
    for (int i = 0; i < n; ++i) {
        auto node = i + 1;
        e[s].emplace_back(node, e[node].size());
        c[s].push_back(1);
        e[node].emplace_back(s, e[s].size() - 1);
        c[node].push_back(0);
    }

    for (int i = 0; i < n; ++i) {
        auto node = i + 1;
        for (int j = 1; j <= ls[i]; ++j) {
            auto node2 = n + j;
            e[node].emplace_back(node2, e[node2].size());
            c[node].push_back(1);
            e[node2].emplace_back(node, e[node].size() - 1);
            c[node2].push_back(0);
        }
        for (int j = rs[i]; j <= m; ++j) {
            auto node2 = n + j;
            e[node].emplace_back(node2, e[node2].size());
            c[node].push_back(1);
            e[node2].emplace_back(node2, e[node].size() - 1);
            c[node2].push_back(0);
        }
    }

    for (int i = 1; i <= m; ++i) {
        auto node = n + i;
        e[node].emplace_back(t, e[t].size());
        c[node].push_back(1);
        e[t].emplace_back(node, e[node].size() - 1);
        c[t].push_back(0);
    }

    int f = 0;
    while (true) {
        vector<int> level;
        bfs(e, c, s, &level);

        if (level[t] < 0) {
            break;
        }

        vector<int> itr(e.size(), 0);
        while (dfs(e, level, s, t, &c, &itr)) {
            ++f;
        }
    }

    return n - f;
}

int main() {
    int n, m;
    cin >> n >> m;

    vector<int> ls(n), rs(n);
    for (int i = 0; i < n; ++i) {
        cin >> ls[i] >> rs[i];
    }

    cout << solve(m, ls, rs) << endl;

    return 0;
}