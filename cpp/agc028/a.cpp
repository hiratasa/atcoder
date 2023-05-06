#include <iostream>
#include <vector>
#include <utility>
#include <cmath>
#include <cassert>
#include <cstdint>

using namespace std;

int64_t gcd(int64_t n, int64_t m) {
    if (n > m) {
        return gcd(m, n);
    }

    assert(n <= m);

    if (n == 0) {
        return m;
    }

    return gcd(m % n, n);
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    string s, t;
    cin >> s >> t;

    assert(n == s.size());
    assert(m == t.size());

    int64_t g = gcd(n, m);
    int64_t a = n / g;
    int64_t b = m / g;

    // n = a * g
    // m = b * g
    // nのi*a番目の文字はmのi*b番目の文字と一致していなければならない
    for (int64_t i = 0; i < g; ++i) {
        if (s[i * a] != t[i * b]) {
            cout << "-1" << endl;
            return 0;
        }
    }

    cout << a * b * g << endl;

    return 0;
}