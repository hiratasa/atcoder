#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& adjs, const vector<int64_t>& a,
         vector<vector<int64_t>>& t, vector<int64_t>& ans, int64_t v,
         int64_t p) {
    auto idx = lower_bound(t.begin() + 1, t.end(), a[v],
                           [&](const vector<int64_t>& tt, int64_t aa) {
                               return tt.back() < aa;
                           }) -
               t.begin();
    if (idx == t.size()) {
        t.push_back(vector{a[v]});
    } else {
        t[idx].push_back(a[v]);
    }

    ans[v] = t.size() - 1;

    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        dfs(adjs, a, t, ans, u, v);
    }

    t[idx].pop_back();
    if (t.back().empty()) {
        t.pop_back();
    }
}

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t v, u;
        cin >> v >> u;
        --v;
        --u;
        adjs[v].push_back(u);
        adjs[u].push_back(v);
    }

    vector<vector<int64_t>> t(1);
    vector<int64_t> ans(n);
    dfs(adjs, a, t, ans, 0, -1);

    for (auto aa : ans) {
        cout << aa << "\n";
    }
}