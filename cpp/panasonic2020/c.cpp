#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c;
    cin >> a >> b >> c;

    // a + 2 * sqrt(a * b) + b < c
    // a + b > c  or 4ab < (c - a - b)^2
    auto d = c - a - b;
    if (d >= 0 && 4 * a * b < d * d) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}