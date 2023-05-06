#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    // no, no
    // b, no
    // no, a
    // b, a
    int64_t n;
    cin >> n;

    int64_t count = 0;
    int64_t count_b = 0;
    int64_t count_a = 0;
    bool has_a_only = false;
    bool has_b_only = false;
    for (auto _ : irange(0L, n)) {
        string ss;
        cin >> ss;
        for (auto i : irange(0uL, ss.size() - 1)) {
            if (ss[i] == 'A' && ss[i + 1] == 'B') {
                ++count;
            }
        }

        if (ss[ss.size() - 1] == 'A') {
            ++count_a;
            if (ss[0] != 'B') {
                has_a_only = true;
            }
        }
        if (ss[0] == 'B') {
            ++count_b;
            if (ss[ss.size() - 1] != 'A') {
                has_b_only = true;
            }
        }
    }

    if (!has_a_only && !has_b_only && min(count_a, count_b) > 0) {
        count += min(count_a, count_b)  - 1;
    } else {
        count += min(count_a, count_b);
    }

    cout << count << endl;
}