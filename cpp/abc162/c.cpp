#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t gcd(int64_t x, int64_t y) {
  if (x == 0) {
    return y;
  }

  return gcd(y % x, x);
}

int main() {
  int64_t k;
  cin >> k;

  int64_t ans = 0;
  for (auto a : irange(1L, k + 1)) {
    for (auto b : irange(1L, k + 1)) {
      int64_t t = gcd(a, b);
      for (auto c : irange(1L, k + 1)) {
        ans += gcd(t, c);
      }
    }
  }

  cout << ans << endl;
}