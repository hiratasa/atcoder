#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
  int64_t n, k;
  cin >> n >> k;

  n %= k;

  cout << min(n, k - n) << endl;
}