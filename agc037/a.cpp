#include <bits/stdc++.h>
#include <boost/functional/hash.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    unordered_map<pair<string, string>, int64_t,
                  boost::hash<pair<string, string>>>
            d;
    unordered_map<pair<string, string>, int64_t,
                  boost::hash<pair<string, string>>>
            d2;
    d.emplace(make_pair("", ""), 1);
    for (auto c : s) {
        for (const auto& dd : d) {
            if (dd.first.second.size() < 2) {
                auto key = make_pair(dd.first.first, dd.first.second + c);
                d2[key] = max(d2[key], dd.second);
            }

            if (dd.first.first != dd.first.second) {
                auto key = make_pair(dd.first.second, string(1, c));
                d2[key] = max(d2[key], dd.second + 1);
            }
        }

        d.swap(d2);
        d2.clear();
    }

    int64_t m = -1;
    for (const auto& dd : d) {
        if (dd.first.first == dd.first.second && dd.first.second.size() == 1) {
            continue;
        }
        m = max(m, dd.second);
    }

    std::cout << m << endl;
}
