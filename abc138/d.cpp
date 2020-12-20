#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

void dfs(const vector<vector<int64_t>>& links, const vector<int64_t>& x,
         vector<int64_t>& out, int64_t cur, int64_t parent, int64_t counter) {
    counter += x[cur];
    out[cur] = counter;
    for (auto v : links[cur]) {
        if (v == parent) {
            continue;
        }

        dfs(links, x, out, v, cur, counter);
    }

    counter -= x[cur];
}

main() {
    int64_t n, q;
    cin >> n >> q;

    vector<vector<int64_t>> links(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        links[a].push_back(b);
        links[b].push_back(a);
    }

    vector<int64_t> x(n);
    for (auto _ : irange(0L, q)) {
        int64_t p, xx;
        cin >> p >> xx;
        --p;
        x[p] += xx;
    }

    vector<int64_t> ans(n);
    dfs(links, x, ans, 0, -1, 0);

    auto delim = "";
    for (auto aa : ans) {
        cout << delim << aa;
        delim = " ";
    }
    cout << endl;
}