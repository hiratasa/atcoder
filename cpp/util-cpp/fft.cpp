// dft by fft
// see: ABC149 E
#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

using V = complex<double>;

// recursive ver.
/*
void dft(vector<V>& f, int64_t dir = 1) {
    int64_t n = f.size();

    if (n == 1) {
        return;
    }

    assert(n % 2 == 0);

    array<vector<V>, 2> ff;
    ff[0].resize(n / 2);
    ff[1].resize(n / 2);

    for (auto i : irange(0L, n)) {
        ff[i % 2][i / 2] = f[i];
    }

    dft(ff[0], dir);
    dft(ff[1], dir);

    double pi = acos(-1);
    V z = polar(1.0, 2 * pi * dir / n);
    V p = 1;
    for (auto i : irange(0L, n)) {
        f[i] = ff[0][i % (n / 2)] + p * ff[1][i % (n / 2)];
        p *= z;
    }
}
*/

// non-recursive ver.
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

vector<V> conv(vector<V>& p, vector<V>& q) {
    assert(p.size() == q.size());
    // assert(max(p) + max(q) < p.size())

    dft(p);
    dft(q);

    vector<V> r(p.size());
    for (auto i : irange(0uL, p.size())) {
        r[i] = p[i] * q[i];
    }

    inv_dft(r);

    transform(r.begin(), r.end(), r.begin(),
              [](V v) { return round(v.real()); });

    return r;
}