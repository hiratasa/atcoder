#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c, d;
    cin >> a >> b >> c >> d;

    cout << ((a - 1) / d >= (c - 1) / b ? "Yes" : "No") << endl;
}