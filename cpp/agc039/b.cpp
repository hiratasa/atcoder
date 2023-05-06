#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> links(n);
    for (auto i : irange(0L, n)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, n)) {
            if (s[j] == '1') {
                links[i].push_back(j);
            }
        }
    }

    int64_t ans = -1;
    for (auto i : irange(0L, n)) {
        vector<int64_t> idx(n, -1);
        idx[i] = 1;
        unordered_set<int64_t> picked;
        picked.insert(i);

        vector<int64_t> current;
        current.push_back(i);
        bool success = true;
        while (!current.empty()) {
            vector<int64_t> next;
            for (auto j : current) {
                for (auto v : links[j]) {
                    if (idx[v] == -1) {
                        idx[v] = idx[j] + 1;
                        next.push_back(v);
                        picked.insert(v);
                    } else if (idx[v] == idx[j] + 1) {
                        continue;
                    } else if (idx[v] == idx[j] - 1) {
                        continue;
                    } else {
                        success = false;
                        break;
                    }
                }

                if (!success) {
                    break;
                }
            }

            if (!success) {
                break;
            }

            current = std::move(next);
        }

        if (picked.size() < n || !success) {
            continue;
        }

        ans = max(ans, *max_element(idx.begin(), idx.end()));
    }

    cout << ans << endl;
}