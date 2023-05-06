#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t k, a, b;
    cin >> k >> a >> b;

    if (b - a <= 2) {
        cout << k + 1 << endl;
        return 0;
    }

    int64_t remains = k, current = 1;
    while (remains > 0) {
        if (remains < 2) {
            current += 1;
            break;
        }

        if (current < a) {
            auto delta = min(remains, a - current);
            remains -= delta;
            current += delta;
            continue;
        }

        auto m = remains / 2;
        remains -= 2 * m;
        current += m * (b - a);
    }

    cout << current << endl;
}