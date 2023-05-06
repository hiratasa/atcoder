#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
  int64_t n, k, c;
  cin >> n >> k >> c;

  string s;
  cin >> s;

  if (c == 0) {
    int64_t k2 = count(s.begin(), s.end(), 'o');
    if (k == k2) {
      for (auto i : irange(0L, n)) {
        if (s[i] == 'o') {
          cout << i + 1 << "\n";
        }
      }
    }

    return 0;
  }

  vector<int64_t> l(n), r(n);
  int64_t m = 0;
  for (auto i : irange(0L, n)) {
    l[i] = (i > 0 ? l[i - 1] : 0L);
    if (s[i] == 'o' && i >= m) {
      ++l[i];
      m = i + c + 1;
    }
    // cerr << "l[" << i << "] = " << l[i] << endl;
  }
  m = n - 1;
  for (auto i : irange(0L, n) | reversed) {
    r[i] = (i + 1 < n ? r[i + 1] : 0L);
    if (s[i] == 'o' && i <= m) {
      ++r[i];
      m = i - c - 1;
    }
    // cerr << "r[" << i << "] = " << r[i] << endl;
  }

  vector<int64_t> ans;
  int64_t j = 0;
  for (auto i : irange(-c + 1, n)) {
    int64_t t = (i > 0 ? l[i - 1] : 0L) + (i + c < n ? r[i + c] : 0L);
    if (t >= k) {
      //   cerr << i << ":ok:[" << j << "," << j + c << ")" << endl;
      j = i + c;
    }

    if (i == j) {
      ans.push_back(i);
      ++j;
    }
  }

  for (auto a : ans) {
    cout << a + 1 << endl;
  }
}