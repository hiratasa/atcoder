#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    for (auto i : irange(0L, k)) {
        vector b(n, 0L);

        priority_queue<int64_t> q;
        for (auto j : irange(0L, n)) {
            q.push(-(j + a[j]));
            b[j] += q.size();
            while (!q.empty() && -q.top() == j) {
                q.pop();
            }
        }
        q = decltype(q)();
        for (auto j : irange(0L, n) | reversed) {
            b[j] += q.size();
            q.push(j - a[j]);
            while (!q.empty() && q.top() == j) {
                q.pop();
            }
        }

        if (a == b) {
            break;
        }

        a = std::move(b);
    }

    const auto* delim = "";
    for (auto aa : a) {
        cout << delim << aa;
        delim = " ";
    }
    cout << endl;
}