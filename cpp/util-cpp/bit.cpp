#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

// BIT
//  区間和と単一要素への加算が高速にできる

//  1-indexedで、
//    iの最後に立っているビットをB（=i&-i)として、
//    s_iは [i - (i&-i) + 1, i] の区間の和を保持
class BIT {
   public:
    explicit BIT(int64_t n) : n(n), b(n + 1) {}

    // [0, i)の和
    int64_t sum(int64_t i) const {
        int64_t s = 0;

        // b[1] ~ b[i] の和
        // (bは1-indexedなのでこれでOK)
        while (i > 0) {
            s += b[i];
            i -= (i & -i);
        }

        return s;
    }

    // [i, j) の和
    int64_t sum(int64_t i, int64_t j) const { return sum(j) - sum(i); }

    void add(int64_t i, int64_t a) {
        // 1-indexedに直す
        ++i;

        while (i <= n) {
            b[i] += a;
            i += (i & -i);
        }
    }

   private:
    int64_t n;
    vector<int64_t> b;
};