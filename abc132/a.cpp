#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    unordered_map<char, int> nums;
    for (auto c : s) {
        ++nums[c];
    }

    if (nums.size() == 2 && nums.begin()->second == 2) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}