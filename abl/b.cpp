#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c, d;
    cin >> a >> b >> c >> d;

    auto lower = max(a, c);
    auto upper = min(b, d);

    if (lower <= upper) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}