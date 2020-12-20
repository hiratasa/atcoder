#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

using BS = bitset<64>;

// copy a
bool check(vector<BS> a, const string& s, int64_t idx) {
    assert(s[idx] == '1');

    int64_t n = s.size();

    BS b = a[idx];
    for (auto i : irange(0L, 64L)) {
        BS t;
        bool found = false;
        for (auto j : irange(idx, n)) {
            if (s[j] == '1') {
                continue;
            }

            if (a[j][i]) {
                if (!found) {
                    found = true;
                    t = a[j];
                    if (b[i]) {
                        b ^= t;
                    }
                }

                a[j] ^= t;
            }
        }

        if (b[i]) {
            return false;
        }
    }

    return true;
}

bool check(const vector<BS>& a, const string& s) {
    int64_t n = s.size();
    for (auto i : irange(0L, n)) {
        if (s[i] == '1') {
            if (!check(a, s, i)) {
                return false;
            }
        }
    }

    return true;
}

int main() {
    int64_t t;
    cin >> t;

    for (auto _ : irange(0L, t)) {
        int64_t n;
        cin >> n;

        vector<bitset<64>> a(n);
        for (auto i : irange(0L, n)) {
            uint64_t aa;
            cin >> aa;
            a[i] = aa;
        }

        string s;
        cin >> s;

        if (!check(a, s)) {
            cout << 1 << endl;
        } else {
            cout << 0 << endl;
        }
    }
}