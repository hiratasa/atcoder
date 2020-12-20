#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t m = sqrt(n);

    vector ans(n, 0L);
    for (auto x : irange(1L, m + 1)) {
        for (auto y : irange(1L, m + 1)) {
            for (auto z : irange(1L, m + 1)) {
                int64_t a = x * x + y * y + z * z + x * y + y * z + z * x;

                if (1 <= a && a <= n) {
                    ++ans[a];
                }
            }
        }
    }

    for (auto i : irange(1L, n + 1)) {
        cout << ans[i] << "\n";
    }
}