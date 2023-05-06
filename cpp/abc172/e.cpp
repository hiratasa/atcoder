#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007L;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector fact(m + 1, 1L), ifact(m + 1, 1L), inv(m + 1, 1L);
    for (auto i : irange(2L, m + 1)) {
        inv[i] = (M - inv[M % i] * (M / i) % M) % M;
        fact[i] = fact[i - 1] * i % M;
        ifact[i] = ifact[i - 1] * inv[i] % M;
    }

    int64_t ans = 0;
    int64_t s = 1;
    for (auto i : irange(0L, n + 1)) {
        ans += M + s * fact[n] * ifact[i] % M * ifact[n - i] % M * fact[m] % M *
                           ifact[m - n] % M * fact[m - i] % M * ifact[m - n] %
                           M;
        ans %= M;
        s *= -1;
    }

    cout << ans << endl;
}