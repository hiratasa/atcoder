#include <bits/stdc++.h>

#include <atcoder/dsu>
#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<std::tuple<int64_t, int64_t, int64_t>> p;
    for (auto i : irange(0L, n)) {
        int64_t x, y;
        cin >> x >> y;

        p.emplace_back(x - 1, y - 1, i);
    }

    std::set<pair<int64_t, int64_t>> s;
    atcoder::dsu d(n);

    sort(p.begin(), p.end());
    for (auto [x, y, i] : p) {
        auto it = s.lower_bound(std::make_pair(y, i));

        if (it == s.begin()) {
            s.emplace(y, i);
        } else {
            auto rit = std::make_reverse_iterator(it);
            while (rit != s.rend()) {
                d.merge(rit->second, i);
                ++rit;
            }

            auto b = s.begin();
            ++b;

            s.erase(b, it);
        }
    }

    for (auto i : irange(0L, n)) {
        cout << d.size(i) << "\n";
    }
}