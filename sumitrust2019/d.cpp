#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    unordered_set<int64_t> s1, s2, s3;
    for (auto c : s) {
        auto k = c - '0';

        for (auto l : s2) {
            s3.insert(l * 10 + k);
        }

        for (auto l : s1) {
            s2.insert(l * 10 + k);
        }

        s1.insert(k);
    }

    cout << s3.size() << endl;
}