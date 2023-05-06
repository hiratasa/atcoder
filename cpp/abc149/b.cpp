#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, k;
    cin >> a >> b >> k;

    a -= k;
    if (a < 0) {
        b -= -a;
        a = 0;
        if (b < 0) {
            b = 0;
        }
    }

    cout << a << " " << b << endl;
}