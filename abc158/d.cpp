#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    int64_t q;
    cin >> q;

    string s2;
    int64_t m = 0;
    for (auto _ : irange(0L, q)) {
        int64_t t;
        cin >> t;

        if (t == 1) {
            ++m;
        } else {
            int64_t f;
            char c;
            cin >> f >> c;

            if ((f + m) % 2 == 0) {
                s.push_back(c);
            } else {
                s2.push_back(c);
            }
        }
    }

    if (m % 2 > 0) {
        swap(s, s2);
    }

    for (auto c : s2 | reversed) {
        cout << c;
    }
    cout << s << endl;
}