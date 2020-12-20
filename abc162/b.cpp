#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
  int64_t n;
  cin >> n;

  int64_t ans = 0;
  for (auto i : irange(1L, n + 1)) {
    if (i % 3 == 0 || i % 5 == 0) {
      continue;
    }

    ans += i;
  }

  cout << ans << endl;
}