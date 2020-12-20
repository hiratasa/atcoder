#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    vector<int64_t> m;
    int64_t t = 0;
    for (auto i : irange(0uL, s.size()) | reversed) {
        auto c = s[i];

        switch (c) {
            case 'M':
                m.push_back(t);
                break;
            case '+':
                ++t;
                break;
            case '-':
                --t;
                break;
        }
    }

    sort(m.begin(), m.end());
    int64_t ans = -1 * accumulate(m.begin(), m.begin() + m.size() / 2, 0L) +
                  accumulate(m.begin() + m.size() / 2, m.end(), 0L);
    cout << ans << endl;
}