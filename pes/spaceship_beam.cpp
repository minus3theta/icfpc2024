#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")

#define NDEBUG

#include <bits/stdc++.h>
#include <queue>
#include <map>
#include <set>
#include <vector>
#include <string>
#include <iostream>

using namespace std;

namespace atcoder {

namespace internal {

// @return same with std::bit::bit_ceil
unsigned int bit_ceil(unsigned int n) {
    unsigned int x = 1;
    while (x < (unsigned int)(n)) x *= 2;
    return x;
}

// @param n `1 <= n`
// @return same with std::bit::countr_zero
int countr_zero(unsigned int n) {
#ifdef _MSC_VER
    unsigned long index;
    _BitScanForward(&index, n);
    return index;
#else
    return __builtin_ctz(n);
#endif
}

// @param n `1 <= n`
// @return same with std::bit::countr_zero
constexpr int countr_zero_constexpr(unsigned int n) {
    int x = 0;
    while (!(n & (1 << x))) x++;
    return x;
}

}  // namespace internal



template <class S, auto op, auto e> struct segtree {
    static_assert(std::is_convertible_v<decltype(op), std::function<S(S, S)>>,
                  "op must work as S(S, S)");
    static_assert(std::is_convertible_v<decltype(e), std::function<S()>>,
                  "e must work as S()");

public:
    segtree() : segtree(0) {}
    explicit segtree(int n) : segtree(std::vector<S>(n, e())) {}
    explicit segtree(const std::vector<S>& v) : _n(int(v.size())) {
        size = (int)internal::bit_ceil((unsigned int)(_n));
        log = internal::countr_zero((unsigned int)size);
        d = std::vector<S>(2 * size, e());
        for (int i = 0; i < _n; i++) d[size + i] = v[i];
        for (int i = size - 1; i >= 1; i--) {
            update(i);
        }
    }

    void set(int p, S x) {
        assert(0 <= p && p < _n);
        p += size;
        d[p] = x;
        for (int i = 1; i <= log; i++) update(p >> i);
    }

    S get(int p) const {
        assert(0 <= p && p < _n);
        return d[p + size];
    }

    S prod(int l, int r) const {
        assert(0 <= l && l <= r && r <= _n);
        S sml = e(), smr = e();
        l += size;
        r += size;

        while (l < r) {
            if (l & 1) sml = op(sml, d[l++]);
            if (r & 1) smr = op(d[--r], smr);
            l >>= 1;
            r >>= 1;
        }
        return op(sml, smr);
    }

    S all_prod() const { return d[1]; }

    template <bool (*f)(S)> int max_right(int l) const {
        return max_right(l, [](S x) { return f(x); });
    }
    template <class F> int max_right(int l, F f) const {
        assert(0 <= l && l <= _n);
        assert(f(e()));
        if (l == _n) return _n;
        l += size;
        S sm = e();
        do {
            while (l % 2 == 0) l >>= 1;
            if (!f(op(sm, d[l]))) {
                while (l < size) {
                    l = (2 * l);
                    if (f(op(sm, d[l]))) {
                        sm = op(sm, d[l]);
                        l++;
                    }
                }
                return l - size;
            }
            sm = op(sm, d[l]);
            l++;
        } while ((l & -l) != l);
        return _n;
    }

    template <bool (*f)(S)> int min_left(int r) const {
        return min_left(r, [](S x) { return f(x); });
    }
    template <class F> int min_left(int r, F f) const {
        assert(0 <= r && r <= _n);
        assert(f(e()));
        if (r == 0) return 0;
        r += size;
        S sm = e();
        do {
            r--;
            while (r > 1 && (r % 2)) r >>= 1;
            if (!f(op(d[r], sm))) {
                while (r < size) {
                    r = (2 * r + 1);
                    if (f(op(d[r], sm))) {
                        sm = op(d[r], sm);
                        r--;
                    }
                }
                return r + 1 - size;
            }
            sm = op(d[r], sm);
        } while ((r & -r) != r);
        return 0;
    }

private:
    int _n, size, log;
    std::vector<S> d;

    void update(int k) { d[k] = op(d[2 * k], d[2 * k + 1]); }
};

}  // namespace atcoder

//------------------------------------------------------------------------------
class XorShift {
public:
    using result_type = uint32_t;
    explicit XorShift(result_type seed){ init(seed); }
    void init(result_type s){
        x = 1812433253U * (s ^ (s >> 30));
        y = 1812433253U * (x ^ (x >> 30)) + 1;
        z = 1812433253U * (y ^ (y >> 30)) + 2;
        w = 1812433253U * (z ^ (z >> 30)) + 3;
    }
    static constexpr result_type min() { return numeric_limits<result_type>::min(); }
    static constexpr result_type max() { return numeric_limits<result_type>::max(); }
    result_type operator() () {
        result_type t = x ^ (x << 11);
        x = y; y = z; z = w;
        return w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));
    }
private:
    result_type x;
    result_type y;
    result_type z;
    result_type w;
};

XorShift rnd(1234567891);

//------------------------------------------------------------------------------
map<pair<int,int>, uint32_t> target_hash;
map<pair<int,int>, int> target_idx;
vector<pair<int,int>> idx_to_target;

struct Input {
    set<pair<int, int>> target;
    bool has_origin = false;

    void input() {
        int x, y;
        target_idx[make_pair(0,0)] = 0;
        idx_to_target.push_back(make_pair(0, 0));
        while(cin >> x >> y){
            if(x == 0 && y == 0){
                has_origin = true;
                continue;
            }
            auto p = make_pair(x, y);
            if(target.count(p)) continue;
            target.insert(p);
            target_hash[p] = rnd();
            int s = target_idx.size();
            target_idx[p] = s;
            idx_to_target.push_back(p);
        }
    }
};

//------------------------------------------------------------------------------
class Timer {
public:
    explicit Timer()
        : mStart(chrono::system_clock::now())
    {}
    void start() { mStart = chrono::system_clock::now(); }
    double msec() const {
        auto t = chrono::system_clock::now();
        return 1e-3 * chrono::duration_cast<std::chrono::microseconds>(t - mStart).count();
    }
private:
    chrono::system_clock::time_point mStart;
};

//------------------------------------------------------------------------------
vector<pair<int, string>> small_moves[13][200];

void make_small_moves(){
    auto qu = queue<pair<pair<int, int>, string>>();
    qu.push({{0, 0}, ""});

    for(int i=0;i<13;i++){
        auto visited = set<pair<int, int>>();
        auto next_qu = queue<pair<pair<int, int>, string>>();
        while(!qu.empty()){
            auto [p, s] = qu.front(); qu.pop();
            for(int dv=-1;dv<=1;dv++){
                int nv = p.second + dv;
                int np = p.first + nv;
                if(visited.count({np, nv})) continue;
                visited.insert({np, nv});
                string ns = s;
                ns.push_back('1' + dv);
                small_moves[i][np + 100].push_back({nv + 100, ns});
                next_qu.push({{np, nv}, ns});
            }
        }
        qu = next_qu;
    }
}


//------------------------------------------------------------------------------
namespace beam_search {

// ビームサーチの設定
struct Config {
    int max_turn;
    size_t beam_width;
    size_t tour_capacity;
    uint32_t hash_map_capacity;
};

// 連想配列
// Keyにハッシュ関数を適用しない
// open addressing with linear probing
// unordered_mapよりも速い
// nは格納する要素数よりも16倍ほど大きくする
template <class Key, class T>
struct HashMap {
    public:
        explicit HashMap(uint32_t n) {
            if (n % 2 == 0) {
                ++n;
            }
            n_ = n;
            valid_.resize(n_, false);
            data_.resize(n_);
        }

        // 戻り値
        // - 存在するならtrue、存在しないならfalse
        // - index
        pair<bool,int> get_index(Key key) const {
            Key i = key % n_;
            while (valid_[i]) {
                if (data_[i].first == key) {
                    return {true, i};
                }
                if (++i == n_) {
                    i = 0;
                }
            }
            return {false, i};
        }

        // 指定したindexにkeyとvalueを格納する
        void set(int i, Key key, T value) {
            valid_[i] = true;
            data_[i] = {key, value};
        }

        // 指定したindexのvalueを返す
        T get(int i) const {
            assert(valid_[i]);
            return data_[i].second;
        }

        void clear() {
            fill(valid_.begin(), valid_.end(), false);
        }

    private:
        uint32_t n_;
        vector<bool> valid_;
        vector<pair<Key,T>> data_;
};

using Hash = uint64_t;

inline Hash calc_hash(int pos_x, int pos_y, int vel_x, int vel_y, set<pair<int,int>>& target, pair<int,int> pass = make_pair(0, 0)) {
    long long a = (long long)target_idx[make_pair(pos_x, pos_y)] << 32;
    for(auto s : target){
        if(s == pass) continue;
        a ^= target_hash[s];
    }
    long long b = (1LL << 32) * vel_x + vel_y;
    return a^b;
}

// 状態遷移を行うために必要な情報
// メモリ使用量をできるだけ小さくしてください
struct Action {
    int32_t pidx0;
    int32_t pidx1;
    int32_t vdif0;
    int32_t vdif1;

    Action(int _pidx0, int _pidx1, int _vdif0, int _vdif1){
        pidx0 = _pidx0;
        pidx1 = _pidx1;
        vdif0 = _vdif0;
        vdif1 = _vdif1;
    }

    tuple<int,int,int,int> decode() const {
        return {pidx0, pidx1, vdif0, vdif1};
    }

    bool operator==(const Action& other) const {
        return pidx0 == other.pidx0 && pidx1 == other.pidx1 && vdif0 == other.vdif0 && vdif1 == other.vdif1;
    }
};

using Cost = long long;

// 状態のコストを評価するための構造体
// メモリ使用量をできるだけ小さくしてください
struct Evaluator {
    Cost score;

    Evaluator(Cost score) : score(score) {}

    // 低いほどよい
    Cost evaluate() const {
        return score;
    }
};

// 展開するノードの候補を表す構造体
struct Candidate {
    Action action;
    Evaluator evaluator;
    Hash hash;
    int parent;

    Candidate(Action action, Evaluator evaluator, Hash hash, int parent) :
        action(action),
        evaluator(evaluator),
        hash(hash),
        parent(parent) {}
};

// ノードの候補から実際に追加するものを選ぶクラス
// ビーム幅の個数だけ、評価がよいものを選ぶ
// ハッシュ値が一致したものについては、評価がよいほうのみを残す
class Selector {
    public:
        explicit Selector(const Config& config) :
            hash_to_index_(config.hash_map_capacity)
        {
            beam_width = config.beam_width;
            candidates_.reserve(beam_width);
            full_ = false;

            costs_.resize(beam_width);
            for (size_t i = 0; i < beam_width; ++i) {
                costs_[i] = {0, i};
            }
        }

        // 候補を追加する
        // ターン数最小化型の問題で、candidateによって実行可能解が得られる場合にのみ finished = true とする
        // ビーム幅分の候補をCandidateを追加したときにsegment treeを構築する
        void push(const Candidate& candidate, bool finished) {
            if (finished) {
                finished_candidates_.emplace_back(candidate);
                return;
            }
            Cost cost = candidate.evaluator.evaluate();
            if (full_ && cost >= st_.all_prod().first) {
                // 保持しているどの候補よりもコストが小さくないとき
                return;
            }
            auto [valid, i] = hash_to_index_.get_index(candidate.hash);

            if (valid) {
                int j = hash_to_index_.get(i);
                if (candidate.hash == candidates_[j].hash) {
                    // ハッシュ値が等しいものが存在しているとき
                    if (full_) {
                        // segment treeが構築されている場合
                        if (cost < st_.get(j).first) {
                            candidates_[j] = candidate;
                            st_.set(j, {cost, j});
                        }
                    } else {
                        // segment treeが構築されていない場合
                        if (cost < costs_[j].first) {
                            candidates_[j] = candidate;
                            costs_[j].first = cost;
                        }
                    }
                    return;
                }
            }
            if (full_) {
                // segment treeが構築されている場合
                int j = st_.all_prod().second;
                hash_to_index_.set(i, candidate.hash, j);
                candidates_[j] = candidate;
                st_.set(j, {cost, j});
            } else {
                // segment treeが構築されていない場合
                int j = candidates_.size();
                hash_to_index_.set(i, candidate.hash, j);
                candidates_.emplace_back(candidate);
                costs_[j].first = cost;

                if (candidates_.size() == beam_width) {
                    // 保持している候補がビーム幅分になったときにsegment treeを構築する
                    full_ = true;
                    st_ = MaxSegtree(costs_);
                }
            }
        }

        // 選んだ候補を返す
        const vector<Candidate>& select() const {
            return candidates_;
        }

        // 実行可能解が見つかったか
        bool have_finished() const {
            return !finished_candidates_.empty();
        }

        // 実行可能解に到達するCandidateを返す
        vector<Candidate> get_finished_candidates() const {
            return finished_candidates_;
        }

        // 最もよいCandidateを返す
        Candidate calculate_best_candidate() const {
            if (full_) {
                size_t best = 0;
                for (size_t i = 0; i < beam_width; ++i) {
                    if (st_.get(i).first < st_.get(best).first) {
                        best = i;
                    }
                }
                return candidates_[best];
            } else {
                size_t best = 0;
                for (size_t i = 0; i < candidates_.size(); ++i) {
                    if (costs_[i].first < costs_[best].first) {
                        best = i;
                    }
                }
                return candidates_[best];
            }
        }

        void clear() {
            candidates_.clear();
            hash_to_index_.clear();
            full_ = false;
        }

    private:
        // 削除可能な優先度付きキュー
        using MaxSegtree = atcoder::segtree<
            pair<Cost,int>,
            [](pair<Cost,int> a, pair<Cost,int> b){
                if (a.first >= b.first) {
                    return a;
                } else {
                    return b;
                }
            },
            []() { return make_pair(numeric_limits<Cost>::min(), -1); }
        >;

        size_t beam_width;
        vector<Candidate> candidates_;
        HashMap<Hash,int> hash_to_index_;
        bool full_;
        vector<pair<Cost,int>> costs_;
        MaxSegtree st_;
        vector<Candidate> finished_candidates_;
};

// 深さ優先探索に沿って更新する情報をまとめたクラス
class State {
    public:
        explicit State(const Input& input) {
            m_target = input.target;
            m_pos = make_pair(0, 0);
            m_vel = make_pair(0, 0);
        }

        // EvaluatorとHashの初期値を返す
        pair<Evaluator,Hash> make_initial_node() {
            return {Evaluator(0), calc_hash(m_pos.first, m_pos.second, m_vel.first, m_vel.second, m_target)};
        }

        // 次の状態候補を全てselectorに追加する
        // 引数
        //   evaluator : 今の評価器
        //   hash      : 今のハッシュ値
        //   parent    : 今のノードID（次のノードにとって親となる）
        void expand(const Evaluator& evaluator, Hash hash, int parent, Selector& selector) {
            const int cur_pos_idx = target_idx[m_pos];
            int find_idx = 13;
            for(int i=0;i<13;i++){
                bool find = false;
                for(auto next_pos : m_target){
                    int pos_dif0 = next_pos.first - m_pos.first - (i+1) * m_vel.first;
                    int pos_dif1 = next_pos.second - m_pos.second - (i+1) * m_vel.second;
                    if(abs(pos_dif0) > 100 || abs(pos_dif1) > 100) continue;
                    if(small_moves[i][pos_dif0 + 100].empty()) continue;
                    if(small_moves[i][pos_dif1 + 100].empty()) continue;
                    for(auto [p0, s0] : small_moves[i][pos_dif0 + 100]){
                        for(auto [p1, s1] : small_moves[i][pos_dif1 + 100]){
                            int min_x = 100000000;
                            int max_x = -100000000;
                            int min_y = 100000000;
                            int max_y = -100000000;
                            for(auto [x, y] : m_target){
                                if(x == next_pos.first && y == next_pos.second) continue;
                                min_x = min(min_x, x);
                                max_x = max(max_x, x);
                                min_y = min(min_y, y);
                                max_y = max(max_y, y);
                            }
                            if(m_target.size() == 1){
                                min_x = max_x;
                                min_y = max_y;
                            }
                            auto cost = evaluator.evaluate() % (1LL << 32) + i * (1LL << 32) + (max_x - min_x) + (max_y - min_y);
                            selector.push(Candidate(Action(cur_pos_idx, target_idx[next_pos], p0-100, p1-100), Evaluator(cost), calc_hash(cur_pos_idx, target_idx[next_pos], m_vel.first + p0 - 100, m_vel.second + p1 - 100, m_target, next_pos), parent), false);
                            find = true;
                        }
                    }
                }
                if(find){
                    find_idx = min(find_idx, i);
                }
                if(i == find_idx+3) return;
            }
            if(find_idx != 13) return;
            int min_step = 1000000000;
            int cnt = 0;
            for(auto next_pos : m_target){
                // ++cnt;
                // cerr << "cnt = " << cnt << " " << min_step << endl << "\r";
                // if(cnt == 10) break;
                auto cur_pos = m_pos;
                auto cur_vel = m_vel;
                int step = 0;
                while(cur_pos != next_pos){
                    ++step;
                    if(step > min_step) break;
                    long long remain0 = next_pos.first - cur_pos.first;
                    long long remain1 = next_pos.second - cur_pos.second;
                    if(remain0 * cur_vel.first < 0){
                        cur_vel.first += (cur_vel.first > 0 ? -1 : 1);
                    } else if(remain0 == 0){
                        if(cur_vel.first != 0){
                            cur_vel.first += (cur_vel.first > 0 ? -1 : 1);
                        }
                    } else {
                        long long remain = abs(remain0);
                        long long v = abs(cur_vel.first);
                        bool ok = false;
                        for(int i=1;i>=0;i--){
                            if(remain >= (v + i + 1) * (v + i) / 2){
                                cur_vel.first += (remain0 > 0 ? i : -i);
                                ok = true;
                                break;
                            }
                        }
                        if(!ok){
                            cur_vel.first += (remain0 > 0 ? -1 : 1);
                        }
                    }
                    if(remain1 * cur_vel.second < 0){
                        cur_vel.second += (cur_vel.second > 0 ? -1 : 1);
                    } else if(remain1 == 0){
                        if(cur_vel.second != 0){
                            cur_vel.second += (cur_vel.second > 0 ? -1 : 1);
                        }
                    } else {
                        long long remain = abs(remain1);
                        long long v = abs(cur_vel.second);
                        bool ok = false;
                        for(int i=1;i>=0;i--){
                            if(remain >= (v + i + 1) * (v + i) / 2){
                                cur_vel.second += (remain1 > 0 ? i : -i);
                                ok = true;
                                break;
                            }
                        }
                        if(!ok){
                            cur_vel.second += (remain1 > 0 ? -1 : 1);
                        }
                    }
                    cur_pos.first += cur_vel.first;
                    cur_pos.second += cur_vel.second;
                }
                if(cur_pos == next_pos){
                    min_step = step;
                    int min_x = 100000000;
                    int max_x = -100000000;
                    int min_y = 100000000;
                    int max_y = -100000000;
                    for(auto [x, y] : m_target){
                        if(x == next_pos.first && y == next_pos.second) continue;
                        min_x = min(min_x, x);
                        max_x = max(max_x, x);
                        min_y = min(min_y, y);
                        max_y = max(max_y, y);
                    }
                    if(m_target.size() == 1){
                        min_x = max_x;
                        min_y = max_y;
                    }
                    auto cost = evaluator.evaluate() % (1LL << 32) + step * (1LL << 32) + (max_x - min_x) + (max_y - min_y);
                    selector.push(Candidate(Action(cur_pos_idx, target_idx[next_pos], cur_vel.first - m_vel.first, cur_vel.second - m_vel.second), Evaluator(cost), calc_hash(cur_pos_idx, target_idx[next_pos], cur_vel.first, cur_vel.second, m_target, next_pos), parent), false);
                }
            }
        }

        // actionを実行して次の状態に遷移する
        void move_forward(Action action) {
            auto [p0, p1, vd0, vd1] = action.decode();
            m_pos.first = idx_to_target[p1].first;
            m_pos.second = idx_to_target[p1].second;
            m_vel.first += vd0;
            m_vel.second += vd1;
            m_target.erase(m_pos);
        }

        // actionを実行する前の状態に遷移する
        // 今の状態は、親からactionを実行して遷移した状態である
        void move_backward(Action action) {
            auto [p0, p1, vd0, vd1] = action.decode();
            m_target.insert(idx_to_target[p1]);
            m_pos.first = idx_to_target[p0].first;
            m_pos.second = idx_to_target[p0].second;
            m_vel.first -= vd0;
            m_vel.second -= vd1;
        }

    private:
        set<pair<int, int>> m_target;
        pair<int, int> m_pos;
        pair<int, int> m_vel;
};

// Euler Tourを管理するためのクラス
class Tree {
    public:
        explicit Tree(const State& state, const Config& config) :
            state_(state)
        {
            curr_tour_.reserve(config.tour_capacity);
            next_tour_.reserve(config.tour_capacity);
            leaves_.reserve(config.beam_width);
            buckets_.assign(config.beam_width, {});
        }

        // 状態を更新しながら深さ優先探索を行い、次のノードの候補を全てselectorに追加する
        void dfs(Selector& selector) {
            if (curr_tour_.empty()) {
                // 最初のターン
                auto [evaluator, hash] = state_.make_initial_node();
                state_.expand(evaluator, hash, 0, selector);
                return;
            }

            for (auto [leaf_index, action] : curr_tour_) {
                if (leaf_index >= 0) {
                    // 葉
                    state_.move_forward(action);
                    auto& [evaluator, hash] = leaves_[leaf_index];
                    state_.expand(evaluator, hash, leaf_index, selector);
                    state_.move_backward(action);
                } else if (leaf_index == -1) {
                    // 前進辺
                    state_.move_forward(action);
                } else {
                    // 後退辺
                    state_.move_backward(action);
                }
            }
        }

        // 木を更新する
        void update(const vector<Candidate>& candidates) {
            leaves_.clear();

            if (curr_tour_.empty()) {
                // 最初のターン
                for (const Candidate& candidate : candidates) {
                    curr_tour_.push_back({(int)leaves_.size(), candidate.action});
                    leaves_.push_back({candidate.evaluator, candidate.hash});
                }
                return;
            }

            for (const Candidate& candidate : candidates) {
                buckets_[candidate.parent].push_back({candidate.action, candidate.evaluator, candidate.hash});
            }

            auto it = curr_tour_.begin();

            // 一本道を反復しないようにする
            while (it->first == -1 && it->second == curr_tour_.back().second) {
                Action action = (it++)->second;
                state_.move_forward(action);
                direct_road_.push_back(action);
                curr_tour_.pop_back();
            }

            // 葉の追加や不要な辺の削除をする
            while (it != curr_tour_.end()) {
                auto [leaf_index, action] = *(it++);
                if (leaf_index >= 0) {
                    // 葉
                    if (buckets_[leaf_index].empty()) {
                        continue;
                    }
                    next_tour_.push_back({-1, action});
                    for (auto [new_action, evaluator, hash] : buckets_[leaf_index]) {
                        int new_leaf_index = leaves_.size();
                        next_tour_.push_back({new_leaf_index, new_action});
                        leaves_.push_back({evaluator, hash});
                    }
                    buckets_[leaf_index].clear();
                    next_tour_.push_back({-2, action});
                } else if (leaf_index == -1) {
                    // 前進辺
                    next_tour_.push_back({-1, action});
                } else {
                    // 後退辺
                    auto [old_leaf_index, old_action] = next_tour_.back();
                    if (old_leaf_index == -1) {
                        next_tour_.pop_back();
                    } else {
                        next_tour_.push_back({-2, action});
                    }
                }
            }
            swap(curr_tour_, next_tour_);
            next_tour_.clear();
        }

        // 根からのパスを取得する
        vector<Action> calculate_path(int parent, int turn) const {
            // cerr << curr_tour_.size() << endl;

            vector<Action> ret = direct_road_;
            ret.reserve(turn);
            for (auto [leaf_index, action] : curr_tour_) {
                if (leaf_index >= 0) {
                    if (leaf_index == parent) {
                        ret.push_back(action);
                        return ret;
                    }
                } else if (leaf_index == -1) {
                    ret.push_back(action);
                } else {
                    ret.pop_back();
                }
            }

        }

    private:
        State state_;
        vector<pair<int,Action>> curr_tour_;
        vector<pair<int,Action>> next_tour_;
        vector<pair<Evaluator,Hash>> leaves_;
        vector<vector<tuple<Action,Evaluator,Hash>>> buckets_;
        vector<Action> direct_road_;
};

// ビームサーチを行う関数
vector<Action> beam_search(const Config& config, const State& state, const Timer& timer) {
    Tree tree(state, config);

    // 新しいノード候補の集合
    Selector selector(config);

    for (int turn = 0; turn < config.max_turn; ++turn) {
        // cerr << "Turn = " << turn << endl;
        // Euler Tourでselectorに候補を追加する
        tree.dfs(selector);

        if (selector.have_finished()) {
            // ターン数最小化型の問題で実行可能解が見つかったとき
            Candidate candidate = selector.get_finished_candidates()[0];
            vector<Action> ret = tree.calculate_path(candidate.parent, turn + 1);
            ret.push_back(candidate.action);
            return ret;
        }

        assert(!selector.select().empty());

        if (turn == config.max_turn - 1) {
            // ターン数固定型の問題で全ターンが終了したとき
            Candidate best_candidate = selector.calculate_best_candidate();
            vector<Action> ret = tree.calculate_path(best_candidate.parent, turn+1);
            ret.push_back(best_candidate.action);
            return ret;
        }

        // 木を更新する
        tree.update(selector.select());

        selector.clear();
    }
}

} // namespace beam_search

//------------------------------------------------------------------------------
struct Solver {
    const Input input;
    vector<beam_search::Action> output;

    Solver(const Input& input) :
        input(input) {}

    void solve(const Timer& timer) {
        size_t beam_width = 50000;
        size_t tour_capacity = 30 * beam_width;
        uint32_t hash_map_capacity = 32 * 30 * beam_width;
        beam_search::Config config = {
            input.target.size(),
            beam_width,
            tour_capacity,
            hash_map_capacity
        };
        beam_search::State state(input);
        output = beam_search::beam_search(config, state, timer);
    }

    void print() const {
        auto pos = make_pair(0, 0);
        auto vel = make_pair(0, 0);
        int length = 0;
        if(input.has_origin){
            ++length;
            cout << 5;
        }
        for (beam_search::Action action : output) {
            auto [p0, p1, vd0, vd1] = action.decode();
            bool find = false;
            const auto next_pos = idx_to_target[p1];
            for(int i=0;i<13;i++){
                int pos_dif0 = next_pos.first - pos.first - (i+1) * vel.first;
                int pos_dif1 = next_pos.second - pos.second - (i+1) * vel.second;
                if(abs(pos_dif0) > 100 || abs(pos_dif1) > 100) continue;
                if(small_moves[i][pos_dif0 + 100].empty()) continue;
                if(small_moves[i][pos_dif1 + 100].empty()) continue;
                for(auto [p0, s0] : small_moves[i][pos_dif0 + 100]){
                    for(auto [p1, s1] : small_moves[i][pos_dif1 + 100]){
                        if(p0 == vd0+100 && p1 == vd1+100){
                            for(int j=0;j<s0.size();j++){
                                ++length;
                                cout << 3 * (s1[j] - '0') + (s0[j] - '0') + 1;
                            }
                            find = true;
                            pos = next_pos;
                            vel.first += vd0;
                            vel.second += vd1;
                            break;
                        }
                    }
                    if(find) break;
                }
                if(find) break;
            }
            if(!find){
                auto& cur_pos = pos;
                auto& cur_vel = vel;
                while(cur_pos != next_pos){
                    long long remain0 = next_pos.first - cur_pos.first;
                    long long remain1 = next_pos.second - cur_pos.second;
                    int dvx = 0, dvy = 0;
                    if(remain0 * cur_vel.first < 0){
                        dvx = (cur_vel.first > 0 ? -1 : 1);
                    } else if(remain0 == 0){
                        if(cur_vel.first != 0){
                            dvx = (cur_vel.first > 0 ? -1 : 1);
                        }
                    } else {
                        long long remain = abs(remain0);
                        long long v = abs(cur_vel.first);
                        bool ok = false;
                        for(int i=1;i>=0;i--){
                            if(remain >= (v + i + 1) * (v + i) / 2){
                                dvx = (remain0 > 0 ? i : -i);
                                ok = true;
                                break;
                            }
                        }
                        if(!ok){
                            dvx = (remain0 > 0 ? -1 : 1);
                        }
                    }
                    if(remain1 * cur_vel.second < 0){
                        dvy = (cur_vel.second > 0 ? -1 : 1);
                    } else if(remain1 == 0){
                        if(cur_vel.second != 0){
                            dvy = (cur_vel.second > 0 ? -1 : 1);
                        }
                    } else {
                        long long remain = abs(remain1);
                        long long v = abs(cur_vel.second);
                        bool ok = false;
                        for(int i=1;i>=0;i--){
                            if(remain >= (v + i + 1) * (v + i) / 2){
                                dvy = (remain1 > 0 ? i : -i);
                                ok = true;
                                break;
                            }
                        }
                        if(!ok){
                            dvy = (remain1 > 0 ? -1 : 1);
                        }
                    }
                    ++length;
                    cout << (3*(dvy+1) + (dvx+2));
                    cur_vel.first += dvx;
                    cur_vel.second += dvy;
                    cur_pos.first += cur_vel.first;
                    cur_pos.second += cur_vel.second;
                }
            }
        }
        cout << endl;
        cerr << "Find length: " << length << endl;
    }
};

int main() {
    make_small_moves();
    Timer timer;
    ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    Input input;
    input.input();

    Solver solver(input);
    solver.solve(timer);
    solver.print();

    return 0;
}
