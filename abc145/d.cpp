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

main() {
    int64_t x, y;
    cin >> x >> y;

    if ((x + y) % 3 > 0) {
        cout << 0 << endl;
        return 0;
    }

    int64_t n = (x + y) / 3;
    if (x < n || y < n) {
        cout << 0 << endl;
        return 0;
    }

    auto dx = x - n, dy = y - n;
    assert(dx + dy == n);

    constexpr auto M = 1000000007L;

    Mod c(1);
    for (auto i : irange(n - dx + 1, n + 1)) {
        c *= i;
    }
    for (auto i : irange(1L, dx + 1)) {
        c /= i;
    }

    cout << c.n << endl;
}