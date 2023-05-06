#include <bits/stdc++.h>

using namespace std;

enum {
    R = 0,
    P = 1,
    S = 2,
};

enum class Status {
    kSame, kWin, kLose,
};

int Encode(char c) {
    switch(c) {
        case 'R':
            return R;
        case 'P':
            return P;
        case 'S':
            return S;
    }
}

vector<int> Encode(const string& c) {
    vector<int> v;
    v.reserve(c.size());

    for (auto cc : c) {
        v.push_back(Encode(cc));
    }

    return v;
}

char Decode(int c) {
    switch(c) {
        case 0:
            return 'R';
        case 1:
            return 'P';
        case 2:
            return 'S';
    }
}

string Decode(const vector<int>& c) {
    string v;
    v.reserve(c.size());

    for (auto cc : c) {
        v.push_back(Decode(cc));
    }

    return v;
}

int64_t Fix(vector<vector<int>>& hands) {
    size_t max_size = max_element(hands.begin(), hands.end(), [](const vector<int>& lhs, const vector<int>& rhs) {
        return lhs.size() < rhs.size();
    })->size();

    max_size = max(max_size, hands.size());

    for (auto&& h : hands) {
        int64_t len = h.size();
        h.resize(max_size);
        for (int64_t i = len; i < max_size; ++i) {
            h[i] = h[i % len];
        }
    }

    return max_size;
}

string TestCase(const vector<string>& cs) {
    const auto kImpossible = "IMPOSSIBLE";

    vector<vector<int>> other_hands(cs.size());
    transform(cs.begin(), cs.end(), other_hands.begin(), [](const auto& c) {
        return Encode(c);
    });

    int64_t max_turn = Fix(other_hands);

    sort(other_hands.begin(), other_hands.end());

    std::vector<int> hands;
    hands.reserve(max_turn);
    int64_t begin = 0, end = other_hands.size();
    for (int64_t i_turn = 0; i_turn < max_turn; ++i_turn) {
        array<int64_t, 3> index{-1, -1, -1};

        for (auto i = end - 1; i >= begin; --i) {
            index[other_hands[i][i_turn]] = i;
        }

        auto num_hands = count_if(index.begin(), index.end(), [](auto idx) {
            return idx >= 0;
        });

        if (num_hands == 1) {
            auto cur_hand = max_element(index.begin(), index.end()) - index.begin();
            hands.push_back((cur_hand + 1) % 3);
            end = begin;
            break;
        } else if (num_hands == 2) {
            auto n = min_element(index.begin(), index.end()) - index.begin();
            auto next_hand = (n + 2) % 3;
            hands.push_back(next_hand);

            begin = index[next_hand];
            if (next_hand == 0) {
                end = index[2];
            }
        } else if (num_hands == 3) {
            return kImpossible;
        }
    }

    if (begin < end) {
        return kImpossible;
    }

    return Decode(hands);
}

int main() {
    int64_t t;
    cin >> t;

    for (auto i = 0L; i < t; ++i) {
        int64_t a;
        cin >> a;

        vector<string> cs(a);
        for (auto&& c : cs) {
            cin >> c;
        }

        auto ans = TestCase(cs);
        cout << "Case #" << i + 1 << ": " << ans << endl;
    }

    return 0;
}