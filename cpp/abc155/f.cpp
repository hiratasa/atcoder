#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<pair<int64_t, int64_t>> bomb(n);
    for (auto i : irange(0L, n)) {
        cin >> bomb[i].first >> bomb[i].second;
    }

    sort(bomb.begin(), bomb.end());

    using Q = priority_queue<pair<int64_t, int64_t>,
                             vector<pair<int64_t, int64_t>>, greater<>>;
    vector<Q> code(n);
    for (auto i : irange(0L, m)) {
        int64_t l, r;
        cin >> l >> r;

        l = lower_bound(bomb.begin(), bomb.end(), make_pair(l, 0L)) -
            bomb.begin();
        r = upper_bound(bomb.begin(), bomb.end(), make_pair(r, n)) -
            bomb.begin();

        if (l < r) {
            code[l].emplace(r, i);
        }
    }

    bool flip = false;
    vector<bool> flips(n + 1);
    vector<vector<int64_t>> u(m);
    vector<bool> used(m);
    for (auto i : irange(0L, n)) {
        if (flips[i]) {
            flip = !flip;
        }

        if (bomb[i].second != flip) {
            if (code[i].empty()) {
                cout << -1 << endl;
                return 0;
            }

            flip = !flip;
            flips[code[i].top().first] = !flips[code[i].top().first];
            used[code[i].top().second] = true;

            // cerr << i << ":" << code[i].top().second << " used." << endl;
            // for (auto j : u[code[i].top().second]) {
            //     cerr << " " << j;
            // }
            // cerr << endl;
            // cerr << i << "-" << code[i].top().first << " flipped." << endl;
        } else {
            // cerr << i << ":" << code[i].top().second << " not used." << endl;
        }

        if (code[i].empty()) {
            continue;
        }

        auto prev = code[i].top().second;
        int64_t st = u[prev].size();
        auto c = code[i].top().first;
        code[i].pop();
        while (!code[i].empty()) {
            if (c != code[i].top().first) {
                code[c].emplace(code[i].top().first, code[i].top().second);
                c = code[i].top().first;
                auto tmp = u[code[i].top().second].size();
                u[code[i].top().second].push_back(prev);
                u[code[i].top().second].insert(u[code[i].top().second].end(),
                                               u[prev].begin(),
                                               u[prev].begin() + st);
                st = tmp;
                prev = code[i].top().second;
            }
            code[i].pop();
        }
    }

    vector<int64_t> nums(m);
    for (auto i : irange(0L, m)) {
        if (used[i]) {
            ++nums[i];
            for (auto uu : u[i]) {
                ++nums[uu];
            }
        }
    }

    vector<int64_t> ans;
    for (auto i : irange(0L, m)) {
        if (nums[i] % 2) {
            ans.push_back(i);
        }
    }

    cout << ans.size() << endl;
    const auto* delim = "";
    for (auto i : ans) {
        cout << delim << i + 1;
        delim = " ";
    }
    cout << endl;
}