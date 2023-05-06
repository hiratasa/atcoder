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

    int64_t m = 1000L;
    vector s(201L, 0L);
    for (auto aa : a) {
        vector s2(201L, 0L);

        s2[m % aa] = m / aa;

        for (auto i : irange(0L, 201L)) {
            m = max(m, i + s[i] * aa);
            s2[i % aa] = max(s2[i % aa], s[i] + i / aa);
        }

        s = std::move(s2);
    }

    cout << m << endl;
}