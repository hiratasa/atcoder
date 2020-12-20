#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

pair<vector<vector<pair<int64_t, int64_t>>>, vector<pair<int64_t, int64_t>>>
GenerateGraph(int64_t n) {
    vector<vector<pair<int64_t, int64_t>>> adjs(n);
    vector<pair<int64_t, int64_t>> links;

    int64_t id = 0;
    for (auto i : irange(0L, n)) {
        for (auto j : irange(i + 1, n)) {
            adjs[i].emplace_back(j, id);
            adjs[j].emplace_back(i, id);
            links.emplace_back(i, j);
            ++id;
        }
    }

    return make_pair(adjs, links);
}

template <typename T>
bool check(const vector<vector<pair<int64_t, int64_t>>>& adjs,
           const T& available, vector<int64_t>& color, int64_t v) {
    for (const auto& link : adjs[v]) {
        auto u = link.first;
        auto id = link.second;

        if (!available[id]) {
            continue;
        }

        if (color[u] >= 0) {
            if (color[v] + color[u] != 1) {
                return false;
            }

            continue;
        }

        color[u] = (color[v] + 1) % 2;
        if (!check(adjs, available, color, u)) {
            return false;
        }
    }

    return true;
}

int main() {
    int64_t n;
    cin >> n;

    const auto& t = GenerateGraph(n);
    const auto& adjs = t.first;
    const auto& links = t.second;

    int64_t m = links.size();

    int64_t ans = 0;
    for (auto u : irange(0uL, 1uL << m)) {
        bitset<200> bs(u);

        vector<int64_t> color(n, -1);
        color[0] = 0;
        if (!check(adjs, bs, color, 0)) {
            continue;
        }

        if (find(color.begin(), color.end(), -1L) != color.end()) {
            continue;
        }

        bool ok = true;
        for (auto i : irange(0L, m)) {
            if (bs[i]) {
                continue;
            }

            if (color[links[i].first] != color[links[i].second]) {
                ok = false;
                break;
            }
        }

        if (!ok) {
            continue;
        }

        ++ans;
    }

    cout << ans << endl;
}