#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

using bs = bitset<17>;

int64_t count1(int64_t k) {
    int64_t ret = 0;
    while (k > 0) {
        if ((k & 1) > 0) {
            ++ret;
        }
        k /= 2;
    }

    return ret;
}

void exec(bs& x, const bs& y, vector<bool>& active, int64_t n) {
    int64_t pos = 0;
    while (pos < n && !(active[pos] && x[pos] != y[pos])) {
        ++pos;
    }

    assert(pos != n);

    active[pos] = false;
    int64_t next_pos = 0;
    while (next_pos < n && !active[next_pos]) {
        ++next_pos;
    }

    if (next_pos == n) {
        x.flip(pos);
        cout << " " << x.to_ullong();
        active[pos] = true;
        return;
    }

    auto z = x;
    z.flip(next_pos);
    exec(x, z, active, n);
    x.flip(pos);
    cout << " " << x.to_ullong();
    exec(x, y, active, n);

    active[pos] = true;
}

main(){
    int64_t n, a, b;
    cin >> n >> a >> b;

    auto ca1 = count1(a);
    auto cb1 = count1(b);
    if ((ca1 % 2) == (cb1 % 2)) {
        cout << "NO" << endl;
        return 0;
    }

    cout << "YES" << endl;

    bitset<17> x(a), y(b);
    vector<bool> active(n, true);
    cout << a;
    exec(x, y, active, n);
    cout << endl;
}