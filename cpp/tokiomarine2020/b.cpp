#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, v, b, w, t;
    cin >> a >> v >> b >> w >> t;

    auto d = abs(a - b);
    auto u = v - w;

    if (u <= 0 || u * t < d) {
        cout << "NO" << endl;
    } else {
        cout << "YES" << endl;
    }
}