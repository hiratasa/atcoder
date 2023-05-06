#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

// -1: 左から落ちる、0: 落ちない、1: 右から落ちる
int64_t simulate(const string& s, const vector<pair<char, int64_t>>& spells,
                 int64_t idx) {
    int64_t n = s.size();

    for (const auto& spell : spells) {
        if (spell.first == s[idx]) {
            idx += spell.second;

            if (idx < 0) {
                return -1L;
            }
            if (idx >= n) {
                return 1L;
            }
        }
    }

    return 0L;
}

int main() {
    int64_t n, q;
    cin >> n >> q;

    string s;
    cin >> s;

    vector<pair<char, int64_t>> spells(q);
    for (auto&& sp : spells) {
        char d;
        cin >> sp.first >> d;

        assert('A' <= sp.first && sp.first <= 'Z');
        assert(d == 'L' || d == 'R');

        sp.second = (d == 'R' ? 1 : -1);
    }

    auto r = irange(0L, n);
    auto it1 = partition_point(r.begin(), r.end(), [&](int64_t idx) {
        return simulate(s, spells, idx) < 0;
    });
    auto it2 = partition_point(r.begin(), r.end(), [&](int64_t idx) {
        return simulate(s, spells, idx) <= 0;
    });

    cout << it2 - it1 << endl;
}