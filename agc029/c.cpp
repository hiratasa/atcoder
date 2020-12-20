#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    // len
    vector<int64_t> a;
    int64_t prev = (1L << 30);
    for (auto i : irange(0L, n)) {
        int64_t aa;
        cin >> aa;
        if (prev >= aa) {
            a.push_back(aa);
        }

        prev = aa;
    }

    auto r = irange(1L, static_cast<int64_t>(a.size()) + 1L);
    auto it = partition_point(r.begin(), r.end(), [&a](int64_t t){
        // len, char
        vector<pair<int64_t, int64_t>> seq;

        seq.emplace_back(a[0], 0);
        for (int64_t i_block = 1; i_block < a.size(); ++i_block) {
            auto len = a[i_block];

#if 0
            cerr << "[" << t << "]" << len << endl;
            for (const auto& kv : seq) {
                cerr << kv.first << ", " << kv.second << endl;
            }
#endif
            if (seq.back().first < len) {
                if (seq.back().second == 0) {
                    seq.back().first = len;
                } else {
                    seq.emplace_back(len, 0);
                }
            }

            if (seq.back().first > len) {
                auto ii = partition_point(seq.begin(), seq.end(), [len](const pair<int64_t, int64_t>& pp) {
                    return pp.first < len;
                });
                seq.erase(ii + 1, seq.end());
                seq.back().first = len;
            }

            bool found = false;
            for (int64_t i = seq.size() - 1; i >= 0; --i) {
                auto&& block = seq[i];
                if (block.second + 1 < t) {
                    auto blocklen = (i == 0 ? block.first : block.first - seq[i-1].first);

                    seq.resize(i + 1);
                    if (blocklen == 1) {
                        ++seq[i].second;
                    } else {
                        --seq[i].first;
                        seq.emplace_back(seq[i].first + 1, seq[i].second + 1);
                    }

                    if (seq.back().first < len) {
                        seq.emplace_back(len, 0);
                    }

                    found = true;
                    break;
                }
            }

            if (!found) {
                return true;
            }
        }

        return false;
    });

    cout << *it << endl;
}
