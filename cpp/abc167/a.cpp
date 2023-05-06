#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s, t;
    cin >> s >> t;

    t.pop_back();

    if (s == t) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}