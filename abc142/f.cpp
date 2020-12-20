#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

enum {
    kNotVisited,
    kVisitedOnWay,
    kVisitedNotOnWay,
};

bool dfs(const vector<vector<int64_t>>& links, vector<int64_t>& state,
         vector<int64_t>& loop, int64_t current) {
    if (state[current] == kVisitedOnWay) {
        loop.push_back(current);
        return true;
    } else if (state[current] == kVisitedNotOnWay) {
        return false;
    }

    state[current] = kVisitedOnWay;
    for (auto v : links[current]) {
        auto tmp = dfs(links, state, loop, v);
        if (tmp) {
            if (loop.size() == 1 || loop.front() != loop.back()) {
                loop.push_back(current);
            }

            return true;
        }
    }
    state[current] = kVisitedNotOnWay;

    return false;
}

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> links(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;

        links[a].push_back(b);
    }

    vector<int64_t> state(n);
    vector<int64_t> loop;
    bool has_loop = false;
    for (int64_t i = 0; i < n && !has_loop; ++i) {
        has_loop = dfs(links, state, loop, i);
    }
    if (!has_loop) {
        cout << -1 << endl;
        return 0;
    }

    loop.pop_back();
    reverse(loop.begin(), loop.end());
    vector<int64_t> next;
    next.reserve(loop.size());
    while (true) {
        vector<int64_t> idx(n, -1);
        for (auto i : irange(0uL, loop.size())) {
            idx[loop[i]] = i;
        }

        next.clear();
        for (auto i : irange(0uL, loop.size())) {
            auto u = loop[i];
            for (auto v : links[u]) {
                if (!(idx[v] == -1 || idx[v] == (i + 1) % loop.size())) {
                    if (idx[u] < idx[v]) {
                        for (auto j : irange(0L, idx[u] + 1)) {
                            next.push_back(loop[j]);
                        }
                        for (auto j : irange(idx[v], (int64_t)loop.size())) {
                            next.push_back(loop[j]);
                        }
                    } else {
                        for (auto j : irange(idx[v], idx[u] + 1)) {
                            next.push_back(loop[j]);
                        }
                    }
                    break;
                }
            }

            if (!next.empty()) {
                break;
            }
        }

        if (next.empty()) {
            break;
        }
        loop = std::move(next);
    }

    cout << loop.size() << endl;
    for (auto v : loop) {
        cout << v + 1 << "\n";
    }
}