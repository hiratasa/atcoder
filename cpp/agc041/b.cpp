#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, v, p;
    cin >> n >> m >> v >> p;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    sort(a.rbegin(), a.rend());
    auto r = irange(0L, n);
    auto ans = partition_point(r.begin(), r.end(),
                               [&](int64_t idx) {
                                   if (idx < p) {
                                       return true;
                                   }

                                   auto aa = a[idx];

                                   auto b = aa + m;
                                   int64_t j =
                                           lower_bound(a.begin(), a.end(), b,
                                                       greater<int64_t>()) -
                                           a.begin();
                                   if (j >= p) {
                                       return false;
                                   }

                                   auto d = m * v;
                                   d -= m * p;

                                   for (int64_t k = p - 1; k < n; ++k) {
                                       if (k == idx) {
                                           continue;
                                       }

                                       d -= min(b - a[k], m);
                                   }

                                   return d <= 0;
                               }) -
               r.begin();

    cout << ans << endl;
}