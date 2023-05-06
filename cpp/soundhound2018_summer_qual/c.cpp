#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, d;
    cin >> n >> m >> d;

    if (d == 0) {
        cout << setprecision(20) << (m - 1) * n / (double)(n * n) << endl;
    } else {
        cout << setprecision(20) << (m - 1) * 2 * (n - d) / (double)(n * n)
             << endl;
    }
}