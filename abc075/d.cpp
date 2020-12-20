#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector<pair<int64_t, int64_t>> x(n), y(n);
    for (auto i : irange(0L, n)) {
        cin >> x[i].first >> y[i].first;
        x[i].second = y[i].second = i;
    }

    sort(x.begin(), x.end());
    sort(y.begin(), y.end());

    vector<pair<int64_t, int64_t>> points(n);
    for (auto i : irange(0L, n)) {
        points[x[i].second].first = i;
        points[y[i].second].second = i;
    }

    vector<vector<int64_t>> s(n + 1, vector<int64_t>(n + 1));
    for (auto i : irange(0L, n)) {
        ++s[points[i].first + 1][points[i].second + 1];
    }

    for (auto i : irange(0L, n + 1)) {
        for (auto j : irange(0L, n)) {
            s[i][j + 1] += s[i][j];
        }
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n + 1)) {
            s[i + 1][j] += s[i][j];
            assert(s[i + 1][j] <= n);
            assert(s[i + 1][j] >= 0);
        }
    }

    assert(s[n][n] == n);

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto left : irange(0L, n)) {
        for (auto right : irange(left + 1, n)) {
            for (auto bottom : irange(0L, n)) {
                for (auto top : irange(bottom + 1, n)) {
                    auto num = s[right + 1][top + 1] - s[right + 1][bottom] -
                               s[left][top + 1] + s[left][bottom];

                    assert(num >= 0);
                    assert(num <= n);

                    // cerr << left << "-" << right << "x" << bottom << "-" <<
                    // top
                    //      << ":" << num << endl;
                    if (num >= k) {
                        ans = min(ans,
                                  (x[right].first - x[left].first) *
                                          (y[top].first - y[bottom].first));
                    }
                }
            }
        }
    }

    assert(ans != numeric_limits<int64_t>::max());

    cout << ans << endl;
}