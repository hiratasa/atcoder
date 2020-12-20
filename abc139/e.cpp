#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t index(int64_t a, int64_t b) {
    if (a < b) {
        swap(a, b);
    }
    return a * (a - 1) / 2 + b;
}

main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> links(n * (n - 1) / 2);
    vector<int64_t> nums(n * (n - 1) / 2);
    for (auto i : irange(0L, n)) {
        int64_t prev = -1;
        for (auto j : irange(0L, n - 1)) {
            int64_t a;
            cin >> a;
            --a;

            auto idx = index(i, a);
            if (prev >= 0) {
                links[prev].push_back(idx);
                ++nums[idx];
            }
            prev = idx;
        }
    }

    vector<int64_t> q;
    vector<int64_t> used(n * (n - 1) / 2);
    for (auto i : irange(0L, n * (n - 1) / 2)) {
        if (nums[i] == 0) {
            q.push_back(i);
        }
    }

    int64_t ans = 0;
    while (!q.empty()) {
        ++ans;
        vector<int64_t> next;
        for (auto qq : q) {
            used[qq] = true;
            for (auto v : links[qq]) {
                --nums[v];
                if (nums[v] == 0) {
                    next.push_back(v);
                }
            }
        }
        q = move(next);
    }

    if (any_of(used.begin(), used.end(), [](bool b) { return !b; })) {
        cout << -1 << endl;
    } else {
        cout << ans << endl;
    }
}