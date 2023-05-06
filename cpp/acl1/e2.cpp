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

    bool operator==(const Mod& rhs) const { return n == rhs.n; }

    int64_t n;
};

class BIT {
   public:
    explicit BIT(int64_t n) : n(n), b(n + 1) {}

    // [0, i)の和
    Mod sum(int64_t i) const {
        Mod s = 0;

        while (i > 0) {
            s += b[i];
            i -= (i & -i);
        }

        return s;
    }

    // [i, j) の和
    Mod sum(int64_t i, int64_t j) const { return sum(j) - sum(i); }

    void add(int64_t i, Mod a) {
        // 1-indexedに直す
        ++i;

        while (i <= n) {
            b[i] += a;
            i += (i & -i);
        }
    }

   private:
    int64_t n;
    vector<Mod> b;
};

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector p(n, 0L);
    for (auto&& pp : p) {
        cin >> pp;
    }

    BIT bit(n + 1), bit1(n + 1);

    Mod half = Mod(1) / 2;

    Mod ans = 0;
    for (auto i : irange(0L, k)) {
        ans += bit.sum(0, p[i]);
        ans += bit1.sum(p[i], n + 1);
        ans -= bit.sum(p[i], n + 1);

        bit.add(p[i], half);
        bit1.add(p[i], Mod(1));
    }

    Mod m = Mod(1) - Mod(1) / k;
    Mod c = 1;
    for (auto i : irange(k, n)) {
        c *= m;

        ans += c * bit.sum(0, p[i]);
        ans += bit1.sum(p[i], n + 1);
        ans -= c * bit.sum(p[i], n + 1);

        bit.add(p[i], half / c);
        bit1.add(p[i], Mod(1));
    }

    cout << ans.n << endl;
}