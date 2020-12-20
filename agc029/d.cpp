#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int main() {
    int64_t h, w, n;
    cin >> h >> w >> n;

    vector<vector<int64_t>> t(h);
    for (auto i : irange(0L, n)) {
        int64_t x, y;
        cin >> x >> y;
        --x;
        --y;
        t[x].push_back(y);
    }

    int64_t s = -1;
    for (auto i : irange(0L, h)) {
        if (find(t[i].begin(), t[i].end(), s + 1) == t[i].end()) {
            ++s;
        }

        if (i == h - 1) {
            cout << h << endl;
            return 0;
        }

        if (t[i + 1].empty()) {
            continue;
        }

        if (any_of(t[i + 1].begin(), t[i + 1].end(),
                   [&](int64_t j) { return j <= s; })) {
            cout << i + 1 << endl;
            return 0;
        }
    }

    return 0;
}