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

    for (auto&& c : s) {
        c = ((c - 'A') + n) % ('Z' - 'A' + 1) + 'A';
    }

    cout << s << endl;
}