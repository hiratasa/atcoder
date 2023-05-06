#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t solve(uint64_t l, int64_t& n, int64_t& m, int64_t b) {
    if (l == 1) {
        n += 1;
        cout << n << " " << m << "\n";
        return 1;
    }

    if ((l & 1) == 0) {
        n += 1;
        m += 2;
        auto v = solve(l / 2, n, m, 2 * b);
        cout << v << " " << v + 1 << " " << b << "\n";
        cout << v << " " << v + 1 << " " << 0 << "\n";
        return v + 1;
    } else {
        m += 1;
        auto v = solve(l - 1, n, m, b);
        cout << 1 << " " << v << " " << (l - 1) * b << "\n";
        return v;
    }
}

int main() {
    int64_t l;
    cin >> l;

    int64_t n = 0, m = 0;
    solve(l, n, m, 1);
}