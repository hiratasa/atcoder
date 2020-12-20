#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b;
    cin >> a >> b;

    for (auto i : irange(1L, 20001L)) {
        if ((int64_t)(i * 0.08 + 1e-10) == a &&
            (int64_t)(i * 0.1 + 1e-10) == b) {
            cout << i << endl;
            return 0;
        }
    }

    cout << -1 << endl;
}