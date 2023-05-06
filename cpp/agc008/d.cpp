#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> x(n);
    for (auto i : irange(1L, n + 1)) {
        cin >> x[i - 1].first;
        --x[i - 1].first;
        x[i - 1].second = i;
    }
    sort(x.begin(), x.end());

    int64_t cur = -1L;
    vector<int64_t> ans(n * n, -1L);
    for (const auto& t : x) {
        auto idx = t.first;
        auto xx = t.second;

        for (auto i : irange(0L, xx - 1)) {
            ++cur;

            while (cur < n * n && ans[cur] != -1L) {
                ++cur;
            }

            assert(cur < n * n);
            ans[cur] = xx;
        }

        if (cur >= idx) {
            cout << "No" << endl;
            return 0;
        }

        ans[idx] = xx;
    }

    cur = n * n;
    for (const auto& t : x | reversed) {
        auto idx = t.first;
        auto xx = t.second;

        for (auto i : irange(xx, n)) {
            --cur;
            while (cur >= 0 && ans[cur] != -1L) {
                --cur;
            }

            assert(cur >= 00);
            ans[cur] = xx;
        }

        if (cur < idx) {
            cout << "No" << endl;
            return 0;
        }
    }

    cout << "Yes" << endl;
    const auto* delim = "";
    for (auto a : ans) {
        assert(a != -1L);
        cout << delim << a;
        delim = " ";
    }
    cout << endl;
}