#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x, y;
    cin >> x >> y;

    if (abs(x - y) >= 2) {
        cout << "Alice" << endl;
    } else {
        cout << "Brown" << endl;
    }
}