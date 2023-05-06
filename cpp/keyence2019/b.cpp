#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s0 = "keyence";

    string s;
    cin >> s;
    auto n0 = s0.size();
    for (auto i : irange(0uL, n0 + 1)) {
        if (s.substr(0, i) == s0.substr(0, i) && s.substr(s.size() - n0 + i) == s0.substr(i)) {
            cout << "YES" << endl;
            return 0;
        }
    }

    cout << "NO" << endl;
}