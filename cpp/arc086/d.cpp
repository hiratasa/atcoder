#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t b = -1, bi = -1, s = 0;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        if (abs(a) > b) {
            b = abs(a);
            bi = i;
            s = a == 0 ? 0 : (a / abs(a));
        }
    }

    cout << 2 * n - 1 << endl;
    for (auto i : irange(0L, n)) {
        cout << bi + 1 << " " << i + 1 << "\n";
    }

    if (s > 0) {
        for (auto i : irange(0L, n - 1)) {
            cout << i + 1 << " " << i + 2 << "\n";
        }
    } else {
        for (auto i : irange(0L, n - 1) | reversed) {
            cout << i + 2 << " " << i + 1 << "\n";
        }
    }
}