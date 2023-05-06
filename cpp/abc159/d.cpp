#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    unordered_map<int64_t, int64_t> um;
    for (auto&& aa : a) {
        cin >> aa;
        ++um[aa];
    }

    int64_t t = 0;
    for (auto kv : um) {
        t += kv.second * (kv.second - 1) / 2;
    }

    for (auto aa : a) {
        cout << t - (um[aa] - 1) << "\n";
    }
}