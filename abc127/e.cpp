#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

struct Mod {
    static constexpr auto kMod = 1000000007L;
    static constexpr auto kIMod = 1000000005L;

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

    int64_t n;
};


main() {
    int64_t n, m, k;
    cin >> n >> m >> k;

    Mod ans = 1;
    for (auto i : irange(1L, n * m + 1)) {
        ans *= i;
    }

    for (auto i : irange(1L, k + 1)) {
        ans /= i;
    }

    for (auto i : irange(1L, n * m - k + 1)) {
        ans /= i;
    }

    cerr << ans.n << endl;

    ans *= n + m;
    ans *= k;
    ans *= k - 1;

    cerr << ans.n << endl;

    ans /= 6;

    cout << ans.n << endl;
}