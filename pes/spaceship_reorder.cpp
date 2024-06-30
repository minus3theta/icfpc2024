#include <iostream>
#include <vector>
#include <string>
#include <iomanip>
#include <random>
#include <chrono>
#include <tuple>
#include <algorithm>
#include <set>

using namespace std;

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
inline long long calc_cost_base(int i0, int i1, int i2, const vector<int>& order, const vector<pair<int,int>>& target){
    const auto [tx0, ty0] = target[order[i0]];
    const auto [tx1, ty1] = target[order[i1]];
    const auto [tx2, ty2] = target[order[i2]];
    int dx = tx2 - 2*tx1 + tx0;
    int dy = ty2 - 2*ty1 + ty0;
    return abs(dx) + abs(dy);
}
//------------------------------------------------------------------------------
long long calc_dif_2opt(int p0, int p1, const vector<int>& order, const vector<pair<int, int>>& target){
    // -- p0 -- p0+1 -- p0+2 -- ... -- p1-1 -- p1 -- p1+1 -- p1+2 -- ...
    // -- p0 -- p1 -- p1-1 -- ... -- p0+2 -- p0+1 -- p1+1 -- p1+2 -- ...
    long long res = 0;
    if(p0 > 0){
        res -= calc_cost_base(p0-1, p0, p0+1, order, target);
        // cerr << "Neg: " << p0-1 << " " << p0 << " " << p0+1 << " " << calc_cost_base(p0-1, p0, p0+1, order, target) << endl;
    }
    if(p0+2 < order.size()){
        res -= calc_cost_base(p0, p0+1, p0+2, order, target);
        // cerr << "Neg: " << p0 << " " << p0+1 << " " << p0+2 << " " << calc_cost_base(p0, p0+1, p0+2, order, target) << endl;
    }
    if(p0 != p1-1 && p1+1 < order.size()){
        res -= calc_cost_base(p1-1, p1, p1+1, order, target);
        // cerr << "Neg: " << p1-1 << " " << p1 << " " << p1+1 << " " << calc_cost_base(p1-1, p1, p1+1, order, target) << endl;
    }
    if(p1+2 < order.size()){
        res -= calc_cost_base(p1, p1+1, p1+2, order, target);
        // cerr << "Neg: " << p1 << " " << p1+1 << " " << p1+2 << " " << calc_cost_base(p1, p1+1, p1+2, order, target) << endl;
    }
    if(p0-1 >= 0){
        res += calc_cost_base(p0-1, p0, p1, order, target);
        // cerr << "Add: " << p0-1 << " " << p0 << " " << p1 << " " << calc_cost_base(p0-1, p0, p1, order, target) << endl;
    }
    if(p0 != p1-1){
        res += calc_cost_base(p0, p1, p1-1, order, target);
        // cerr << "Add: " << p0 << " " << p1 << " " << p1-1 << " " << calc_cost_base(p0, p1, p1-1, order, target) << endl;
    }
    if(p0 != p1-1 && p1+1 < order.size()){
        res += calc_cost_base(p0+2, p0+1, p1+1, order, target);
        // cerr << "Add: " << p0+2 << " " << p0+1 << " " << p1+1 << " " << calc_cost_base(p0+2, p0+1, p1+1, order, target) << endl;
    }
    if(p1+2 < order.size()){
        res += calc_cost_base(p0+1, p1+1, p1+2, order, target);
        // cerr << "Add: " << p0+1 << " " << p1+1 << " " << p1+2 << " " << calc_cost_base(p0+1, p1+1, p1+2, order, target) << endl;
    }
    // cerr << "======" << endl;
    return res;
}

//------------------------------------------------------------------------------
long long calc_dif_insert(int p0, int p1, const vector<int>& order, const vector<pair<int, int>>& target){
    // p0 の後に p1 を挿入する
    if(p1 == p0+1) return 0; // 動かない
    if(p1+1 == p0){
        // -- p1-1 -- p1 -- p0 -- p0+1 -- ...
        // -- p1-1 -- p0 -- p1 -- p0+1 -- ...
        long long res = 0;
        if(p1-2 >= 0){
            res -= calc_cost_base(p1-2, p1-1, p1, order, target);
            res += calc_cost_base(p1-2, p1-1, p0, order, target);
        }
        if(p1-1 >= 0){
            res -= calc_cost_base(p1-1, p1, p0, order, target);
            res += calc_cost_base(p1-1, p0, p1, order, target);
        }
        if(p0+1 < order.size()){
            res -= calc_cost_base(p1, p0, p0+1, order, target);
            res += calc_cost_base(p0, p1, p0+1, order, target);
        }
        if(p0+2 < order.size()){
            res -= calc_cost_base(p0, p0+1, p0+2, order, target);
            res += calc_cost_base(p1, p0+1, p0+2, order, target);
        }
        return res;
    }
    if(p1+2 == p0){
        // -- p1-1 -- p1 -- p1+1 -- p0 -- p0+1 -- ...
        // -- p1-1 -- p1+1 -- p0 -- p1 -- p0+1 -- ...
        long long res = 0;
        if(p1-2 >= 0){
            res -= calc_cost_base(p1-2, p1-1, p1, order, target);
            // cerr << "neg: " << p1-2 << " " << p1-1 << " " << p1 << " " << calc_cost_base(p1-2, p1-1, p1, order, target) << endl;
            res += calc_cost_base(p1-2, p1-1, p1+1, order, target);
            // cerr << "add: " << p1-2 << " " << p1-1 << " " << p1+1 << " " << calc_cost_base(p1-2, p1-1, p0+1, order, target) << endl;
        }
        if(p1-1 >= 0){
            res -= calc_cost_base(p1-1, p1, p1+1, order, target);
            // cerr << "neg: " << p1-1 << " " << p1 << " " << p1+1 << " " << calc_cost_base(p1-1, p1, p1+1, order, target) << endl;
            res += calc_cost_base(p1-1, p1+1, p0, order, target);
            // cerr << "add: " << p1-1 << " " << p1+1 << " " << p0 << " " << calc_cost_base(p1-1, p1+1, p0, order, target) << endl;
        }
        res -= calc_cost_base(p1, p1+1, p0, order, target);
        // cerr << "neg: " << p1 << " " << p1+1 << " " << p0 << " " << calc_cost_base(p1, p1+1, p0, order, target) << endl;
        res += calc_cost_base(p1+1, p0, p1, order, target);
        // cerr << "add: " << p1+1 << " " << p0 << " " << p1 << " " << calc_cost_base(p1+1, p0, p1, order, target) << endl;
        if(p0+1 < order.size()){
            res -= calc_cost_base(p1+1, p0, p0+1, order, target);
            // cerr << "neg: " << p1+1 << " " << p0 << " " << p0+1 << " " << calc_cost_base(p1+1, p0, p0+1, order, target) << endl;
            res += calc_cost_base(p0, p1, p0+1, order, target);
            // cerr << "add: " << p0 << " " << p1 << " " << p0+1 << " " << calc_cost_base(p0, p1, p0+1, order, target) << endl;
        }
        if(p0+2 < order.size()){
            res -= calc_cost_base(p0, p0+1, p0+2, order, target);
            // cerr << "neg: " << p0 << " " << p0+1 << " " << p0+2 << " " << calc_cost_base(p0, p0+1, p0+2, order, target) << endl;
            res += calc_cost_base(p1, p0+1, p0+2, order, target);
            // cerr << "add: " << p1 << " " << p0+1 << " " << p0+2 << " " << calc_cost_base(p1, p0+1, p0+2, order, target) << endl;
        }
        // cerr << "======" << endl;
        return res;
    }
    if(p0+2 == p1){
        // -- p0-1 -- p0 -- p0+1 -- p1 -- p1+1 -- p1+2 -- ...
        // -- p0-1 -- p0 -- p1 -- p0+1 -- p1+1 -- p1+2 -- ...
        long long res = 0;
        if(p0-1 >= 0){
            res -= calc_cost_base(p0-1, p0, p0+1, order, target);
            res += calc_cost_base(p0-1, p0, p1, order, target);
        }
        res -= calc_cost_base(p0, p0+1, p1, order, target);
        res += calc_cost_base(p0, p1, p0+1, order, target);
        if(p1+1 < order.size()){
            res -= calc_cost_base(p0+1, p1, p1+1, order, target);
            res += calc_cost_base(p1, p0+1, p1+1, order, target);
        }
        if(p1+2 < order.size()){
            res -= calc_cost_base(p1, p1+1, p1+2, order, target);
            res += calc_cost_base(p0+1, p1+1, p1+2, order, target);
        }
        return res;
    }
    // -- p0 -- p0+1 -- p0+2 -- ... -- p1-1 -- p1 -- p1+1 -- p1+2 -- ...
    // -- p0 -- p1 -- p0+1 -- p0+2 -- ... -- p1-1 -- p1+1 -- p1+2 -- ...
    // or
    // -- p1-1 -- p1 -- p1+1 -- ... -- p0-1 -- p0 -- p0+1 -- p0+2 -- ...
    // -- p1-1 -- p1+1 -- ... -- p0-1 -- p0 -- p1 -- p0+1 -- p0+2 -- ...
    long long res = 0;
    if(p0-1 >= 0 && p0+1 < order.size()) res -= calc_cost_base(p0-1, p0, p0+1, order, target);
    if(p0+2 < order.size()) res -= calc_cost_base(p0, p0+1, p0+2, order, target);
    if(p1-2 != p0 && p1-2 >= 0) res -= calc_cost_base(p1-2, p1-1, p1, order, target);
    if(p1-1 >= 0 && p1+1 < order.size()) res -= calc_cost_base(p1-1, p1, p1+1, order, target);
    if(p1+2 < order.size()) res -= calc_cost_base(p1, p1+1, p1+2, order, target);
    if(p0-1 >= 0) res += calc_cost_base(p0-1, p0, p1, order, target);
    if(p0+1 < order.size()) res += calc_cost_base(p0, p1, p0+1, order, target);
    if(p0+2 < order.size()) res += calc_cost_base(p1, p0+1, p0+2, order, target);
    if(p1-2 >= 0 && p1+1 < order.size()) res += calc_cost_base(p1-2, p1-1, p1+1, order, target);
    if(p1-1 >= 0 && p1+2 < order.size()) res += calc_cost_base(p1-1, p1+1, p1+2, order, target);
    return res;
}

//------------------------------------------------------------------------------
long long calc_score(const vector<int>& order, const vector<pair<int,int>>& target){
    long long res = 0;
    for(int i=0;i+2<order.size();i++){
        res += calc_cost_base(i, i+1, i+2, order, target);
    }
    return res;
}

//------------------------------------------------------------------------------
int main(){
    std::ios_base::sync_with_stdio(false);
    vector<pair<int,int>> target;
    set<pair<int, int>> seen;
    bool has_origin = false;
    int x, y;
    target.emplace_back(0, 0);
    while(cin >> x >> y){
        if(x == 0 && y == 0){
            has_origin = true;
            continue;
        }
        if(seen.count({x,y})) continue;
        seen.insert({x, y});
        target.emplace_back(x, y);
    }
    vector<int> order(target.size());
    for(int i=0;i<target.size();i++) order[i] = i;
    long long base_dif = 0;
    for(int i=0;i+1<target.size();i++){
        base_dif += abs(target[i].first - target[i+1].first);
        base_dif += abs(target[i].second - target[i+1].second);
    }
    cerr << "first: " << calc_score(order, target) << endl;

    // // targetの最初の要素以外をsecondの降順でソート
    // sort(order.begin()+1, order.end(), [&](int a, int b){
    //     return target[a].second > target[b].second;
    // });
    // cerr << "first: " << calc_score(order, target) << endl;
    int end = target.size();
    // for(int i=1;i<order.size();i++){
    //     if(target[order[i]].second < 0){ end = i; break; }
    // }

    const double initial_temp = 0.01 * base_dif / (target.size()-1);
    const double final_temp = 0;
    const long long max_iter = 2000000000LL;
    long long cur_score = calc_score(order, target);
    vector<int> best_order = order;
    long long best_score = cur_score;
    for(long long iter=0;iter<max_iter;iter++){
        int type = rnd() % 2;
        int p0, p1;
        long long dif;
        if(type == 0){
            // 2-opt
            p0 = rnd() % end;
            p1 = rnd() % end;
            while(abs(p0-p1) < 3){
                p0 = rnd() % end;
                p1 = rnd() % end;
            }
            if(p0 > p1) swap(p0, p1);
            dif = calc_dif_2opt(p0, p1, order, target);
        } else {
            // insertion
            p0 = rnd() % (end-1) + 1;
            p1 = rnd() % (end-1) + 1;
            while(p0 == p1 || p0 == p1-1){
                p0 = rnd() % (end-1) + 1;
                p1 = rnd() % (end-1) + 1;
            }
            dif = calc_dif_insert(p0, p1, order, target);
        }
        const double ratio = (double)iter / max_iter;
        const double rev_ratio = 1 - ratio;
        const double temperature = initial_temp + (final_temp - initial_temp) * (1 - rev_ratio * rev_ratio * rev_ratio);
        // スコアがtemperature悪化するときの採用確率は約36.8%
        if(dif < 0 || bernoulli_distribution(exp(-dif/temperature))(rnd)){
            cur_score += dif;
            if(type == 0){
                for(int i=0;p0+1+i<p1-i;i++){
                    swap(order[p0+1+i], order[p1-i]);
                }
            } else {
                vector<int> new_order;
                for(int i=0;i<order.size();i++){
                    if(i == p1) continue;
                    new_order.push_back(order[i]);
                    if(i == p0) new_order.push_back(order[p1]);
                }
                order = new_order;
            }
            if(cur_score < best_score){
                cerr << "iter: " << iter << ", score: " << best_score << " -> " << cur_score << endl;
                best_score = cur_score;
                best_order = order;
            }
        }
    }
    bool first = true;
    for(auto v: best_order){
        if(first && !has_origin){
            first = false;
            continue;
        }
        first = false;
        cout << target[v].first << " " << target[v].second << endl;
    }
    cerr << "final: " << calc_score(best_order, target) << endl;
}

/*
0 0
1 -1
1 -3
2 -5
2 -8
3 -10

-1, -1
1, 0
1, -1
1, 1

 */