#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, a, b, c, d;
    cin >> n >> a >> b >> c >> d;

    --a;
    --b;
    --c;
    --d;

    string s;
    cin >> s;

    for (auto i : irange(a + 1, c - 1)) {
        if (s[i] == '#' && s[i + 1] == '#') {
            cout << "No" << endl;
            return 0;
        }
    }

    for (auto i : irange(b + 1, d - 1)) {
        if (s[i] == '#' && s[i + 1] == '#') {
            cout << "No" << endl;
            return 0;
        }
    }

    if (d < c) {
        for (auto i : irange(b, d + 1)) {
            if (s[i - 1] == '.' && s[i] == '.' && s[i + 1] == '.') {
                cout << "Yes" << endl;
                return 0;
            }
        }
        cout << "No" << endl;
        return 0;
    } else {
        cout << "Yes" << endl;
        return 0;
    }
}