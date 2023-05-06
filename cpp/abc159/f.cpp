#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Mod {
    static constexpr auto kMod = 998244353L;

    Mod() : n(0) {}
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

int main() {
    int64_t n, s;
    cin >> n >> s;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    Mod ans;
    vector<Mod> dp(s);
    for (auto i : irange(0L, n)) {
        auto aa = a[i];
        dp[0] = i + 1;
        if (aa > s) {
            continue;
        }

        ans += dp[s - aa] * (n - i);

        for (auto j : irange(aa, s) | reversed) {
            dp[j] += dp[j - aa];
        }
    }

    cout << ans.n << endl;
}