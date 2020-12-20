#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    --n;

    int64_t d = 26, k = 1;
    while (true) {
        if (n < d) {
            string s;

            for (auto i : irange(0L, k)) {
                s.push_back('a' + n % 26);
                n /= 26;
            }

            std::reverse(s.begin(), s.end());

            cout << s << endl;

            return 0;
        }

        n -= d;
        d *= 26;
        ++k;
    }
}