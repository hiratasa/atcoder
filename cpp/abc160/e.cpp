#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x, y, a, b, c;
    cin >> x >> y >> a >> b >> c;

    vector<int64_t> p(a), q(b), r(c);
    for (auto&& pp : p) {
        cin >> pp;
    }
    for (auto&& qq : q) {
        cin >> qq;
    }
    for (auto&& rr : r) {
        cin >> rr;
    }

    constexpr auto INF = numeric_limits<int64_t>::max();
    p.push_back(INF);
    q.push_back(INF);
    sort(p.rbegin(), p.rend());
    sort(q.rbegin(), q.rend());
    sort(r.rbegin(), r.rend());
    p.resize(x + 1);
    q.resize(y + 1);
    int64_t s = 0;
    for (auto i : irange(0L, c)) {
        if (p.back() < q.back()) {
            if (p.back() >= r[i]) {
                break;
            }
            p.pop_back();
        } else {
            if (q.back() >= r[i]) {
                break;
            }
            q.pop_back();
        }
        s += r[i];
    }

    s += accumulate(p.begin() + 1, p.end(), 0L);
    s += accumulate(q.begin() + 1, q.end(), 0L);

    cout << s << endl;
}