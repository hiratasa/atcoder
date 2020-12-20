#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x;
    cin >> x;

    for (auto n = x;; ++n) {
        bool is_prime = true;
        for (int64_t p = 2; p * p <= n; ++p) {
            if (n % p == 0) {
                is_prime = false;
                break;
            }
        }

        if (is_prime) {
            cout << n << endl;
            return 0;
        }
    }
}