#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t r, c, n;
    cin >> r >> c >> n;

    auto on_edge = [&](int64_t x, int64_t y) {
        return (x == 0 || x == r) || (y == 0 || y == c);
    };

    vector<pair<pair<int64_t, int64_t>, int64_t>> px0, pxr, py0, pyc;
    for (auto i : irange(0L, n)) {
        array<int64_t, 2> x, y;
        cin >> x[0] >> y[0] >> x[1] >> y[1];

        if (on_edge(x[0], y[0]) && on_edge(x[1], y[1])) {
            for (auto j : irange(0L, 2L)) {
                if (x[j] == 0) {
                    px0.emplace_back(make_pair(x[j], y[j]), i);
                } else if (x[j] == r) {
                    pxr.emplace_back(make_pair(x[j], y[j]), i);
                } else if (y[j] == 0) {
                    py0.emplace_back(make_pair(x[j], y[j]), i);
                } else if (y[j] == c) {
                    pyc.emplace_back(make_pair(x[j], y[j]), i);
                }
            }
        }
    }

    sort(px0.begin(), px0.end());
    sort(pxr.begin(), pxr.end());
    sort(py0.begin(), py0.end());
    sort(pyc.begin(), pyc.end());

    vector<bool> in_st(n, false);
    vector<int64_t> st;
    for (const auto& entry : px0) {
        auto idx = entry.second;
        if (in_st[idx]) {
            if (st.back() != idx) {
                cout << "NO" << endl;
                return 0;
            }
            st.pop_back();
        } else {
            st.push_back(idx);
            in_st[idx] = true;
        }
    }
    for (const auto& entry : pyc) {
        auto idx = entry.second;
        if (in_st[idx]) {
            if (st.back() != idx) {
                cout << "NO" << endl;
                return 0;
            }
            st.pop_back();
        } else {
            st.push_back(idx);
            in_st[idx] = true;
        }
    }
    for (const auto& entry : pxr | reversed) {
        auto idx = entry.second;
        if (in_st[idx]) {
            if (st.back() != idx) {
                cout << "NO" << endl;
                return 0;
            }
            st.pop_back();
        } else {
            st.push_back(idx);
            in_st[idx] = true;
        }
    }
    for (const auto& entry : py0 | reversed) {
        auto idx = entry.second;
        if (in_st[idx]) {
            if (st.back() != idx) {
                cout << "NO" << endl;
                return 0;
            }
            st.pop_back();
        } else {
            st.push_back(idx);
            in_st[idx] = true;
        }
    }

    cout << "YES" << endl;
}