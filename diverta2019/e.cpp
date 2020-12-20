#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    int64_t x = 0;
    vector<int64_t> xx(n);
    for (auto i : irange(0L, n)) {
        int64_t aa;
        cin >> aa;

        x ^= aa;
        xx[i] = x;
    }

    constexpr auto M = 1000000007;

    auto last_x = xx.back();
    cerr << last_x << endl;
    if (last_x > 0) {
        int64_t count = 0;
        int64_t count0 = 0;
        for (auto x : xx) {
            if (x == 0) {
                count0 += count;
                count0 %= M;
            } else if (x == last_x) {
                count += count0 + 1;
                count %= M;
            }
        }
        cout << count0 + 1 << endl;
    } else {
        unordered_map<int64_t, int64_t> count0;
        unordered_map<int64_t, int64_t> count;
        unordered_map<int64_t, int64_t> last_num0;
        int64_t num0 = 0;
        for (auto i : irange(0L, n)) {
            auto x = xx[i];
            if (x == 0) {
                ++num0;
            } else {
                count0[x] += (count[x] * ((num0 - last_num0[x]) % M)) % M;
                count[x] += count0[x] + 1;
                count[x] %= M;
                last_num0[x] = num0;
            }
        }

        int64_t ans = 0;
        for (auto kv : count) {
            ans += kv.second;
            ans %= M;
        }

        int64_t p0 = 1;
        for (auto i : irange(0L, num0 - 1)) {
            p0 *= 2;
            p0 %= M;
        }

        cerr << ans << endl;
        cerr << p0 << endl;
        cout << (ans + p0) % M << endl;
    }
}