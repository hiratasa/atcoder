#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
  int64_t n;
  cin >> n;

  vector<pair<int64_t, int64_t>> c(n);
  for (auto&& cc : c) {
    cin >> cc.first >> cc.second;
    cc.first += 1L << 30;
    cc.second += 1L << 30;
  }

  int64_t ans = n;
  for (auto i : irange(0L, n)) {
    for (auto j : irange(0L, n)) {
      if (i == j) {
        continue;
      }

      auto p = c[i].first - c[j].first;
      auto q = c[i].second - c[j].second;

      vector<pair<tuple<int64_t, int64_t, int64_t>, int64_t>> d;
      for (const auto& cc : c) {
        d.emplace_back(
            make_tuple(
                p == 0 ? cc.first : cc.first % abs(p),
                q == 0 ? cc.second : cc.second % abs(q),
                ((p == 0 || q == 0) ? 0 : (cc.first / p - cc.second / q))),
            p == 0 ? cc.second / abs(q) : cc.first / abs(p));
      }

      sort(d.begin(), d.end());
      auto key = d.front().first;
      auto pos = d.front().second;
      int64_t count = 1;
      for (auto i : irange(1uL, d.size())) {
          if (d[i].first != key || d[i].second != pos + 1) {
              ++count;
          }
        key = d[i].first;
        pos = d[i].second;
      }
      ans = min(ans, count);
    }
  }

  cout << ans << endl;
}