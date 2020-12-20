#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    cout << ((s[2] == s[3] && s[4] == s[5]) ? "Yes" : "No") << endl;
}