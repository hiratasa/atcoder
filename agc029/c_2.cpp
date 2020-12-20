#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

struct E;

E* next();

struct E {
    E() = default;

    void init(int64_t ll, int64_t rr, int64_t nn) {
        l = ll;
        r = rr;
        num = nn;
        left_child = nullptr;
        right_child = nullptr;
    }

    void update(int64_t a, int64_t b, int64_t m) {
        if (b <= l || a >= r) {
            return;
        }

        if (a <= l && r <= b) {
            num = m;
            return;
        }

        if (!left_child) {
            left_child = next();
            left_child->init(l, (l + r) / 2, num);
        }
        left_child->update(a, b, m);

        if (!right_child) {
            right_child = next();
            right_child->init((l + r) / 2, r, num);
        }
        right_child->update(a, b, m);

        num = -1;
    }

    int64_t get(int64_t index) const {
        if (!(l <= index && index < r)) {
            return -1;
        }

        if (num >= 0) {
            return num;
        }

        auto m = left_child->get(index);
        if (m >= 0) {
            return m;
        }

        m = right_child->get(index);
        return m;
    }

    int64_t num;
    int64_t l, r;
    E* left_child = nullptr;
    E* right_child = nullptr;
};

vector<vector<E>> pool;
pair<int, int> next_idx;

E* next() {
    constexpr auto block = 10000;

    if (next_idx.first >= pool.size()) {
        pool.resize(next_idx.first + 1);
        pool.back().resize(block);
    }

    auto* ret = &pool[next_idx.first][next_idx.second];

    ++next_idx.second;
    if (next_idx.second == block) {
        ++next_idx.first;
        next_idx.second = 0;
    }

    return ret;
}

void reset_pool() {
    next_idx.first = 0;
    next_idx.second = 0;
}

main() {
    int64_t n;
    cin >> n;

    // len
    vector<int64_t> a;
    int64_t prev = (1L << 30);
    int64_t m = 0;
    for (auto i : irange(0L, n)) {
        int64_t aa;
        cin >> aa;
        if (prev >= aa) {
            a.push_back(aa);
            m = max(aa, m);
        }

        prev = aa;
    }

    auto r = irange(1L, static_cast<int64_t>(a.size()) + 1L);
    auto it = partition_point(r.begin(), r.end(), [&a, m](int64_t t){
        reset_pool();
        E root;
        root.init(0, m + 1, 0);

        // prev len
        int64_t s = a[0];
        for (int64_t i_block = 1; i_block < a.size(); ++i_block) {
            auto len = a[i_block];

            if (s < len) {
                root.update(s, len, 0);
            }

            s = len;

            bool found = false;
            for (auto i = s - 1; i >= 0; --i) {
                auto v = root.get(i);
                if (v + 1 < t) {
                    root.update(i, i + 1, v + 1);
                    if (i + 1 != s) {
                        root.update(i + 1, s, 0);
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
