#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_map<int64_t, int64_t> m;
    int64_t mi = n, mx = 0;
    for (auto _ : irange(0L, n)) {
        int64_t a;
        cin >> a;
        ++m[a];
        mx = max(mx, a);
        mi = min(mi, a);
    }

    if (mi * 2 < mx) {
        cout << "Impossible" << endl;
        return 0;
    }

    if (m[(mx + 1) / 2] != 1 + (mx % 2)) {
        cout << "Impossible" << endl;
        return 0;
    }

    for (auto i : irange((mx + 1) / 2 + 1, mx + 1)) {
        if (m[i] < 2) {
            cout << "Impossible" << endl;
            return 0;
        }
    }

    cout << "Possible" << endl;
}