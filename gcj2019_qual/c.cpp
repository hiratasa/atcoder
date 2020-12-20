#include <bits/stdc++.h>

using namespace std;

// only positive integer
class BigInt {
  public:
    static constexpr auto D = std::numeric_limits<uint32_t>::max();
    static constexpr auto M = static_cast<uint64_t>(D) + 1ul;
    
    // for conversion between BigInt and string
    static constexpr auto W = 9;
    static constexpr auto SW = 1000000000;

    friend ostream& operator<<(ostream& os, const BigInt&);

  	BigInt() : vals(1, 0) {}  
    explicit BigInt(uint32_t n) : vals(1, n) {}
    explicit BigInt(const string& s) {
        // init to zero
        vals.resize(1);
        
        for (auto i = 0ul; i < (s.size() + W - 1) / W; ++i) {
            uint64_t b = ((s.size() > (i + 1) * W) ? s.size() - (i + 1) * W : 0);
            uint64_t l = ((s.size() > (i + 1) * W) ? W : s.size() - i * W);
            BigInt x(stoul(s.substr(b, l)));
            for (auto j = 0ul; j < i; ++j) {
                x *= SW;
            }
            *this += x;
        }
    }
    
    bool operator==(const BigInt& rhs) const {
        return vals == rhs.vals;
    }
    
    bool operator!=(const BigInt& rhs) const {
        return !(*this == rhs);
    }
    
    bool less(const BigInt& rhs, uint64_t offset = 0) const {
        if (vals.size() < rhs.vals.size() + offset) {
            return true;
        }
        
        if (vals.size() > rhs.vals.size() + offset) {
            return false;
        }
        
        for (auto i = 0ul; i < vals.size() - offset; ++i) {
            if (vals[vals.size() - 1 - i] > rhs.vals[rhs.vals.size() - 1 - i]) {
                return false;
            } else if (vals[vals.size() - 1 - i] < rhs.vals[rhs.vals.size() - 1 - i]) {
                return true;
            }
        }
        
        return false;
        
    }
    
    bool operator<(const BigInt& rhs) const {
        return this->less(rhs);
    }
    
    bool operator<=(const BigInt& rhs) const {
        return (*this == rhs || *this < rhs);
    }
    
    bool operator>(const BigInt& rhs) const {
        return !(*this <= rhs);
    }
    
    bool operator>=(const BigInt& rhs) const {
        return !(*this < rhs);
    }

    bool operator>(uint32_t rhs) const {
        if (vals.size() > 1) {
            return true;
        }

        return vals.back() > rhs;
    }
    
    void subtract_self(const BigInt& rhs, uint64_t offset = 0) {
        // We assume that *this >= rhs
        uint32_t carry = 0;
        for (auto i = 0ul; i < vals.size() - offset; ++i) {
            uint64_t rr = (i < rhs.vals.size() ? rhs.vals[i] : 0) + carry;
            if (vals[i + offset] >= rr) {
                vals[i + offset] -= rr;
                carry = 0;
            } else {
                carry = 1;
                vals[i + offset] = M + vals[i + offset] - rr;
            }
        }

        assert(carry == 0);
        
        while (vals.back() == 0 && vals.size() > 1) {
            vals.resize(vals.size() - 1);
        }
    }

    BigInt& operator-=(const BigInt& rhs) {
        subtract_self(rhs);
        return *this;
    }
    
    BigInt operator-(const BigInt& rhs) const {
        BigInt tmp = *this;
        tmp -= rhs;
        return tmp;
    }
    
    BigInt& operator+=(const BigInt& rhs) {
        uint32_t carry = 0;
        for (auto i = 0ul; i < vals.size(); ++i) {
            uint64_t s = static_cast<uint64_t>(vals[i]) + carry;
            if (i < rhs.vals.size()) {
                s += rhs.vals[i];
            }
            
            carry = s / M;
            s %= M;

            vals[i] = s;
        }

        auto prev_size = vals.size();
        if (vals.size() < rhs.vals.size()) {
            vals.resize(rhs.vals.size());
        }

        for (auto i = prev_size; i < rhs.vals.size(); ++i) {
            uint64_t s = static_cast<uint64_t>(carry) + rhs.vals[i];

            carry = s / M;
            s %= M;

            vals[i] = s;
        }
        
        if (carry > 0) {
            vals.push_back(carry);
        }

        return *this;
    }
    
    BigInt operator+(const BigInt& rhs) const {
        BigInt tmp = *this;
        tmp += rhs;
        return tmp;
    }

    BigInt multiply(uint32_t rhs, uint64_t offset = 0) const {
        BigInt ans(0);
        ans.vals.resize(offset + vals.size());

        uint32_t carry = 0;
        for (auto i = 0ul; i < vals.size(); ++i) {
            uint64_t s = static_cast<uint64_t>(vals[i]) * rhs + carry;
            
            carry = s / M;
            s %= M;
            ans.vals[offset + i] = s;
        }
        
        if (carry > 0) {
            ans.vals.push_back(carry);
        }
        
        return ans;
    }

    
    BigInt& operator*=(uint32_t rhs) {
        *this = this->multiply(rhs);
        return *this;
    }
    
    BigInt operator*(uint32_t rhs) const {
        return this->multiply(rhs);
    }
    
    BigInt operator*(const BigInt& rhs) const {
        BigInt ans(0);
        
        for (auto i = 0ul; i < rhs.vals.size(); ++i) {
            ans += this->multiply(rhs.vals[i], i);
        }
        
        return ans;
    }
    
    BigInt& operator*=(const BigInt& rhs) {
        *this = *this * rhs;
        return *this;
    }

    uint64_t debug() const {
        uint64_t x = 0;
        uint64_t b = 1;
        for (auto v : vals) {
            x += b * v;
            b *= 10;
        }

        return x;
    }

    string debug2() const {
        ostringstream ss;
        for (auto v : vals) {
            ss << v;
        }

        return ss.str();
    }
    
    pair<BigInt, BigInt> divide(const BigInt& rhs) const {
        BigInt ans(0);
        BigInt remains = *this;
        
        for (auto i = 0ul; i < vals.size(); ++i) {
            if (remains.less(rhs, vals.size() - 1 - i)) {
                continue;
            }
            
            // inclusive range
            uint64_t l = 1, r = D;
            while (l < r) {
                auto m = (l + r + 1) / 2;
                if (remains.less(rhs * m, vals.size() - 1 - i)) {
                    // m itself is not the target
                    r = m - 1;
                } else {
                    l = m;
                }
            }
            
            if (ans.vals.size() == 1 && ans.vals.back() == 0) {
                // fix ans's length
                ans.vals.resize(vals.size() - i);
            }
            ans.vals[vals.size() - 1 - i] = l;
            remains.subtract_self(rhs * l, vals.size() - 1 - i);
        }
        
        return make_pair(ans, remains);
    }
    
    BigInt& operator/=(const BigInt& rhs) {
        *this = this->divide(rhs).first;
        return *this;
    }
    
    BigInt operator/(const BigInt& rhs) const {
        return this->divide(rhs).first;
    }
    
    BigInt& operator%=(const BigInt& rhs) {
        *this = this->divide(rhs).second;
        return *this;
    }
    
    BigInt operator%(const BigInt& rhs) const {
        return this->divide(rhs).second;
    }
    
    string to_string() const {
        auto tmp = *this;
    
        string str;
        while (tmp > BigInt(0)) {
            ostringstream oss;
            oss << std::setw(W) << std::setfill('0');

            auto p = tmp.divide(BigInt(SW));
            oss << p.second.vals[0];
            str = oss.str() + str;
            tmp = move(p.first);
        }
        
        if (str.size() == 0) {
            return "0";
        }
        
        for (auto i = 0ul; i < str.size(); ++i) {
            if (str[i] != '0') {
                return str.substr(i);
            }
        }
        
        return "0";
    }

  private:
    friend void swap(BigInt& lhs, BigInt& rhs);

    vector<uint32_t> vals;
};

void swap(BigInt& lhs, BigInt& rhs) {
    lhs.vals.swap(rhs.vals);
}

ostream& operator<<(ostream& os, const BigInt& v) {
    return os << v.to_string();
}

istream& operator>>(istream& is, BigInt& v) {
    string s;
    auto&& ret = cin >> s;
    v = BigInt(s);
    return ret;
}

template <typename T>
T gcd(T lhs, T rhs) {
    while (rhs > 0) {
        lhs %= rhs;
        swap(lhs, rhs);
    }
    
    return lhs;
}

main() {
    int64_t t;
    cin >> t;
    
    using Int = BigInt;
    for (auto i = 0l; i < t; ++i) {
        string n;
        int64_t l;
        cin >> n >> l;
        
        int32_t num_leading_same = 0;
        Int prev(0);
        vector<Int> factors;
        for (auto i = 0; i < l; ++i) {
            Int m;
            cin >> m;
            if (prev > 0) {
                if (prev == m && factors.empty()) {
                    ++num_leading_same;
                    continue;
                }

                auto g = gcd(prev, m);
                if (factors.empty()) {
                    // at the first
                    factors.push_back(prev / g);
                }
                factors.push_back(g);
                // To prevent confusing case when two primes appear alternatively
                prev = m / factors.back();
            } else {
                prev = m;
            }
        }
        factors.push_back(prev);

        auto unique_factors = factors;
        sort(unique_factors.begin(), unique_factors.end());
        unique_factors.erase(unique(unique_factors.begin(), unique_factors.end()), unique_factors.end());
        
        cout << "Case #" << i + 1 << ": ";
        for (auto i = 0; i < num_leading_same; ++i) {
            auto it = lower_bound(unique_factors.begin(), unique_factors.end(), factors[(num_leading_same - i) % 2]);
            cout << static_cast<char>('A' + (it - unique_factors.begin()));
        }

        for (const auto& f : factors) {
            auto it = lower_bound(unique_factors.begin(), unique_factors.end(), f);
            cout << static_cast<char>('A' + (it - unique_factors.begin()));
        }
        cout << endl;
    }
}