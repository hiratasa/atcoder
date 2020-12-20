#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    if (s == "RRR") {
        cout << 3 << endl;
    } else if (s == "RRS" || s == "SRR") {
        cout << 2 << endl;
    } else if (s == "RSS" || s == "SRS" || s == "SSR" || s == "RSR") {
        cout << 1 << endl;
    } else {
        cout << 0 << endl;
    }
}