#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> d(n, -1L);
    bool ok = true;
    for (auto i : irange(0L, m)) {
        int64_t s, c;
        cin >> s >> c;
        --s;

        if (d[s] < 0) {
            d[s] = c;
        } else if (d[s] != c) {
            ok = false;
        }
    }

    if (!ok || (d[0] == 0 && n > 1)) {
        cout << -1 << endl;
        return 0;
    }

    if (d[0] == -1) {
        d[0] = (n == 1 ? 0 : 1);
    }
    for (auto i : irange(1L, n)) {
        if (d[i] == -1) {
            d[i] = 0;
        }
    }

    for (auto i : irange(0L, n)) {
        cout << d[i];
    }
    cout << endl;
}