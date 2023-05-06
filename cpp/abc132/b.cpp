#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> p(n);
    for (auto&& pp : p) {
        cin >> pp;
    }

    int64_t m = 0;
    for (auto i : irange(1L, n - 1)) {
        if (p[i] >= max(p[i - 1], p[i + 1])) {
            continue;
        }

        if (p[i] <= min(p[i - 1], p[i + 1])) {
            continue;
        }

        ++m;
    }

    cout << m << endl;
}