#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, n;
    cin >> h >> n;

    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        h -= a;
    }

    if (h <= 0) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}