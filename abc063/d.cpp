#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, a, b;
    cin >> n >> a >> b;

    vector<int64_t> h(n);
    for (auto&& hh : h) {
        cin >> hh;
    }

    auto cannot_beat = [&](int64_t m) {
        int64_t t = 0;

        for (auto hh : h) {
            auto h1 = hh - m * b;
            if (h1 > 0) {
                t += (h1 - 1) / (a - b) + 1;
            }
        }

        return t > m;
    };

    auto r = irange(0L, 1000000001L);
    auto ans = *partition_point(r.begin(), r.end(), cannot_beat);

    cout << ans << endl;
}