#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t dfs(const vector<vector<int64_t>>& links, vector<int64_t>& distance, int64_t node) {
    auto d = distance[node];

    int64_t mn = node;
    int64_t m = d;
    for (auto a : links[node]) {
        if (distance[a] != numeric_limits<int64_t>::max()) {
            continue;
        }

        distance[a] = d + 1;
        auto nn = dfs(links, distance, a);
        if (distance[nn] > m) {
            mn = nn;
            m = distance[nn];
        }
    }

    return mn;
}

main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> links(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        links[a].push_back(b);
        links[b].push_back(a);
    }

    vector<int64_t> distance(n, numeric_limits<int64_t>::max());
    distance[0] = 0;
    int64_t u = dfs(links, distance, 0);

    cerr << "u=" << u << endl;

    distance.assign(n, numeric_limits<int64_t>::max());
    distance[u] = 0;
    int64_t v = dfs(links, distance, u);

    cerr << "v=" << v << endl;

    auto d = distance[v];

    cerr << "d=" << d << endl;

    if (d % 3 == 1) {
        cout << "Second" << endl;
    } else {
        cout << "First" << endl;
    }

    return 0;
}
