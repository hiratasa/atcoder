#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int n, k;
    cin >> n >> k;

    string s;
    cin >> s;

    s[k - 1] = s[k - 1] - 'A' + 'a';
    cout << s << endl;
}