#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    vector<int64_t> p(n, -1L);
    queue<int64_t> q;
    q.push(0);
    p[0] = 0;
    while (!q.empty()) {
        auto v = q.front();
        q.pop();

        for (auto u : adjs[v]) {
            if (p[u] >= 0) {
                continue;
            }

            p[u] = v + 1;
            q.push(u);
        }
    }

    cout << "Yes" << endl;
    for (auto v : irange(1L, n)) {
        cout << p[v] << "\n";
    }
}