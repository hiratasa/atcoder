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
    int64_t r1, c1, r2, c2;
    cin >> r1 >> c1 >> r2 >> c2;

    vector<Mod> fact(2000002L, 1);
    for (auto i : irange(1L, 2000002L)) {
        fact[i] = fact[i - 1] * i;
    }

    Mod ans = 0;
    for (auto r : irange(r1, r2 + 1)) {
        // C(r + c + 1, r + 1)
        ans += fact[r + c2 + 1] / fact[r + 1] / fact[c2];
        ans -= fact[r + (c1 - 1) + 1] / fact[r + 1] / fact[c1 - 1];
    }

    cout << ans.n << endl;
}