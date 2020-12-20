#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t t1, t2;
    cin >> t1 >> t2;
    bitset<41> a, b, x;
    a = t1;
    b = t1 + t2;
    for (auto _ : irange(0L, n - 2)) {
        int64_t t;
        cin >> t;
        x ^= t;
    }

    if (n == 2 && t1 < t2) {
        cout << -1 << endl;
        return 0;
    }

    if ((t1 ^ t2 ^ x.to_ullong()) == 0) {
        cout << 0 << endl;
        return 0;
    }

    cerr << a << "," << b << "," << x << endl;

    array dp{-1L, -1L};
    int64_t s = 0;
    for (auto i : irange(0L, 41L) | reversed) {
        if (b[i]) {
            if (x[i]) {
                if (dp[0] >= 0) {
                    dp[0] += (1L << i);
                }
                dp[1] = -1;
            } else {
                if (dp[1] >= 0) {
                    dp[1] = max(dp[0], dp[1] + (1L << i));
                } else {
                    dp[1] = dp[0];
                }
                dp[0] = -1;
            }
        } else {
            if (x[i]) {
                dp[0] = -1;
                if (dp[1] >= 0) {
                    dp[1] = dp[1] + (1L << i);
                }
            } else {
                if (dp[1] >= 0) {
                    dp[0] = max(dp[0], dp[1] + (1L << i));
                }
                dp[1] = -1;
            }
        }

        int64_t t = t1 & ~((1L << (i + 1)) - 1);
        if (a[i]) {
            if (b[i]) {
                if (x[i]) {
                    if (s == 0) {
                        dp[0] = t;
                    }
                } else {
                    if (s == 0) {
                        dp[1] = t;
                    }
                }
            } else {
                if (x[i]) {
                    if (s == 1) {
                        dp[1] = t;
                    }
                } else {
                    if (s == 0) {
                        dp[0] = t;
                    }
                }
            }
        }

        if (((t1 >> i) ^ ((b.to_ullong() >> i) - (t1 >> i)) ^
             (x.to_ullong() >> i)) == 0) {
            s = 0;
        } else if (((t1 >> i) ^ ((b.to_ullong() >> i) - (t1 >> i) - 1) ^
                    (x.to_ullong() >> i)) == 0) {
            s = 1;
        } else {
            s = 2;
        }

        cerr << i << ":" << s << "," << dp[0] << "," << dp[1] << endl;
    }

    if (dp[0] <= 0) {
        cout << -1 << endl;
        // for (auto d : irange(1L, (int64_t)t1 + 1)) {
        //     assert((d ^ (t2 + (t1 - d))) != x.to_ullong());
        // }
    } else {
        cout << t1 - dp[0] << endl;

        assert((dp[0] ^ (t2 + (t1 - dp[0]))) == x.to_ullong());
        // assert(dp[0] < t1);
        // for (auto d : irange(dp[0] + 1, (int64_t)t1 + 1)) {
        //     assert((d ^ (t2 + (t1 - d))) != x.to_ullong());
        // }
    }

    return 0;
}