#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    unordered_map<char, int64_t> m;
    for (auto _ : irange(0L, h)) {
        string s;
        cin >> s;

        for (auto c : s) {
            ++m[c];
        }
    }

    auto n4 = accumulate(m.begin(), m.end(), 0L, [](int64_t s, const auto& kv) {
        return s + kv.second / 4 * 4;
    });

    auto n2 = accumulate(m.begin(), m.end(), 0L, [](int64_t s, const auto& kv) {
        if (kv.second % 4 == 2) {
            return s + 2L;
        } else {
            return s;
        }
    });

    auto n1 = accumulate(m.begin(), m.end(), 0L, [](int64_t s, const auto& kv) {
        if (kv.second % 4 != 2) {
            return s + kv.second % 4;
        } else {
            return s;
        }
    });

    auto no = count_if(m.begin(), m.end(),
                       [](const auto& kv) { return kv.second % 2 > 0; });

    if (h % 2 == 0 && w % 2 == 0) {
        if (n4 == h * w) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    } else if (h % 2 == 0) {
        if (no == 0 && n2 <= h) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    } else if (w % 2 == 0) {
        if (no == 0 && n2 <= w) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    } else {
        if (no == 1 && n1 + n2 <= h + w - 1) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    }
}