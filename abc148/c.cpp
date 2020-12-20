#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t gcd(int64_t a, int64_t b) {
    if (a < b) {
        swap(a, b);
    }

    assert(a >= b);
    if (b == 0) {
        return a;
    }

    return gcd(b, a % b);
}

int main() {
    int64_t a, b;
    cin >> a >> b;

    cout << a * b / gcd(a, b) << endl;
}