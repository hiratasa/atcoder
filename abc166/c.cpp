#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> h(n);
    for (auto&& hh : h) {
        cin >> hh;
    }

    vector<bool> ok(n, true);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;

        if (h[a] <= h[b]) {
            ok[a] = false;
        }
        if (h[a] >= h[b]) {
            ok[b] = false;
        }
    }

    cout << count(ok.begin(), ok.end(), true) << endl;
}