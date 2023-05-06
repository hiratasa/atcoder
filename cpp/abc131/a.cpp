#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    for (auto i : irange(1uL, s.size())) {
        if (s[i - 1] == s[i]) {
            cout << "Bad" << endl;
            return 0;
        }
    }

    cout << "Good" << endl;
}