#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, a, b;
    cin >> h >> w >> a >> b;

    constexpr auto M = 1000000007L;

    vector<int64_t> fact(200001, 1), ifact(200001, 1), inv(200001, 1);
    for (auto i : irange(2L, 200001L)) {
        fact[i] = (fact[i - 1] * i) % M;
        inv[i] = M - inv[M % i] * (M / i) % M;
        ifact[i] = ifact[i - 1] * inv[i] % M;

        assert(fact[i] * ifact[i] % M == 1);
    }

    int64_t ans = 0;
    for (auto i : irange(0L, h - a)) {
        // C(i + b - 1, i) * C(h - i - 1 + w - b - 1, h - i - 1)
        ans += fact[i + b - 1] * ifact[i] % M * ifact[b - 1] % M *
               fact[h - i - 1 + w - b - 1] % M * ifact[h - i - 1] % M *
               ifact[w - b - 1] % M;
        ans %= M;
    }

    cout << ans << endl;
}