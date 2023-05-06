#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t k, x, y;
    cin >> k >> x >> y;

    auto d = abs(x) + abs(y);
    if (k % 2 == 0 && d % 2 > 0) {
        cout << -1 << endl;
        return 0;
    }

    int64_t dirx = (x > 0 ? 1 : -1), diry = (y > 0 ? 1 : -1);

    if (d < k && d % 2 > 0) {
        cout << 3 << endl;

        auto r = 3 * k - d;

        auto dx = r / 4;
        auto dy = (r + 3) / 4;

        cout << -dirx * dx << " " << diry * (k - dx) << endl;
        cout << dirx * (2 * k - abs(y) - dy - 2 * dx) << " " << y + diry * dy
             << endl;
        cout << x << " " << y << endl;
    } else if (abs(x) < abs(y)) {
        int64_t r = (k - d % k) % k;
        if (d / k == 0) {
            r += k;
        }
        if (r % 2 > 0) {
            r += k;
        }
        assert((d + r) % k == 0);

        cout << (d + r) / k << endl;

        int64_t cx = 0, cy = 0;
        while (abs(cx) + k < abs(x) + r / 2) {
            cx += dirx * k;
            cout << cx << " " << cy << "\n";
        }

        cy += diry * (k - (abs(x) + r / 2 - abs(cx)));
        cx = x + dirx * r / 2;
        cout << cx << " " << cy << "\n";

        if (abs(cy) < abs(y)) {
            while (abs(cy) + k < abs(y)) {
                cy += diry * k;
                cout << cx << " " << cy << "\n";
            }

            cx -= dirx * (k - (abs(y) - abs(cy)));
            cy = y;
            cout << cx << " " << cy << "\n";
        }

        while (cx != x) {
            cx -= dirx * k;
            cout << cx << " " << cy << "\n";
        }
    } else {
        int64_t r = (k - d % k) % k;
        if (d / k == 0) {
            r += k;
        }
        if (r % 2 > 0) {
            r += k;
        }
        assert((d + r) % k == 0);

        cout << (d + r) / k << endl;

        int64_t cx = 0, cy = 0;
        while (abs(cy) + k < abs(y) + r / 2) {
            cy += diry * k;
            cout << cx << " " << cy << "\n";
        }

        cx += dirx * (k - (abs(y) + r / 2 - abs(cy)));
        cy = y + diry * r / 2;
        cout << cx << " " << cy << "\n";

        if (abs(cx) < abs(x)) {
            while (abs(cx) + k < abs(x)) {
                cx += dirx * k;
                cout << cx << " " << cy << "\n";
            }

            cy -= diry * (k - (abs(x) - abs(cx)));
            cx = x;
            cout << cx << " " << cy << "\n";
        }

        while (cy != y) {
            cy -= diry * k;
            cout << cx << " " << cy << "\n";
        }
    }
}