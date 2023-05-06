#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    if (n % 2 != 0) {
        cout << "No" << endl;
        return 0;
    }

    if (s.substr(0, n / 2) == s.substr(n / 2)) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}