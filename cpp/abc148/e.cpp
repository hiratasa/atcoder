#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    if (n % 2 == 1) {
        cout << 0 << endl;
        return 0;
    }

    int64_t num2 = n / 2;
    int64_t num5 = 0;
    for (int64_t k = 10; k <= n; k *= 5) {
        num5 += n / k;
    }

    cout << min(num2, num5) << endl;
}