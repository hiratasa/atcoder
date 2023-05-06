#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    int64_t b = 0;
    for (auto i : irange(0L, 60L) | reversed) {
        vector<int64_t> idxs;
        for (auto aa : a | indexed()) {
            if ((aa.value() & (1L << i)) > 0) {
                idxs.push_back(aa.index());
            }
        }

        if (idxs.empty()) {
            continue;
        }

        int64_t num = idxs.size();
        if (num % 2 == 1) {
            continue;
        }

        auto aa = a[idxs.front()];
        for (auto idx : idxs) {
            a[idx] ^= aa;
        }

        if ((b & (1L << i)) > 0) {
            continue;
        }

        b ^= aa;
    }

    int64_t c = b;
    for (auto aa : a) {
        c ^= aa;
    }

    cout << b + c << endl;
}