#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> f(n);
    for (auto&& ff : f) {
        cin >> ff;
    }

    sort(a.begin(), a.end());
    sort(f.rbegin(), f.rend());

    int64_t score = 0;
    for (auto i : irange(0L, n)) {
        score = max(a[i] * f[i], score);
    }

    auto r = irange(0L, score + 1);
    auto aa = *partition_point(r.begin(), r.end(), [&](int64_t s) {
        vector<int64_t> c(n);
        for (auto i : irange(0L, n)) {
            c[i] = s / f[i];
        }

        int64_t kk = 0;
        for (auto i : irange(0L, n)) {
            kk += max(a[i] - c[i], 0L);
        }

        return !(kk <= k);
    });

    cout << aa << endl;
}