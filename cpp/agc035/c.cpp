#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int main() {
    int64_t n;
    cin >> n;

    if (__builtin_popcountll(n) == 1) {
        cout << "No" << endl;
        return 0;
    }

    cout << "Yes" << endl;

    auto link = [&](int64_t a, int64_t b) { cout << a << " " << b << "\n"; };

    if (n == 3) {
        link(3, 2);
        link(2, 1);
        link(1, 3 + n);
        link(3 + n, 2 + n);
        link(2 + n, 1 + n);
        return 0;
    }

    auto m = (n + 1) / 4;
    for (auto i : irange(1L, m)) {
        link(4 * i + 3, 4 * i + 2);
        link(4 * i + 2, 4 * i + 1);
        link(4 * i + 1, 4 * i);
        link(4 * i, 4 * i + 3 + n);
        link(4 * i + 3 + n, 4 * i + 2 + n);
        link(4 * i + 2 + n, 4 * i + 1 + n);
        link(4 * i + 1 + n, 4 * i + n);

        if (i != m - 1) {
            link(4 * m - 1, 4 * i + 3);
        }
    }

    if (m > 1) {
        link(4 * m - 1, 1);
    }

    if (n % 4 == 0) {
        link(3, 2);
        link(2, 1);
        link(1, 3 + n);
        link(3 + n, 2 + n);
        link(2 + n, 1 + n);

        link(4 * m, 4 * m - 1);
        link(4 * m + n, (4 * m) ^ (4 * m - 1));
    } else if (n % 4 == 1 || n % 4 == 2) {
        link(3, 2);
        link(2, 4 * m);
        link(4 * m, 4 * m + 1);
        link(4 * m + 1, 3 + n);
        link(3 + n, 2 + n);
        link(2 + n, 4 * m + n);
        link(4 * m + n, 4 * m + 1 + n);

        link(1, 4 * m);
        link(4 * m + 1, 1 + n);

        if (n % 4 == 2) {
            link(4 * m + 2, 2);
            link(4 * m, 4 * m + 2 + n);
        }
    } else if (n % 4 == 3) {
        link(3, 2);
        link(2, 1);
        link(1, 3 + n);
        link(3 + n, 2 + n);
        link(2 + n, 1 + n);
    }
}