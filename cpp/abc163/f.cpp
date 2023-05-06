#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t dfs(const vector<vector<int64_t>>& links, const vector<int64_t>& colors,
            vector<int64_t>& ans, vector<vector<int64_t>>& stacks, int64_t v,
            int64_t p) {
    int64_t ret = 1;
    for (auto u : links[v]) {
        if (u == p) {
            continue;
        }

        stacks[colors[v]].push_back(0);
        auto mm = dfs(links, colors, ans, stacks, u, v);
        auto tmp = stacks[colors[v]].back();
        stacks[colors[v]].pop_back();
        ans[colors[v]] += (mm - tmp) * (mm - tmp + 1) / 2;

        ret += mm;
    }

    stacks[colors[v]].back() += ret;

    return ret;
}

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> colors(n);
    for (auto&& c : colors) {
        cin >> c;
        --c;
    }
    vector<vector<int64_t>> links(n);
    for (auto i : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        links[a].push_back(b);
        links[b].push_back(a);
    }

    vector<int64_t> ans(n);
    vector stacks(n, vector<int64_t>(1));
    dfs(links, colors, ans, stacks, 0, -1);

    for (auto i : irange(0L, n)) {
        ans[i] += (n - stacks[i].back()) * (n - stacks[i].back() + 1) / 2;
    }

    for (auto i : irange(0L, n)) {
        cout << n * (n + 1) / 2 - ans[i] << "\n";
    }
}