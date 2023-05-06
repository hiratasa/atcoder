#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
  string s;
  cin >> s;

  cout << ((s[0] == '7' || s[1] == '7' || s[2] == '7') ? "Yes" : "No") << endl;
}