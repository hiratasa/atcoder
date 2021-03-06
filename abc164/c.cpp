#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_set<string> s;
    for (auto i : irange(0L, n)) {
        string ss;
        cin >> ss;
        s.insert(ss);
    }

    cout << s.size() << endl;
}