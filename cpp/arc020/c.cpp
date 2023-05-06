#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t mpow(int64_t a, int64_t b, int64_t m) {
    if (b == 0) {
        return 1;
    }

    auto t = mpow(a, b / 2, m);

    return t * t % m * (b % 2 > 0 ? a : 1) % m;
}

// 1 + a + ... + a^(b - 1) mod m
int64_t msumpow(int64_t a, int64_t b, int64_t m) {
    if (b <= 1) {
        return 1;
    }

    if (b % 2 > 0) {
        return (msumpow(a, b - 1, m) * a % m + 1) % m;
    }

    auto t = msumpow(a, b / 2, m);

    return (t * mpow(a, b / 2, m) % m + t) % m;
}

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> al(n);
    for (auto&& t : al) {
        cin >> t.first >> t.second;
    }

    int64_t b;
    cin >> b;

    int64_t r = 0;
    for (const auto& t : al) {
        auto a = t.first;
        auto l = t.second;

        int64_t d = log10(a) + 1;
        int64_t dd = d * l;

        r = r * mpow(10, dd, b) % b;

        int64_t ar = a % b;
        r += ar * msumpow(mpow(10, d, b), l, b);
        r %= b;
    }

    cout << r << endl;
}