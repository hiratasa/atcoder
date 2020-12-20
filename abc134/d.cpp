#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> b(n, -1);
    for (auto i : irange(n - 1, -1L, -1L)) {
        int64_t s = 0;
        for (int64_t j = 2; (i + 1) * j <= n; ++j) {
            s += b[(i + 1) * j - 1];
        }

        b[i] = (s - a[i] + 2) % 2;
    }

    auto m = count_if(b.begin(), b.end(), [](int64_t bb) { return bb; });
    cout << m << endl;
    const auto* delim = "";
    for (auto i : irange(0L, n)) {
        if (b[i]) {
            cout << delim << i + 1;
            delim = " ";
        }
    }
    cout << "\n";
}