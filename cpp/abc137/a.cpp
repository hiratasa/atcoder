#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t a, b;
    cin >> a >> b;

    cout << max({a + b, a - b, a * b}) << endl;
}