#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector xy(n, make_pair(0L, 0L));
    vector p(n, 0L);
    for (auto i : irange(0L, n)) {
        cin >> xy[i].first >> xy[i].second >> p[i];
    }

    uint64_t m = pow(3L, n) + 0.5;
    for (uint64_t s : irange(0uL, m)) {
        vector b(n, 0L);
        auto ss = s;
        int64_t c = -1L;
        for (auto i : irange(0L, n)) {
            b[i] = ss % 3;
            ss /= 3;
            if (b[i] > 0) {
                c = i;
            }
        }

        auto prev = s - (uint64_t)(pow(3L, c) * b[c]);
    }
}