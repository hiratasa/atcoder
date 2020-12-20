#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
  int64_t n;
  cin >> n;

  string s;
  cin >> s;
  vector<int64_t> t(n);
  unordered_map<char, int64_t> um = {{'R', 1}, {'G', 2}, {'B', 4}};
  for (auto i : irange(0L, n)) {
    t[i] = um[s[i]];
  }

  int64_t ans = 0;
  array<int64_t, 8> dp = {};
  dp[0] = 1;
  for (auto i : irange(0L, n)) {
    decltype(dp) next = dp;
    for (auto k : irange(0L, 8L)) {
      if ((k & t[i]) == 0) {
        next[k | t[i]] += dp[k];
      }
    }

    dp = next;

    for (auto j : irange(0L, i) | reversed) {
      if (i + (i - j) >= n) {
        break;
      }

      if ((t[j] | t[i] | t[i + (i - j)]) == 7) {
        --ans;
      }
    }
  }

  ans += dp[7];
  cout << ans << endl;
}