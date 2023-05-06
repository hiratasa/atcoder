#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    unordered_map<int64_t, int64_t> nums;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;

        ++nums[a];
    }

    if (nums.size() == 1) {
        if (nums.begin()->first == 0) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    } else if (n % 3 > 0) {
        cout << "No" << endl;
    } else if (nums.size() == 2) {
        if (nums[0] != n / 3) {
            cout << "No" << endl;
        } else {
            cout << "Yes" << endl;
        }
    } else {
        array<int64_t, 3> m;
        int64_t i = 0;
        for (const auto& kv : nums) {
            if (kv.second != n / 3) {
                cout << "No" << endl;
                return 0;
            }
            m[i] = kv.first;
            ++i;
        }

        if ((m[0] ^ m[1]) != m[2]) {
            cout << "No" << endl;
        } else {
            cout << "Yes" << endl;
        }
    }
}