#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t l;
    cin >> l;

    auto k = l / 3.0;

    cout << setprecision(10) << k * k * k << endl;
}