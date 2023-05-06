#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    string s;
    cin >> s;

    int64_t k;
    cin >> k;

    bool b = false;
    int64_t tmp = 0;
    char prev = -1;
    for (auto c : s) {
        if (c == prev && !b) {
            ++tmp;
            b = true;
        } else {
            b = false;
        }

        prev = c;
    }
    auto b1 = b;

    int64_t tmp2 = 0;
    for (auto c : s) {
        if (c == prev && !b) {
            ++tmp2;
            b = true;
        } else {
            b = false;
        }

        prev = c;
    }
    auto b2 = b;

    if (s.size() == 1) {
        cout << k / 2 << endl;
    } else if (s.front() == s.back()) {
        if (b2) {
            cout << tmp * ((k + 1) / 2) + tmp2 * (k / 2) << endl;
        } else {
            if (b1) {
                throw new std::runtime_error("invalid b1.");
                return 1;
            }
            cout << tmp + tmp2 * (k - 1) << endl;
        }
    } else {
        cout << tmp * k << endl;
    }
}