#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
  int64_t n;
  cin >> n;

  vector<int64_t> a(n);
  int64_t m = 0;
  for (auto&& aa : a) {
    cin >> aa;
    m += aa;
  }

  sort(a.begin(), a.end());

  int64_t count = 0;
  if (a.front() > 0) {
    m -= 2 * a.front();
    count = 1;
  } else if (a.back() <= 0) {
    m *= -1;
    m += 2 * a.back();
    count = a.size() - 1;
  } else {
    for (auto i : irange(0L, n)) {
      if (a[i] <= 0) {
        m -= 2 * a[i];
        ++count;
      } else {
        break;
      }
    }
  }

  vector<pair<int64_t, int64_t>> ans;
  int64_t current = m;
  for (auto i : irange(0L, count - 1)) {
    current += a[i];
    ans.emplace_back(current, a[i]);
  }

  bool flag = true;
  for (auto i : irange(n-1, count-1, -1L)) {
    if (flag) {
      current = a[i] - current;
      ans.emplace_back(a[i], current);
      flag = false;
    } else {
      current += a[i];
      ans.emplace_back(current, a[i]);
    }
  }

  cout << m << "\n";
  for (auto it = ans.rbegin(); it != ans.rend(); ++it) {
    cout << it->first << " " << it->second << "\n";
  }
}