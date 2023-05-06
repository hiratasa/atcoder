#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, q;
    cin >> n >> q;

    vector<int64_t> rate(n), k(n);
    vector<multiset<int64_t>> s(200001L);
    for (auto i : irange(0L, n)) {
        int64_t a, b;
        cin >> a >> b;
        rate[i] = a;
        k[i] = b;
        s[b].insert(a);
    }

    multiset<int64_t> eq;
    for (const auto& ss : s) {
        if (ss.empty()) {
            continue;
        }

        eq.insert(*ss.rbegin());
    }

    for (auto _ : irange(0L, q)) {
        int64_t c, d;
        cin >> c >> d;
        --c;

        auto pre = k[c];

        eq.erase(eq.find(*s[pre].rbegin()));
        s[pre].erase(s[pre].find(rate[c]));
        if (!s[pre].empty()) {
            eq.insert(*s[pre].rbegin());
        }
        k[c] = d;
        if (!s[d].empty()) {
            eq.erase(eq.find(*s[d].rbegin()));
        }
        s[d].insert(rate[c]);
        eq.insert(*s[d].rbegin());

        cout << *eq.begin() << "\n";
    }
}