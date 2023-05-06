#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h1, m1, h2, m2, k;
    cin >> h1 >> m1 >> h2 >> m2 >> k;

    auto l = (h2 * 60 + m2) - (h1 * 60 + m1);

    cout << l - k << endl;
}