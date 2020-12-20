#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string t;
    cin >> t;

    transform(t.begin(), t.end(), t.begin(), [](char c) {
        if (c == '?') {
            return 'D';
        } else {
            return c;
        }
    });

    cout << t << endl;
}