#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, x;
    cin >> n >> x;

    if (x == 1 || x == 2 * n - 1) {
        cout << "No" << endl;
        return 0;
    }

    cout << "Yes" << endl;
    for (auto i : irange(0L, 2 * n - 1)) {
        cout << (x - 1 + n + i) % (2 * n - 1) + 1 << "\n";
    }
}