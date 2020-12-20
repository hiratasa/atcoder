#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
  int64_t n;
  cin >> n;

  vector<int64_t> a(n);
  for (auto&& aa : a) {
    cin >> aa;
  }

  vector<array<int64_t, 2>> dp(n + 4);
  for (auto i : irange(0L, n)) {
    if (i % 2 == 0) {
      dp[i + 4][0] = max({dp[i + 2][0] + a[i]});
      dp[i + 4][1] = max({dp[i + 3][0], dp[i + 2][0], dp[i + 2][1] + a[i],
                          dp[i + 1][0] + a[i], dp[i][0] + a[i]});
    } else {
      dp[i + 4][0] =
          max({dp[i + 3][0], dp[i + 2][0] + a[i], dp[i + 1][0] + a[i]});
      dp[i + 4][1] = max({dp[i + 3][1], dp[i + 2][0], dp[i + 2][1] + a[i],
                          dp[i + 1][0], dp[i + 1][1] + a[i], dp[i][0] + a[i]});
    }
  }

  cout << (n % 2 == 0 ? dp.back()[0] : dp.back()[1]) << endl;
}