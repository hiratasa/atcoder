#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t t;
    cin >> t;

    for (auto _ : irange(0L, t)) {
        int64_t n;
        cin >> n;

        array<vector<pair<int64_t, int64_t>>, 2> camels;
        int64_t s = 0;
        for (auto _ : irange(0L, n)) {
            int64_t k, l, r;
            cin >> k >> l >> r;

            auto d = l - r;
            if (d > 0) {
                s += r;
                camels[0].emplace_back(k, d);
            } else {
                s += l;
                camels[1].emplace_back(n - k, -d);
            }
        }

        sort(camels[0].begin(), camels[0].end());
        sort(camels[1].begin(), camels[1].end());

        for (const auto& camel : camels) {
            int64_t sp = 0;
            priority_queue<int64_t, vector<int64_t>, std::greater<>> q;
            for (const auto& c : camel) {
                int64_t np = q.size() + 1;
                if (c.first == 0) {
                    // NOP
                } else if (np > c.first) {
                    if (q.top() < c.second) {
                        sp += -q.top() + c.second;
                        q.pop();
                        q.push(c.second);
                    }
                } else {
                    q.push(c.second);
                    sp += c.second;
                }
            }

            s += sp;
        }

        cout << s << "\n";
    }
}