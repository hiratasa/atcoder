#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x;
    cin >> x;

    cout << (x / 500) * 1000 + (x % 500) / 5 * 5 << endl;
}