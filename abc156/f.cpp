#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t k, q;
    cin >> k >> q;

    vector<int64_t> d(k);
    for (auto&& dd : d) {
        cin >> dd;
    }

    for (auto _ : irange(0L, q)) {
        int64_t n, x, m;
        cin >> n >> x >> m;

        int64_t s = 0;
        for (auto dd : d) {
            s += (m - 1 + dd) % m + 1;
        }

        s *= (n - 1) / k;

        for (auto i : irange(0L, (n - 1) % k)) {
            s += (m - 1 + d[i]) % m + 1;
        }

        s += x;

        cout << (n - 1) - (s / m - x / m) << "\n";
    }
}