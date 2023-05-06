#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, h, m;
    cin >> a >> b >> h >> m;

    cout << setprecision(20)
         << sqrt(a * a + b * b -
                 2 * a * b *
                         cos(2.0 * M_PI / 720 * (60 * h + m) -
                             2.0 * M_PI / 60 * m))
         << endl;
}