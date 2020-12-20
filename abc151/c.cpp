#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<bool> ac(n);
    vector<int64_t> wa(n);
    for (auto i : irange(0L, m)) {
        int64_t p;
        string s;
        cin >> p >> s;
        --p;

        if (s == "AC") {
            ac[p] = true;
        } else if (!ac[p]) {
            ++wa[p];
        }
    }

    int64_t num_ac = 0, num_wa = 0;
    for (auto i : irange(0L, n)) {
        if (ac[i]) {
            ++num_ac;
            num_wa += wa[i];
        }
    }

    cout << num_ac << " " << num_wa << endl;
}