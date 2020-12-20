#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    vector<int> nums(5);

    for (auto _ : irange(0, 3)) {
        int a, b;
        cin >> a >> b;
        ++nums[a];
        ++nums[b];
    }

    int k = 0;
    for (auto m : nums) {
        if (m == 1) {
            ++k;
        }
    }

    cout << (k % 2 == 0 ? "YES" : "NO") << endl;
}