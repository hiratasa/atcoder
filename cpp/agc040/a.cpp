#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    string s;
    cin >> s;

    int64_t sum = 0;
    int64_t current = 0;
    int64_t n = 1;
    int64_t limit = 0;
    for (auto c : s) {
        if (c == '<') {
            ++current;
            limit = current;
            n = 0;
        } else {
            --current;

            if (current < 0) {
                --limit;
                if (limit == 0) {
                    ++n;
                }
                sum += (-current) * n;
            }
            current = 0;
            ++n;
        }
        sum += current;
    }

    cout << sum << endl;
}