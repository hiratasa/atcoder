#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t k, x;
    cin >> k >> x;

    if (500 * k >= x) {
        cout << "Yes\n";
    } else {
        cout << "No\n";
    }
}