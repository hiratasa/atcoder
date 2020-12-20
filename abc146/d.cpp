#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<pair<int64_t, int64_t>>>& links,
         vector<int64_t>* output, int64_t cur, int64_t parent_color = 0) {
    int64_t color = 1;
    if (color == parent_color) {
        ++color;
    }
    for (const auto& link : links[cur]) {
        auto id = link.second;
        if ((*output)[id] > 0) {
            continue;
        }

        (*output)[id] = color;
        dfs(links, output, link.first, color);

        ++color;
        if (color == parent_color) {
            ++color;
        }
    }
}

main() {
    int64_t n;
    cin >> n;

    vector<vector<pair<int64_t, int64_t>>> links(n);
    for (auto i : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        links[a].emplace_back(b, i);
        links[b].emplace_back(a, i);
    }

    uint64_t k = 0;
    for (const auto& l : links) {
        k = max(k, l.size());
    }

    vector<int64_t> ans(n - 1);
    dfs(links, &ans, 0);

    cout << k << endl;
    for (auto a : ans) {
        cout << a << "\n";
    }
}