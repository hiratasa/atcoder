#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool solve(const char* c, const char* c1, int64_t n, int64_t fn, int64_t fn1,
           vector<bool>& u) {
    if (n == 0) {
        return true;
    }

    if (n == 1) {
        return *c == 'b';
    }

    if (n == 2) {
        return *c == 'a';
    }

    if (n == 3) {
        if (c1 != nullptr && *c1 != 'a') {
            return false;
        }
        if (*c == 'a') {
            u.push_back(0);
            return true;
        } else {
            u.push_back(1);
            return true;
        }
    }

    // clang-format off
    // F(n, 4m) = F(n - 1, 2m) + F(n - 2, m) = F(n - 2, m) + F(n - 3, m/2) + F(n - 2, m)
    // F(n, 4m + 1) = F(n - 2, m) + F(n - 1, 2m) = F(n - 2, m) + F(n - 2, m) + F(n - 3, m/2)
    // F(n, 4m + 2) = F(n - 1, 2m + 1) + F(n - 2, m) = F(n - 3, m/2) + F(n - 2, m) + F(n - 2, m)
    // F(n, 4m + 3) = F(n - 2, m) + F(n - 1, 2m + 1) = F(n - 2, m) + F(n - 3, m/2) + F(n - 2, m)
    // clang-format on

    int64_t fn2 = fn - fn1;
    int64_t fn3 = fn1 - fn2;
    if (std::equal(c, c + fn2, c + fn1) &&
        (c1 == nullptr || std::equal(c, c + fn2, c1))) {
        u.push_back(0);
        u.push_back(0);
        bool ok = solve(c, c + fn2, n - 2, fn2, fn3, u);
        if (ok) {
            return true;
        }
        u.pop_back();
        u.pop_back();
    }
    if (std::equal(c, c + fn2, c + fn2) &&
        (c1 == nullptr || std::equal(c + fn2, c + fn, c1))) {
        u.push_back(1);
        u.push_back(0);
        bool ok = solve(c, c + 2 * fn2, n - 2, fn2, fn3, u);
        if (ok) {
            return true;
        }
        u.pop_back();
        u.pop_back();
    }
    if (std::equal(c + fn3, c + fn1, c + fn1) &&
        (c1 == nullptr || std::equal(c, c + fn1, c1))) {
        u.push_back(0);
        u.push_back(1);
        bool ok = solve(c + fn3, c, n - 2, fn2, fn3, u);
        if (ok) {
            return true;
        }
        u.pop_back();
        u.pop_back();
    }

    return false;
}

int main() {
    string s;
    cin >> s;

    if (s.size() == 1) {
        if (s[0] == 'b') {
            cout << "1 0" << endl;
        } else {
            cout << "2 0" << endl;
        }
        return 0;
    }

    vector<int64_t> f(30);
    int64_t n = -1;
    f[1] = f[2] = 1;
    for (auto i : irange(3L, 30L)) {
        f[i] = f[i - 1] + f[i - 2];

        if (f[i] == s.size()) {
            n = i;
            break;
        }
    }

    vector<bool> u;
    solve(&s[0], nullptr, n, f[n], f[n - 1], u);

    int64_t ans = 0;
    for (auto b : u | reversed) {
        ans *= 2;
        ans += b;
    }

    cout << n << " " << ans << endl;
}
