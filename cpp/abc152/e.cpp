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
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    unordered_map<int64_t, int64_t> u;
    for (auto aa : a) {
        for (int64_t m = 2; m * m <= aa; ++m) {
            int64_t c = 0;
            while (aa % m == 0) {
                ++c;
                aa /= m;
            }
            if (c > 0) {
                u[m] = max(u[m], c);
            }
        }
        if (aa > 1) {
            u[aa] = max(u[aa], 1L);
        }
    }

    Mod lcm = 1;
    for (auto kv : u) {
        lcm *= Mod(kv.first).pow(kv.second);
    }

    Mod ans = 0;
    for (auto aa : a) {
        ans += lcm / aa;
    }

    cout << ans.n << endl;
}