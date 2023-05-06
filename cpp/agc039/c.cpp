#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 998244353;

int64_t calc(const string& s, vector<int64_t>& uu, int64_t g) {
    int64_t n = s.size();

    auto ss = s.substr(0, g);
    int64_t u = 0;
    int64_t dd = 1;
    for (auto i : irange(0uL, ss.size())) {
        u += dd * (ss[ss.size() - i - 1] - '0');
        u %= M;
        dd *= 2;
        dd %= M;
    }

    auto rr = ss;
    for (auto&& c : rr) {
        if (c == '1') {
            c = '0';
        } else {
            c = '1';
        }
    }

    string r;
    r.reserve(n);
    for (auto i : irange(0L, n / g)) {
        if (i % 2 == 0) {
            r += ss;
        } else {
            r += rr;
        }
    }

    if (r <= s) {
        ++u;
    }
    u %= M;

    for (int64_t i = 1; i * i <= g; ++i) {
        if (g % i > 0) {
            continue;
        }

        if (g != i) {
            u += M - uu[i];
        }
        if (g != g / i && i * i != g) {
            u += M - uu[g / i];
        }

        u %= M;
    }

    uu[g] = u;

    int64_t ans = u * 2 * g;
    ans %= M;

    return ans;
}

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    int64_t num = 0;
    int64_t ans = 0;
    vector<int64_t> uu(n + 1);
    int64_t i = 1;
    for (; i * i <= n; ++i) {
        if (n % i > 0) {
            continue;
        }

        if ((n / i) % 2 == 1) {
            ans += calc(s, uu, i);
            ans %= M;
            cerr << i << ":" << ans << endl;
        }
    }
    for (--i; i >= 1; --i) {
        if (n % i > 0) {
            continue;
        }

        if (i * i == n) {
            continue;
        }

        if (i % 2 == 1) {
            ans += calc(s, uu, n / i);
            ans %= M;
            cerr << n / i << ":" << ans << endl;
        }
    }

    cout << ans << endl;
}