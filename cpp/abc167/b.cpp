#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c, k;
    cin >> a >> b >> c >> k;

    if (k <= a) {
        cout << k << endl;
    } else if (k <= a + b) {
        cout << a << endl;
    } else {
        cout << a - (k - a - b) << endl;
    }
}