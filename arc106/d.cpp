#include <bits/stdc++.h>

#include <atcoder/convolution>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector p(10000001L, 0L);
    for (auto aa : a) {
        ++p[aa];
    }

    auto aa = convolution(p, p);
}