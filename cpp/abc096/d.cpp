#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> ans;
    for (auto i : irange(2L, 55556L)) {
        if (i % 5 != 1) {
            continue;
        }

        bool ok = true;
        for (auto j = 2L; j * j <= i; ++j) {
            if (i % j == 0) {
                ok = false;
                break;
            }
        }

        if (ok) {
            ans.push_back(i);
        }

        if (ans.size() == n) {
            break;
        }
    }

    const auto* delim = "";
    for (auto a : ans) {
        cout << delim << a;
        delim = " ";
    }
    cout << endl;
}