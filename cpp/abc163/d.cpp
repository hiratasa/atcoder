#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    constexpr auto M = 1000000007L;
    int64_t ans = 0;
    for (auto i : irange(k, n + 2)) {
        ans += (n + 1 + n + 1 - i) * i / 2 - i * i / 2 + 1;
        ans %= M;
    }

    cout << ans << endl;
}