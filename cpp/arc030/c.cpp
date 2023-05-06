#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& adjs, vector<int64_t>& idxs,
         vector<bool>& visited, int64_t v) {
    visited[v] = true;
    for (auto u : adjs[v]) {
        if (visited[u]) {
            continue;
        }

        dfs(adjs, idxs, visited, u);
    }

    idxs.push_back(v);
}

void rdfs(const vector<vector<int64_t>>& adjs,
          vector<unordered_set<int64_t>>& gadjs, vector<int64_t>& g,
          int64_t v) {
    for (auto u : adjs[v]) {
        if (g[u] >= 0) {
            if (g[u] != g[v]) {
                gadjs[g[u]].insert(g[v]);
            }
            continue;
        }

        g[u] = g[v];
        rdfs(adjs, gadjs, g, u);
    }
}

int main() {
    int64_t n, m, k;
    cin >> n >> m >> k;

    vector<char> c(n);
    for (auto&& cc : c) {
        cin >> cc;
    }

    vector<vector<int64_t>> adjs(n), radjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        radjs[b].push_back(a);
    }

    vector<bool> visited(n);
    vector<int64_t> idxs;
    for (auto i : irange(0L, n)) {
        if (!visited[i]) {
            dfs(adjs, idxs, visited, i);
        }
    }

    assert(idxs.size() == n);

    vector<unordered_set<int64_t>> gadjs;
    vector<int64_t> g(n, -1);
    vector<int64_t> nums;
    vector<array<int64_t, 'z' - 'a' + 1>> words;
    for (auto v : idxs | reversed) {
        if (g[v] < 0) {
            g[v] = gadjs.size();
            gadjs.resize(g[v] + 1);
            nums.resize(g[v] + 1);
            words.resize(g[v] + 1);
            rdfs(radjs, gadjs, g, v);
        }
        ++words[g[v]][c[v] - 'a'];
        ++nums[g[v]];
    }

    int64_t ng = gadjs.size();
    vector<int64_t> r(ng);
    for (auto v : irange(0L, ng) | reversed) {
        if (!gadjs[v].empty()) {
            r[v] = r[*max_element(
                    gadjs[v].begin(), gadjs[v].end(),
                    [&](int64_t lhs, int64_t rhs) { return r[lhs] < r[rhs]; })];
        }
        r[v] += nums[v];
    }

    if (*max_element(r.begin(), r.end()) < k) {
        cout << -1 << endl;
        return 0;
    }

    vector<string> s(ng);

    string ans;
    for (auto v : irange(0L, ng)) {
        if (s[v].empty()) {
            for (auto cc : irange('a' + 0, 'z' + 1)) {
                for (auto _ : irange(0L, words[v][cc - 'a'])) {
                    s[v].push_back(cc);
                }
            }

            if (s[v].size() > k) {
                s[v].resize(k);
            }
        }

        for (auto u : gadjs[v]) {
            assert(v < u);

            auto c0 = find_if(words[u].begin(), words[u].end(),
                              [&](int64_t x) { return x > 0; }) -
                      words[u].begin() + 'a';
            assert(c0 >= 'a');
            assert(c0 <= 'z');
            string ss = s[v];
            auto i = min(max(0L, k - r[u]), static_cast<int64_t>(ss.size()));
            while (i < ss.size() && ss[i] <= c0) {
                ++i;
            }

            ss.resize(i);

            for (auto cc : irange('a' + 0, 'z' + 1)) {
                for (auto _ : irange(0L, words[u][cc - 'a'])) {
                    ss.push_back(cc);
                }
            }

            if (ss.size() > k) {
                ss.resize(k);
            }

            int64_t ns = min(ss.size(), s[u].size());
            if (s[u].empty() || ss.substr(0L, ns) < s[u].substr(0L, ns) ||
                (ss.substr(0L, ns) == s[u].substr(0L, ns) &&
                 ss.size() > s[u].size())) {
                s[u] = ss;
            }
        }

        if (s[v].size() == k && (ans.empty() || s[v] < ans)) {
            ans = s[v];
        }
    }

    cout << ans << endl;
}