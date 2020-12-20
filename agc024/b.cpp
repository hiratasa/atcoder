#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> q(n, 0L);
    for (auto i : irange(0L, n)) {
        int64_t p;
        cin >> p;
        --p;
        if (p == 0) {
            q[p] = 1;
        } else {
            q[p] = q[p - 1] + 1;
        }
    }

    cout << n - *max_element(q.begin(), q.end()) << endl;
}