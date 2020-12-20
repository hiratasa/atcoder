#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    constexpr int64_t M = 'z' - 'a' + 1;

    unordered_map<int64_t, int64_t> m;
    m[0] = 0;
    bitset<M> bs;
    for (auto c : s) {
        auto i = c - 'a';
        bs.flip(i);

        if (!m.count(bs.to_ulong())) {
            m[bs.to_ulong()] = numeric_limits<int64_t>::max();
        }
        auto bs2 = bs;
        for (auto j : irange(0L, M)) {
            bs2.flip(j);
            if (m.count(bs2.to_ulong())) {
                m[bs.to_ulong()] = min(m[bs.to_ulong()], m[bs2.to_ulong()] + 1);
            }
            bs2.flip(j);
        }

        // cerr << bs << ":" << m[bs.to_ulong()] << endl;
    }

    if (bs.none()) {
        cout << 1 << endl;
    } else {
        cout << m[bs.to_ulong()] << endl;
    }
}