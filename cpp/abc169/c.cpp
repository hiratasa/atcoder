#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a;
    double bb;
    cin >> a >> bb;

    bb *= 100L;
    int64_t b = (bb + 0.5);

    cout << a * b / 100 << endl;
}