#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool check(const string& a, const string& b, int64_t offset) {
    if (offset < 0) {
        return check(b, a, -offset);
    }

    for (auto i : irange(0uL, b.size())) {
        if (offset + i >= a.size()) {
            return true;
        }

        if (a[offset + i] != '?' && b[i] != '?' && a[offset + i] != b[i]) {
            return false;
        }
    }

    return true;
}

void calc(const string& a, const string& b, unordered_set<int64_t>& idxs) {
    for (auto i : irange(-4000L, 4001L)) {
        if (check(a, b, i)) {
            idxs.insert(i);
        }
    }
}

int main() {
    string a, b, c;
    cin >> a >> b >> c;

    unordered_set<int64_t> ab, ac, bc;
    calc(a, b, ab);
    calc(a, c, ac);
    calc(b, c, bc);

    int64_t ans = 6000;
    for (auto i : ab) {
        for (auto j : bc) {
            if (-(i + j) >= (int64_t)c.size() || i + j >= (int64_t)a.size() ||
                ac.count(i + j)) {
                int64_t s = min({0L, i, i + j});
                int64_t e = max({(int64_t)a.size(), i + (int64_t)b.size(),
                                 i + j + (int64_t)c.size()});
                ans = min(ans, e - s);
            }
        }
    }

    cout << ans << endl;
}