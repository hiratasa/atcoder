#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(int64_t n, string& s, int64_t i, char next) {
    if (i == n) {
        cout << s << endl;
        return;
    }

    for (char c = 'a'; c <= next; ++c) {
        s[i] = c;
        dfs(n, s, i + 1, max((char)(c + 1), next));
    }
}

int main() {
    int64_t n;
    cin >> n;

    string s;
    s.resize(n);
    dfs(n, s, 0, 'a');
}