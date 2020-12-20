#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& adjs, vector<int64_t>& c,
         vector<int64_t>& d, int64_t v, int64_t p) {
    d[v] = c.back();
    c.pop_back();

    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        dfs(adjs, c, d, u, v);
    }
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    int64_t m = 0;
    vector<int64_t> c(n);
    for (auto&& cc : c) {
        cin >> cc;
        m += cc;
    }

    sort(c.begin(), c.end());
    m -= c.back();

    vector<int64_t> d(n);
    dfs(adjs, c, d, 0L, -1L);

    cout << m << endl;

    const auto* delim = "";
    for (auto dd : d) {
        cout << delim << dd;
        delim = " ";
    }
    cout << endl;
}