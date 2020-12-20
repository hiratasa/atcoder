#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    if (n == 1 && m == 0) {
        cout << 1 << " " << 2 << endl;
        return 0;
    }

    if (m < 0 || m >= n - 1) {
        cout << -1 << endl;
        return 0;
    }

    auto k = n - (m + 2);

    cout << 2 << " " << k + 3 << "\n";

    for (auto i : irange(0L, k)) {
        cout << i + 3 << " " << k + i + 4 << "\n";
    }

    for (auto i : irange(0L, m)) {
        cout << 2 * k + 5 + 2 * i - 1 << " " << 2 * k + 5 + 2 * i << "\n";
    }

    cout << 1 << " " << 1000000000L << "\n";
}