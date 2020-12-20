#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Mod {
    static constexpr auto kMod = 1000000007L;

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

        if (m.n == 0) {
            throw;
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
    int64_t n, m, d;
    cin >> n >> m >> d;

    vector a(m, 0L);
    for (auto&& aa : a) {
        cin >> aa;
        --aa;
    }

    sort(a.begin(), a.end());

    auto r = n % d;

    Mod ans = 0;

    Mod nd1(n / d + 1);
    Mod nd(n / d);

    ans += nd1 * (nd1 + 1) / 2 * r;
    ans += nd * (nd + 1) / 2 * (d - r);

    unordered_map<int64_t, vector<int64_t>> b;
    for (auto aa : a) {
        b[aa % d].push_back(aa);
    }

    for (auto&& [rr, c] : b) {
        ans -= (rr < r ? nd1 * (nd1 + 1) / 2 : nd * (nd + 1) / 2);

        int64_t prev = -1;
        for (auto cc : c) {
            Mod nn = cc / d - prev - 1;
            ans += nn * (nn + 1) / 2;
            prev = cc / d;
        }

        if (rr < r) {
            Mod nn = nd1 - prev - 1;
            ans += nn * (nn + 1) / 2;
        } else {
            Mod nn = nd - prev - 1;
            ans += nn * (nn + 1) / 2;
        }
    }

    cout << ans.n << endl;
}