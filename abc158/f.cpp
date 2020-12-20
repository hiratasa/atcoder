#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> xd(n);
    for (auto&& t : xd) {
        cin >> t.first >> t.second;
    }

    sort(xd.begin(), xd.end());

    constexpr auto M = 998244353;
    constexpr auto K = 1L << 40;
    map<int64_t, int64_t> m;
    m[-K] = 1;
    m[K] = 0;
    for (const auto& t : xd) {
        {
            auto b = m.begin();
            ++b;
            auto e = m.upper_bound(t.first);
            m.erase(b, e);
        }

        {
            auto tmp = m.begin()->second;
            auto b = m.begin();
            ++b;
            tmp -= b->second;
            tmp = (tmp + M) % M;
            auto e = m.upper_bound(t.first + t.second);

            m.erase(b, e);
            m[t.first + t.second] = m.begin()->second;
            m.begin()->second += tmp;
            m.begin()->second %= M;
        }
    }

    cout << m.begin()->second << endl;
}