#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<bool>> a(h);
    for (auto i : irange(0L, h)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, w)) {
            a[i].push_back(s[j] == '#');
        }
    }

    for (auto i : irange(0L, h)) {
        for (auto j : irange(0L, w)) {
            if (j == 0 || (j < w - 1 && i % 2 == 0) || a[i][j]) {
                cout << '#';
            } else {
                cout << '.';
            }
        }
        cout << "\n";
    }
    cout << "\n";

    for (auto i : irange(0L, h)) {
        for (auto j : irange(0L, w)) {
            if (j == w - 1 || (j > 0 && i % 2 == 1) || a[i][j]) {
                cout << '#';
            } else {
                cout << '.';
            }
        }
        cout << "\n";
    }
}