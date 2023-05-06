#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    vector<string> days = {"SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"};

    string s;
    cin >> s;
    auto it = find(days.begin(), days.end(), s);

    cout << (days.end() - it) << endl;
}