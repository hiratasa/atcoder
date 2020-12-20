#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t g;
    cin >> g;
    for (auto _ : irange(1L, n)) {
        int64_t a;
        cin >> a;
        g = std::gcd(g, a);
    }

    cout << g << endl;
}