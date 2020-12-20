#include <bits/stdc++.h>

#include <boost/optional.hpp>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t dfs(int64_t d, int64_t n, int64_t& k) {
  if (d == 1) {
    --k;
    if (k == 0) {
      return n * d;
    } else {
      return -1;
    }
  }

  for (auto i : irange(max(0L, n - 1), min(9L, n + 1) + 1L)) {
    int64_t ret = dfs(d / 10, i, k);

    if (k == 0) {
      return n * d + ret;
    }
  }

  return -1L;
}

int main() {
  int64_t k;
  cin >> k;

  for (int64_t d = 1;; d *= 10) {
    for (auto i : irange(1L, 10L)) {
      auto ret = dfs(d, i, k);

      if (ret > 0) {
        cout << ret << endl;
        return 0;
      }
    }
  }
}