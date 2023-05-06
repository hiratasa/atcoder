#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

using V = complex<double>;

void dft(vector<V>& f, int64_t dir = 1) {
    int64_t n = f.size();

    if (n == 1) {
        return;
    }

    assert(n % 2 == 0);

    int64_t d = round(log2(n));

    for (auto i : irange(0L, n)) {
        int64_t j = 0;

        for (auto l : irange(0L, d)) {
            j |= ((i >> l) & 1) << (d - l - 1);
        }

        if (i < j) {
            swap(f[i], f[j]);
        }
    }

    double pi = acos(-1);
    for (auto i : irange(0L, d)) {
        auto b = 1L << i;

        for (auto j : irange(0L, n / 2 / b)) {
            V z = polar(1.0, 2 * pi * dir / (2 * b));
            V p = 1;

            for (auto k : irange(0L, b)) {
                auto t1 = f[j * 2 * b + k];
                auto t2 = f[j * 2 * b + k + b];
                f[j * 2 * b + k] = t1 + p * t2;
                f[j * 2 * b + k + b] = t1 - p * t2;
                p *= z;
            }
        }
    }
}

void inv_dft(vector<V>& f) {
    int64_t n = f.size();

    dft(f, -1);
    for (auto&& x : f) {
        x /= V(n);
    }
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<V> p(1L << 18);
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        p[a] += 1;
    }

    dft(p);

    vector<V> q(p.size());
    for (auto i : irange(0uL, p.size())) {
        q[i] = p[i] * p[i];
    }

    inv_dft(q);

    int64_t ans = 0, num = 0;
    for (auto i : irange(0uL, q.size()) | reversed) {
        int64_t qq = q[i].real() + 0.5;

        if (num + qq > m) {
            ans += (m - num) * i;
            break;
        }

        ans += qq * i;
        num += qq;
    }

    cout << ans << endl;
}