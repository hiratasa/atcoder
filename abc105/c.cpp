#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    if (n == 0) {
        cout << 0 << endl;
        return 0;
    }

    vector<int64_t> ans;
    while (n != 0) {
        if (n % 2 != 0) {
            ans.push_back(1);
            --n;
        } else {
            ans.push_back(0);
        }
        n /= -2;
    }

    for (auto a : ans | reversed) {
        cout << a;
    }
    cout << endl;
}