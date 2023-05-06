#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int root(vector<int>& g, int x) {
    if (g[x] != x) {
        g[x] = root(g, g[x]);
    }

    return g[x];
}

void merge(vector<int>& g, int x, int y) {
    g[root(g, x)] = root(g, y);
}

main() {
    int n, m;
    cin >> n >> m;

    vector<int> g(n);
    iota(g.begin(), g.end(), 0);

    for (int _ : irange(0, m)) {
        int x, y, z;
        cin >> x >> y >> z;

        --x;
        --y;
        merge(g, x, y);
    }

    unordered_set<int> s;
    for (int i : irange(0, n)) {
        s.insert(root(g, i));
    }

    cout << s.size() << endl;
}