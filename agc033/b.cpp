#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t h, w, n;
    cin >> h >> w >> n;

    int64_t s_x, s_y;
    // 順番に注意
    cin >> s_y >> s_x;

    string s_a, s_b;
    cin >> s_a;
    cin >> s_b;

    // to right
    int64_t pos = s_x;
    for (auto i : irange(0L, n)) {
        if (s_a[i] == 'R') {
            ++pos;
        }

        if (pos > w) {
            cerr << "right" << endl;
            cout << "NO" << endl;
            return 0;
        }

        if (s_b[i] == 'L' && pos > 1) {
            --pos;
        }
    }

    // to left
    pos = s_x;
    for (auto i : irange(0L, n)) {
        if (s_a[i] == 'L') {
            --pos;
        }

        if (pos <= 0) {
            cerr << "left" << endl;
            cout << "NO" << endl;
            return 0;
        }

        if (s_b[i] == 'R' && pos < w) {
            ++pos;
        }
    }

    // to top
    pos = s_y;
    for (auto i : irange(0L, n)) {
        if (s_a[i] == 'U') {
            --pos;
        }

        if (pos <= 0) {
            cerr << "top" << endl;
            cout << "NO" << endl;
            return 0;
        }

        if (s_b[i] == 'D' && pos < h) {
            ++pos;
        }
    }

    // to bottom
    pos = s_y;
    for (auto i : irange(0L, n)) {
        if (s_a[i] == 'D') {
            ++pos;
        }

        if (pos > h) {
            cerr << "bottom" << endl;
            cout << "NO" << endl;
            return 0;
        }

        if (s_b[i] == 'U' && pos > 1) {
            --pos;
        }
    }

    cout << "YES" << endl;
    return 0;
}
