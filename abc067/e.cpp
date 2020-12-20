#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

struct Mod {
    static constexpr auto kMod = 1000000007L;

    // can be implicitly converted
    Mod(int64_t n) : n(n) {}

    Mod operator*(Mod m) const { return (n * (m.n % kMod)) % kMod; }

    Mod& operator*=(Mod m) {
        *this = *this * m;
        return *this;
    }

    Mod pow(int64_t p) {
        if (p == 0) {
            return 1;
        }
        if (p == 1) {
            return n;
        }

        int64_t r = this->pow(p / 2).n;
        if (p % 2 == 0) {
            return r * r % kMod;
        } else {
            return (r * r % kMod) * n % kMod;
        }
    }

    Mod operator/(Mod m) const {
        if (n == 0) {
            return 0;
        }

        return *this * m.pow(kMod - 2);
    }

    Mod& operator/=(Mod m) {
        *this = *this / m;
        return *this;
    }

    Mod operator+(Mod m) const { return (n + m.n) % kMod; }

    Mod& operator+=(Mod m) {
        *this = *this + m;
        return *this;
    }

    Mod operator-(Mod m) const { return (kMod + n - m.n) % kMod; }

    Mod& operator-=(Mod m) {
        *this = *this - m;
        return *this;
    }

    int64_t n;
};

main() {
    int64_t n, a, b, c, d;
    cin >> n >> a >> b >> c >> d;

    vector<Mod> fact(n + 1, 0);
    vector<Mod> ifact(n + 1, 0);
    fact[0] = 1;
    ifact[0] = 1;
    for (auto i : irange(1L, n + 1)) {
        fact[i] = fact[i - 1] * i;
        ifact[i] = ifact[i - 1] / i;
    }

    // K人を全てのグループがi人以下であるようにグループ分けする数 = dp[i][K]
    vector<vector<Mod>> dp(b + 1, vector<Mod>(n + 1, 0));
    for (auto i : irange(0L, a)) {
        dp[i][0] = 1;
    }
    for (auto i : irange(a, b + 1)) {
        dp[i][0] = 1;
        for (auto k : irange(1L, n + 1)) {
            dp[i][k] = dp[i - 1][k];
            // i人ちょうどのグループがg個あるとする
            auto z = ifact[i].pow(c);
            for (auto g = c; i * g <= k && g <= d; ++g) {
                dp[i][k] += fact[k] * ifact[g] * z * ifact[k - g * i] *
                            dp[i - 1][k - g * i];
                z *= ifact[i];
            }
        }
    }

    for (auto i : irange(0L, b + 1)) {
        for (auto j : irange(0L, n + 1)) {
            // cerr << i << "," << j << ":" << dp[i][j] << "\n";
        }
    }

    cout << dp[b][n].n << endl;
}