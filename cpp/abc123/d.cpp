#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x, y, z, k;
    cin >> x >> y >> z >> k;

    vector<int64_t> a(x), b(y), c(z);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }
    for (auto&& cc : c) {
        cin >> cc;
    }

    vector<int64_t> d;
    for (auto bb : b) {
        for (auto cc : c) {
            d.push_back(bb + cc);
        }
    }

    sort(d.rbegin(), d.rend());

    d.resize(k);

    vector<int64_t> e;
    for (auto aa : a) {
        for (auto dd : d) {
            e.push_back(aa + dd);
        }
    }

    sort(e.rbegin(), e.rend());

    e.resize(k);

    for (auto ee : e) {
        cout << ee << "\n";
    }
}