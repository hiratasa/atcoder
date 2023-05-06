#include <iostream>
#include <vector>
#include <utility>
#include <cmath>
#include <cassert>
#include <cstdint>

using namespace std;

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

Mod weight_d(int n, int d) {
    static Mod f = 0;
    static vector<Mod> w(n + 1, -1);

    if (f.n == 0) {
        f.n = 1;
        for (int j = 2; j <= n; ++j) {
            f *= j;
        }
    }

    if (w[d].n >= 0) {
        return w[d];
    }

    return w[d] = f / d;
}

Mod weight(int n, int i) {
    Mod w(0);
    for (int j = 0; j < n; ++j) {
        auto ww = weight_d(n, abs(j - i) + 1);
        w += ww;
    }

    return w;
}

int64_t solve(int n, const vector<int64_t>& as) {
    Mod ans(0);

    for (int i = 0; i < n; ++i) {
        ans += weight(n, i) * as[i];
    }

    return ans.n;
}


int main() {
    int n;
    cin >> n;

    vector<int64_t> a(n);
    for (int64_t i = 0; i < n; ++i) {
        cin >> a[i];
    }

    cout << solve(n, a) << endl;

    return 0;
}