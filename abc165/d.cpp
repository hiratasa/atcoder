#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, n;
    cin >> a >> b >> n;

    auto x = min(b - 1, n);
    cout << a * x / b - a * (x / b) << endl;
}