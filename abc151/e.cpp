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
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    sort(a.begin(), a.end());

    vector<Mod> fact(n + 1, 1L);
    for (auto i : irange(2L, n + 1)) {
        fact[i] = fact[i - 1] * i;
    }

    Mod ans;
    for (auto i : irange(0L, n)) {
        if (i >= k - 1) {
            // C(i, k - 1)
            ans += fact[i] / fact[k - 1] / fact[i - k + 1] * a[i];
        }

        if (i <= n - k) {
            // C(n - i - 1, k - 1)
            ans -= fact[n - i - 1] / fact[k - 1] / fact[n - k - i] * a[i];
        }
    }

    cout << ans.n << endl;
}