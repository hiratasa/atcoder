#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    constexpr auto M = 1000000007L;
    int64_t n;
    cin >> n;

    vector<int64_t> c(n);
    for (auto&& cc : c) {
        cin >> cc;
    }

    sort(c.begin(), c.end());
    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        ans += c[i] * (n - i + 1);
        ans %= M;
    }
    for (auto i : irange(0L, n - 1)) {
        ans *= 4;
        ans %= M;
    }

    cout << ans << endl;
}