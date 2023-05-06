#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_map<string, int64_t> m;
    for (auto _ : irange(0L, n)) {
        string s;
        cin >> s;
        ++m[s];
    }

    for (const auto* key : {"AC", "WA", "TLE", "RE"}) {
        cout << key << " x " << m[key] << endl;
    }
}