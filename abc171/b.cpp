#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector p(n, 0L);
    for (auto&& pp : p) {
        cin >> pp;
    }

    sort(p.begin(), p.end());
    p.resize(k);

    cout << accumulate(p.begin(), p.end(), 0L) << endl;
}