#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    sort(a.begin(), a.end());

    --n;
    int64_t ans = 0;
    ans += a.back();
    a.pop_back();
    --n;
    while (n > 0) {
        ans += a.back();
        --n;
        if (n > 0) {
            ans += a.back();
            --n;
        }
        a.pop_back();
    }

    cout << ans << endl;
}