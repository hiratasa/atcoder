#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    long double p;
    int64_t n;
    cin >> p >> n;

    auto ans = 0.5 - 0.5 * pow(1 - 2 * p, (long double)n);
    cout << setprecision(20) << ans << endl;
}