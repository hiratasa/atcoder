#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    string s;
    cin >> s;

    for (auto cc : index(s)) {
        auto c = cc.value();
        if ((cc.index() + 1) % 2 == 0) {
            if (c == 'R') {
                cout << "No" << endl;
                return 0;
            }
        } else {
            if (c == 'L') {
                cout << "No" << endl;
                return 0;
            }
        }
    }

    cout << "Yes" << endl;
}