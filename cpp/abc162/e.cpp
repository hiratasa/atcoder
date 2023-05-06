#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007L;

int64_t powm(int64_t x, int64_t y) {
    if (y == 0) {
        return 1;
    }

    if (y == 1) {
        return x;
    }

    int64_t t = powm(x, y / 2);

    return t * t % M * powm(x, y % 2) % M;
}

int main() {
    int64_t n, k;
    cin >> n >> k;

    int64_t ans = 0;
    vector<int64_t> nums(k + 1);
    for (auto g : irange(1L, k + 1) | reversed) {
        nums[g] += powm(k / g, n);
        for (auto gg = 2 * g; gg <= k; gg += g) {
            nums[g] += M - nums[gg];
            nums[g] %= M;
        }
        ans += g * nums[g] % M;
        ans %= M;
    }

    cout << ans << endl;
}