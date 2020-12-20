#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    if (n == 4) {
        cout << "4\n";
        cout << "1 2\n";
        cout << "1 3\n";
        cout << "4 2\n";
        cout << "4 3\n";
    } else if (n % 2 == 0) {
        cout << 2 * n << endl;

        auto k = n / 2;
        for (auto i : irange(1L, k)) {
            cout << i << " " << i + 1 << "\n";
            cout << i << " " << n - i << "\n";
            cout << n - i + 1 << " " << i + 1 << "\n";
            cout << n - i + 1 << " " << n - i << "\n";
        }
        cout << 1 << " " << k << "\n";
        cout << 1 << " " << k + 1 << "\n";
        cout << n << " " << k << "\n";
        cout << n << " " << k + 1 << "\n";
    } else if (n == 3) {
        cout << "2\n";
        cout << "1 3\n";
        cout << "2 3\n";
    } else {
        cout << 2 * n - 2 << endl;

        auto k = n / 2; // == (n - 1) / 2
        for (auto i : irange(1L, k)) {
            cout << i << " " << i + 1 << "\n";
            cout << i << " " << 2 * k - i << "\n";
            cout << 2 * k - i + 1 << " " << i + 1 << "\n";
            cout << 2 * k - i + 1 << " " << 2 * k - i << "\n";
        }
        cout << 1 << " " << n << "\n";
        cout << 2 * k << " " << n << "\n";
        cout << k << " " << n << "\n";
        cout << k + 1 << " " << n << "\n";
    }


}