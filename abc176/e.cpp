#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, m;
    cin >> h >> w >> m;

    unordered_set<pair<int64_t, int64_t>, boost::hash<pair<int64_t, int64_t>>>
            targets;
    vector nh(h, 0L), nw(w, 0L);
    for (auto _ : irange(0L, m)) {
        int64_t y, x;
        cin >> y >> x;
        --y;
        --x;
        ++nh[y];
        ++nw[x];
        targets.emplace(y, x);
    }

    auto mxh = *max_element(nh.begin(), nh.end());
    auto mxw = *max_element(nw.begin(), nw.end());

    vector ih(0L, 0L), iw(0L, 0L);
    for (auto i : irange(0L, h)) {
        if (nh[i] == mxh) {
            ih.push_back(i);
        }
    }
    for (auto i : irange(0L, w)) {
        if (nw[i] == mxw) {
            iw.push_back(i);
        }
    }

    if (ih.size() * iw.size() > m) {
        cout << mxh + mxw << endl;
        return 0;
    }

    for (auto y : ih) {
        for (auto x : iw) {
            if (targets.count(make_pair(y, x)) == 0) {
                cout << mxh + mxw << endl;
                return 0;
            }
        }
    }

    cout << mxh + mxw - 1 << endl;
}