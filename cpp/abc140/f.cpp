#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    multiset<int64_t, std::greater<>> s;
    for (auto i : irange(0L, 1L << n)) {
        int64_t ss;
        cin >> ss;
        s.insert(ss);
    }

    multiset<int64_t, std::greater<>> t;
    t.insert(*s.begin());
    s.erase(s.begin());
    for (auto i : irange(0L, n)) {
        multiset<int64_t> t2;
        for (auto tt : t) {
            auto it = s.upper_bound(tt);
            if (it == s.end()) {
                cout << "No" << endl;
                return 0;
            }

            t2.insert(*it);
            s.erase(it);
        }

        for (auto tt : t2) {
            t.insert(tt);
        }
    }

    cout << "Yes" << endl;
}