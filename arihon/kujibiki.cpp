#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    unordered_set<int64_t> k{};
    for (auto _ : irange(0L, n)) {
        int64_t kk;
        cin >> kk;
        k.insert(kk);
    }

    decltype(k) k2;
    for (auto kk : k) {
        for (auto kkk : k) {
            k2.insert(kk + kkk);
        }
    }

    for (auto kk : k2) {
        if (k2.count(m - kk) > 0) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;
}