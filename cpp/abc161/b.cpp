#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
  int64_t n, m;
  cin >> n >> m;

  vector<int64_t> a(n);
  int64_t s = 0;
  for (auto&& aa : a) {
    cin >> aa;
    s += aa;
  }

  sort(a.rbegin(), a.rend());
  cout << (a[m - 1] < (s + 4 * m - 1) / (4 * m) ? "No" : "Yes") << endl;
}