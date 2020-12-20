#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_map<int64_t, int64_t> a;
    for (auto _ : irange(0L, n)) {
        int64_t aa;
        cin >> aa;
        ++a[aa];
    }

    for (auto i : irange(1L, 1000001L)) {
        if (!a.count(i)) {
            continue;
        }

        for (int64_t j = i * 2; j <= 1000000L; j += i) {
            a.erase(j);
        }

        if (a[i] > 1) {
            a.erase(i);
        }
    }

    cout << a.size() << endl;
}