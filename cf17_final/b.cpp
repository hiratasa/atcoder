#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    auto na = count(s.begin(), s.end(), 'a');
    auto nb = count(s.begin(), s.end(), 'b');
    auto nc = count(s.begin(), s.end(), 'c');

    if (max({na, nb, nc}) - min({na, nb, nc}) <= 1) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}