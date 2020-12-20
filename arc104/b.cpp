#include <bits/stdc++.h>

#include <boost/functional/hash.hpp>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    unordered_map<pair<int64_t, int64_t>, int64_t,
                  boost::hash<pair<int64_t, int64_t>>>
            m;

    m[make_pair(0, 0)] = 1;

    int64_t nat = 0, ncg = 0;
    for (auto c : s) {
        switch (c) {
            case 'A':
                ++nat;
                break;
            case 'T':
                --nat;
                break;
            case 'C':
                ++ncg;
                break;
            case 'G':
                --ncg;
                break;
        }

        ++m[make_pair(nat, ncg)];
    }

    int64_t ans = 0;
    for (const auto& [_, k] : m) {
        ans += k * (k - 1) / 2;
    }

    cout << ans << endl;
}