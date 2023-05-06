#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    if (w == 1 || h == 1) {
        cout << 1 << endl;
        return 0;
    }

    cout << /* even row */ ((h + 1) / 2) * ((w + 1) / 2) +
                    /* odd row */ ((h / 2) * (w / 2))
         << endl;
}