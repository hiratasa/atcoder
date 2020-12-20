#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    auto calc = [&](int64_t i) { return min(i - 1, n) - max(1L, i - n) + 1; };

    int64_t ans = 0;
    for (auto i : irange(2L, 2 * n + 1)) {
        auto j = i - k;

        if (2 <= j && j <= 2 * n) {
            ans += calc(i) * calc(j);
        }
    }

    cout << ans << endl;
}