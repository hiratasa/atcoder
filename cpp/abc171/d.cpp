#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_map<int64_t, int64_t> m;
    int64_t s = 0;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;

        ++m[a];
        s += a;
    }

    int64_t q;
    cin >> q;

    for (auto _ : irange(0L, q)) {
        int64_t b, c;
        cin >> b >> c;

        s += (c - b) * m[b];
        m[c] += m[b];
        m[b] = 0;

        cout << s << "\n";
    }
}