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
    int64_t A, B, C;
    cin >> A >> B >> C;

    // r / (c+1)
    Mod r_c1 = Mod(B) / Mod(A) - 1;
    // c / (r+1)
    Mod c_r1 = Mod(C) / Mod(A) - 1;

    if (r_c1.n == 0) {
        cout << 0 << " " << c_r1.n << endl;
        return 0;
    }

    if (c_r1.n == 0) {
        cout << r_c1.n << " " << 0 << endl;
        return 0;
    }

    // [c / (r+1) + 1] / [(c+1) / r - c / (r+1)] = r
    int64_t r = ((c_r1 + 1) / (Mod(1) / r_c1 - c_r1)).n;
    int64_t c = (c_r1 * (r + 1)).n;
    cout << r << " " << c << endl;
}