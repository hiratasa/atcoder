#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, a, b;
    cin >> n >> a >> b;

    cout << min(a * n, b) << endl;
}
