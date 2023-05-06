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

    sort(a.begin(), a.end());

    auto m = a.back();

    auto it = lower_bound(a.begin(), a.end(), (m + 1) / 2);
    if (*it == m) {
        --it;
    } else if (it != a.begin() && (m - 2 * *(it - 1)) < (2 * *it - m)) {
        --it;
    }

    cout << m << " " << *it << endl;
}