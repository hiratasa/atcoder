#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    unordered_map<int64_t, int64_t> pos;
    vector<int64_t> next(n);
    for (auto i : irange(n - 1, -1L, -1L)) {
        if (pos.count(a[i]) > 0) {
            next[i] = pos[a[i]];
        } else {
            next[i] = -1;
        }
        pos[a[i]] = i;
    }

    vector<int64_t> hop(n);
    for (auto i : irange(n - 1, -1L, -1L)) {
        if (next[i] < 0) {
            hop[i] = i;
        } else if (next[i] == n - 1) {
            hop[i] = -1;
        } else {
            hop[i] = hop[next[i] + 1];
        }
    }

    unordered_map<int64_t, int64_t> first;
    unordered_map<int64_t, int64_t> last;
    for (auto i : irange(0L, n)) {
        if (first.count(a[i]) == 0) {
            first[a[i]] = i;
        }
        last[a[i]] = i;
    }

    vector<pair<int64_t, int64_t>> arr;
    auto hhh = hop[0];
    while (hhh >= 0) {
        arr.emplace_back(a[hhh], hhh);

        if (hhh == n - 1) {
            break;
        }

        hhh = hop[hhh + 1];
    }

    if (arr.empty()) {
        // output is empty
        cout << endl;
        return 0;
    }

    int64_t offset = 0;
    auto cur_num = arr.front().first;
    unordered_map<int64_t, int64_t> appear;
    vector<int64_t> hop_idx;
    hop_idx.push_back(-1);
    hop_idx.push_back(arr.front().second);
    appear[-1] = 0;
    appear[arr.front().second] = 1;
    while (true) {
        auto f = first[cur_num];

        if (f == n - 1) {
            hop_idx.push_back(-1);
            break;
        }

        auto h = hop[f + 1];
        hop_idx.push_back(h);
        if (appear.count(h) > 0) {
            offset = appear[h];
            break;
        }

        cur_num = a[h];
        appear[h] = hop_idx.size() - 1;
    }
    int64_t prd = hop_idx.size() - offset - 1;
    cerr << offset << " " << prd << endl;

    int64_t hh = 0;
    if (k < offset) {
        hh = hop_idx[k];
    } else {
        hh = hop_idx[(k - offset) % prd + offset];
    }

    if (hh < 0) {
        // output is empty
        cout << endl;
        return 0;
    }

    arr.clear();
    hhh = hh;
    while (hhh >= 0) {
        arr.emplace_back(a[hhh], hhh);

        if (hhh == n - 1) {
            break;
        }

        hhh = hop[hhh + 1];
    }

    const auto* delim = "";
    for (auto aa : arr) {
        cout << delim << aa.first;
        delim = " ";
    }
    cout << endl;
}