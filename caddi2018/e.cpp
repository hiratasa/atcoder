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

    int64_t ans = 0;
    vector<int64_t> right(n, 0);
    right[n - 1] = 0;
    int64_t current = a[n - 1];
    for (auto i : irange(n - 2, -1, -1)) {
        if ()
        right[i] = 

    }
    {
        int64_t current = a[0];
        for (auto i : irange(1L, n)) {
            aa = a[i];
            if (aa >= current) {
                continue;
            }

            int64_t x = log2(current / (double) aa) / 2.0;
            aa <<= 2 * x;
            ans += 2 * x;
            while (aa < current) {
                aa *= 4;
                ans += 2;
            }
            current = aa;
        }
    }

    int64_t prev = ans;
    for (auto p : irange(0L, n)) {
        int64_t delta = p + 1;

        if (p + 1 < n) {
            if (a[p] > a[p + 1]) {
                int64_t aa = a[p + 1];
                int64_t x = log2(a[p] / (double) aa) / 2.0;
                aa <<= 2 * x;
                while (aa < a[p]) {
                    aa *= 4;
                    x += 1;
                }
            }

        }

    }


}