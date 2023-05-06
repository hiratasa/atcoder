#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    array<bool, 10> v{};

    for (auto _ : irange(0, 4)) {
        int n;
        cin >> n;
        v[n] = true;
    }

    cout << ((v[1] && v[9] && v[7] && v[4]) ? "YES" : "NO") << endl;
}