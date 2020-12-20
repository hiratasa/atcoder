#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, m;
    cin >> n >> m;

    string s;
    cin >> s;

    vector<bool> gameover(n + 1);
    int64_t len = 0;
    for (auto c : s | indexed(0)) {
        gameover[c.index()] = (c.value() == '1');
        if (gameover[c.index()]) {
            ++len;

            if (len >= m) {
                cout << -1 << endl;
                return 0;
            }
        } else {
            len = 0;
        }
    }

    vector<int64_t> path;
    path.push_back(n);
    while (path.back() > 0) {
        auto cur = path.back();
        if (cur - m <= 0) {
            path.push_back(0);
            break;
        }

        for (auto cur2 : irange(cur - m, cur)) {
            if (gameover[cur2]) {
                continue;
            }

            path.push_back(cur2);
            break;
        }
    }

    const auto* delim = "";
    for (uint64_t i = 0; i < path.size() - 1; ++i) {
        cout << delim << path[path.size() - 2 - i] - path[path.size() - 1 - i];
        delim = " ";
    }
    cout << endl;
}