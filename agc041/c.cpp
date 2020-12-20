#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void foo4(int64_t h, int64_t w) {
    // clang-format off
    string str[] = {
        "aacd",
        "bbcd",
        "efgg",
        "efhh",
    };
    // clang-format on

    assert(h % 4 == 0);
    int64_t offset = 4;
    w -= 4;
    cout.fill('.');
    for (auto _ : irange(0L, h / 4)) {
        cout << setw(offset) << str[0] << setw(w) << "" << endl;
        ;
        cout << setw(offset) << str[1] << setw(w) << "" << endl;
        ;
        cout << setw(offset) << str[2] << setw(w) << "" << endl;
        ;
        cout << setw(offset) << str[3] << setw(w) << "" << endl;
        ;

        offset += 4;
        w -= 4;
    }
}

void foo5(int64_t w) {
    // clang-format off
    string str[] = {
        "aabbc",
        "hii.c",
        "h..jd",
        "g..jd",
        "gffee"
    };
    // clang-format on

    cout.fill('.');

    cout << setw(w) << str[0] << endl;
    cout << setw(w) << str[1] << endl;
    cout << setw(w) << str[2] << endl;
    cout << setw(w) << str[3] << endl;
    cout << setw(w) << str[4] << endl;
}

void foo6(int64_t w) {
    // clang-format off
    string str[] = {
        ".aaccd",
        "b..f.d",
        "b..fee",
        "ccd.aa",
        "f.db..",
        "feeb..",
    };
    // clang-format on

    cout.fill('.');

    cout << setw(w) << str[0] << endl;
    cout << setw(w) << str[1] << endl;
    cout << setw(w) << str[2] << endl;
    cout << setw(w) << str[3] << endl;
    cout << setw(w) << str[4] << endl;
    cout << setw(w) << str[5] << endl;
}

void foo7(int64_t w) {
    // clang-format off
    string str[] = {
        ".aadcc.",
        "e..d..b",
        "e.ff..b",
        "aa..eaa",
        "d...e.d",
        "d..a..d",
        ".ffacc.",
    };
    // clang-format on

    cout.fill('.');

    cout << setw(w) << str[0] << endl;
    cout << setw(w) << str[1] << endl;
    cout << setw(w) << str[2] << endl;
    cout << setw(w) << str[3] << endl;
    cout << setw(w) << str[4] << endl;
    cout << setw(w) << str[5] << endl;
    cout << setw(w) << str[6] << endl;
}

int main() {
    int64_t n;
    cin >> n;

    if (n == 2) {
        cout << -1 << endl;
        return 0;
    }

    if (n == 3) {
        cout << "aa." << endl;
        cout << "..b" << endl;
        cout << "..b" << endl;
        return 0;
    }

    if (n % 4 == 0) {
        foo4(n, n);
    } else if (n % 4 == 1) {
        foo4(n - 5, n);
        foo5(n);
    } else if (n % 4 == 2) {
        foo4(n - 6, n);
        foo6(n);
    } else if (n % 4 == 3) {
        foo4(n - 7, n);
        foo7(n);
    }
}