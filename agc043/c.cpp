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

    Mod pow(int64_t p) const {
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

    int64_t m[3];
    vector<vector<int64_t>> links[3];
    for (auto i : irange(0L, 3L)) {
        links[i].resize(n);
        cin >> m[i];

        for (auto _ : irange(0L, m[i])) {
            int64_t a, b;
            cin >> a >> b;
            --a;
            --b;

            if (a > b) swap(a, b);

            links[i][b].push_back(a);
        }
    }

    vector<Mod> s(3), s2(3);
    Mod mm = Mod(10).pow(18);
    vector<vector<bool>> active(3, vector<bool>(n, true));
    for (auto i : irange(0L, 3L)) {
        for (auto j : irange(0L, n) | reversed) {
            if (!active[i][j]) {
                s2[i] += mm.pow(j + 1);
                continue;
            }

            s[i] += mm.pow(j + 1);

            for (auto b : links[i][j]) {
                active[i][b] = false;
            }
        }
    }

    cout << ((s[0] * s[1] * s[2]) + (s[0] * s2[1] * s2[2]) +
             (s2[0] * s[1] * s2[2]) + (s2[0] * s2[1] * s[2]))
                    .n
         << endl;
}