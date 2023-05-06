#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    int64_t a = 1, b = n;
    for (auto i : irange(0L, m)) {
        cout << a << " " << b << "\n";

        if (n % 2 == 0 && 2 * (b - a) > n && 2 * (b - a - 2) <= n) {
            --b;
        }
        ++a;
        --b;
    }
}