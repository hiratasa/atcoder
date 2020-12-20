#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

struct Mod {
    static constexpr auto kMod = 1000003L;
    static constexpr auto kIMod = 1000001L;

    // can be implicitly converted
    Mod(int64_t n) : n(n) {}

    Mod operator*(Mod m) const {
        return (n * (m.n % kMod)) % kMod;
    }

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

        int64_t r = this->pow(p/2).n;
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

        return *this * m.pow(kIMod);
    }

    Mod& operator/=(Mod m) {
        *this = *this / m;
        return *this;
    }

    Mod operator+(Mod m) const {
        return (n + m.n) % kMod;
    }

    Mod& operator+=(Mod m) {
        *this = *this + m;
        return *this;
    }

    Mod operator-(Mod m) const {
        return (kMod + n - m.n) % kMod;
    }

    Mod& operator-=(Mod m) {
        *this = *this - m;
        return *this;
    }

    int64_t n;
};

Mod operator/(int64_t lhs, Mod rhs) {
    return Mod(lhs) / rhs;
}

Mod operator*(int64_t lhs, Mod rhs) {
    return Mod(lhs) * rhs;
}

Mod operator+(int64_t lhs, Mod rhs) {
    return Mod(lhs) + rhs;
}

Mod operator-(int64_t lhs, Mod rhs) {
    return Mod(lhs) - rhs;
}

main() {
    auto M = Mod::kMod;

    vector<Mod> fact(M, 1);
    for (auto i : irange(2L, M)) {
        fact[i] = fact[i - 1] * i;
    }

    int64_t q;
    cin >> q;

    for (auto _ : irange(0L, q)) {
        int64_t x, d, n;
        cin >> x >> d >> n;

        if (d == 0) {
            cout << (Mod(x).pow(n)).n << endl;
            continue;
        }

        if (n >= M) {
            cout << 0 << endl;
            continue;
        }

        auto k = (Mod(M - x) / d).n;

        if (k <= n - 1) {
            cout << 0 << endl;
            continue;
        }

        auto a = fact[n - 1 - k + M] / fact[-1 - k + M];

        a *= Mod(d).pow(n);

        cout << a.n << endl;
    }


}