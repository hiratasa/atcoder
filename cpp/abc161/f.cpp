#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

// 1は除く
vector<int64_t> factors(int64_t n) {
  vector<int64_t> ret;
  for (int64_t i = 2; i * i <= n; ++i) {
    if (n % i == 0) {
      ret.push_back(i);
      if (n / i != i) {
        ret.push_back(n / i);
      }
    }
  }

  ret.push_back(n);

  return ret;
}

int main() {
  int64_t n;
  cin >> n;

  if (n == 2) {
    cout << 1 << endl;
    return 0;
  }

  int64_t ans = 0;
  const auto& n_factors = factors(n);
  for (auto f : n_factors) {
    auto nn = n;
    while (nn % f == 0) {
      nn /= f;
    }

    if (nn % f == 1) {
      ++ans;
    }
  }

  const auto& n_1_factors = factors(n - 1);
  ans += n_1_factors.size();

  cout << ans << endl;
}