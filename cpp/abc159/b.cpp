#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    int64_t n = s.size();
    for (auto i : irange(0L, (n - 1) / 2)) {
        if (s[i] != s[(n - 1) / 2 - 1 - i] ||
            s[(n + 3) / 2 + i - 1] != s[n - 1 - i] || s[i] != s[n - 1 - i]) {
            cout << "No" << endl;
            return 0;
        }
    }

    cout << "Yes" << endl;
}