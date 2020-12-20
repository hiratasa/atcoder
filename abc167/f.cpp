#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<string> s(n);
    for (auto&& ss : s) {
        cin >> ss;
    }

    vector<pair<int64_t, int64_t>> t1, t2;
    for (const auto& ss : s) {
        int64_t r = 0L, l = 0L;
        for (auto c : ss) {
            if (c == ')') {
                if (l > 0) {
                    --l;
                } else {
                    ++r;
                }
            } else {
                ++l;
            }
        }

        if (l - r >= 0) {
            t1.emplace_back(r, l - r);
        } else {
            t2.emplace_back(l, r - l);
        }
    }

    sort(t1.begin(), t1.end());

    int64_t r = 0;
    for (const auto& tt : t1) {
        if (r - tt.first < 0) {
            cout << "No" << endl;
            return 0;
        }

        r += tt.second;
    }

    sort(t2.begin(), t2.end());
    int64_t l = 0;
    for (const auto& tt : t2) {
        if (l - tt.first < 0) {
            cout << "No" << endl;
            return 0;
        }

        l += tt.second;
    }

    if (l == r) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}