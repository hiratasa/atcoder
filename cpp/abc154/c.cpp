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
    for (auto&& aa : a) {
        cin >> aa;
    }

    unordered_set<int64_t> s;
    for (auto aa : a) {
        if (s.count(aa)) {
            cout << "NO" << endl;
            return 0;
        }

        s.insert(aa);
    }

    cout << "YES" << endl;
}