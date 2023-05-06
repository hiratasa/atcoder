#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

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

Mod calc_l(int64_t l, int64_t d) {
    auto z = 0;
    Mod ret = 0;
    for (auto i : irange(0L, d) | reversed) {
        auto a = (1uL << i);

        if ((l & a) > 0) {
            continue;
        }

        ret += Mod(2).pow(z) * Mod(3).pow(i);
        ++z;
    }

    ret += Mod(2).pow(z);

    return ret;
}

Mod calc_r(int64_t r, int64_t d) { return calc_l(~r, d); }

Mod calc_lr(int64_t l, int64_t r, int64_t d) {
    Mod ret = 0;
    for (auto i : irange(0L, d) | reversed) {
        auto a = (1uL << i);

        if ((l & a) == (r & a)) {
            continue;
        }

        if ((l & a) == 0 && (r & a) > 0) {
            ret += calc_l(l, i);
            ret += calc_r(r, i);
        } else {
            return ret;
        }
    }

    ret += 1;

    return ret;
}

Mod calc(int64_t l, int64_t r) {
    Mod ans = 0;

    int64_t a = 1, b = 0;
    while ((a << 1) <= l) {
        a = (a << 1);
        ++b;
    }

    if ((a << 1) > r) {
        return calc_lr(l, r, b);
    }

    ans += calc_l(l, b);

    a = (a << 1);
    ++b;
    while ((a << 1) <= r) {
        ans += Mod(3).pow(b);
        a = (a << 1);
        ++b;
    }

    ans += calc_r(r, b);

    return ans;
}

main() {
    int64_t l, r;
    cin >> l >> r;

    cout << calc(l, r).n << endl;
}