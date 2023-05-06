#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<bool> e(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;

        e[a] = !e[a];
        e[b] = !e[b];
    }

    if (count(e.begin(), e.end(), true) == 0) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}