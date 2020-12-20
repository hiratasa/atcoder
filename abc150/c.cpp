#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> p(n), q(n);
    for (auto&& pp : p) {
        cin >> pp;
    }
    for (auto&& qq : q) {
        cin >> qq;
    }
    if (p > q) {
        swap(p, q);
    }

    int64_t ans = 0;
    while (p < q) {
        ++ans;
        next_permutation(p.begin(), p.end());
    }

    cout << ans << endl;
}