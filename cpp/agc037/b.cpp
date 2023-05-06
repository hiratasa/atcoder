#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t color_index(char c) {
    switch (c) {
        case 'R':
            return 0;
        case 'G':
            return 1;
        case 'B':
            return 2;
    }
}

main() {
    constexpr auto M = 998244353;

    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    vector<int64_t> nums(8, 0), nums2;
    int64_t k = 1;
    for (auto c : s) {
        auto idx = color_index(c);

        nums2 = nums;
        if (nums[3] == 0 && nums[5] == 0 && nums[6] == 0) {
            if (nums[1] == 0 && nums[2] == 0 && nums[4] == 0) {
                ++nums2[1u << idx];
            } else if (nums[1u << idx] > 0) {
                ++nums2[1u << idx];
            } else {
                for (auto i : {1, 2, 4}) {
                    if (nums[i] > 0) {
                        ++nums2[i + (1u << idx)];
                        --nums2[i];
                        k = (k * nums[i]) % M;
                    }
                }
            }
        } else {
            if (nums[7 - (1u << idx)] > 0) {
                ++nums2[7];
                --nums2[7 - (1u << idx)];
                k = (k * nums[7 - (1u << idx)]) % M;
            } else if (nums[1] == 0 && nums[2] == 0 && nums[4] == 0) {
                ++nums2[1u << idx];
            } else if (nums[1u << idx] > 0) {
                ++nums2[1u << idx];
            } else {
                for (auto i : {1, 2, 4}) {
                    if (nums[i] > 0) {
                        ++nums2[i + (1u << idx)];
                        --nums2[i];
                        k = (k * nums[i]) % M;
                    }
                }
            }
        }

        nums.swap(nums2);
        nums2.clear();
    }

    for (auto i : irange(1L, n + 1)) {
        k = (k * i) % M;
    }

    cout << k << endl;
}