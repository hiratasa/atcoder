#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t grundy(const vector<vector<int64_t>>& adjs, int64_t v, int64_t p) {
    int64_t g = 0;

    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        auto t = grundy(adjs, u, v);

        g ^= t + 1;
    }

    return g;
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t x, y;
        cin >> x >> y;
        --x;
        --y;
        adjs[x].push_back(y);
        adjs[y].push_back(x);
    }

    auto g = grundy(adjs, 0, -1);

    if (g != 0) {
        cout << "Alice" << endl;
    } else {
        cout << "Bob" << endl;
    }
}