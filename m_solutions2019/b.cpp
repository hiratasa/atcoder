#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    int64_t k = s.size();
    int64_t w = 0;
    for (auto c : s) {
        if (c == 'o') {
            ++w;
        }
    }

    if (15 - k + w >= 8) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}