#include <bits/stdc++.h>

#include <atcoder/math>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    for (auto _ : irange(0L, n)) {
        int64_t n, m, a, b;
        cin >> n >> m >> a >> b;

        cout << floor_sum(n, m, a, b) << endl;
    }
}