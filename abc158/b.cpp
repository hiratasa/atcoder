#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, a, b;
    cin >> n >> a >> b;

    cout << n / (a + b) * a + min(n % (a + b), a) << endl;
}