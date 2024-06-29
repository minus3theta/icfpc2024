#include <cassert>
#include <cstddef>
#include <istream>
#include <ostream>
#include <iostream>

using namespace std;

namespace {
struct P {
  int x, y;

  P(int x, int y) : x(x), y(y) {}

  P operator+(const P& p) const {
    return {x + p.x, y + p.y};
  }

  P operator-(const P& p) const {
    return {x - p.x, y - p.y};
  }

  P operator*(const int& i) const {
    return {x * i, y * i};
  }

  P& operator+=(const P& p) {
    x += p.x;
    y += p.y;
    return *this;
  }

  bool operator==(const P& p) const {
      return x == p.x && y == p.y;
  }

  bool operator<(const P& p) const {
    return x < p.x || (x == p.x && y < p.y);
  }

  int distance(const P& p) const {
    return max(abs(x - p.x), abs(y - p.y));
  }

  int distance2(const P& p) const {
    return abs(x - p.x) + abs(y - p.y);
  }

  friend ostream& operator<<(ostream& os, const P& p) {
    os << "(" << p.x << ", " << p.y << ")";
    return os;
  }
};

vector<P> moves = {
  {-1, -1},
  {0, -1},
  {1, -1},
  {-1, 0},
  {0, 0},
  {1, 0},
  {-1, 1},
  {0, 1},
  {1, 1},
};

P findNearest(vector<P> &ps, P c, P v) {
  for (size_t i = 1; i < 100; i++) {
    P a = c + v * i;
    int d = (i+1) * i / 2;
    vector<pair<int, P>> candidates;
    for (auto &p : ps) {
      if (a.distance(p) <= d) {
        candidates.emplace_back(a.distance2(p), p);
        return p;
      }
    }
    if (!candidates.empty()) {
      sort(candidates.begin(), candidates.end());
      return candidates[0].second;
    }
  }
  throw "No nearest point found";
}

void remove(vector<P> &ps, P p) {
  ps.erase(find(ps.begin(), ps.end(), p));
}

tuple<P, size_t> getDirection(const P& c, const P& v, const P& nearest) {
  auto x = c + v;
  P dire = moves[4];
  size_t idx = 4;
  int dist = x.distance2(nearest);
  for (size_t i = 0; i < moves.size(); i++) {
    auto new_v = v + moves[i];
    if (abs(new_v.x) > 4 || abs(new_v.y) > 4) {
      continue;
    }
    if ((x + moves[i]).distance2(nearest) < dist) {
      dire = moves[i];
      idx = i;
      dist = (x + moves[i]).distance2(nearest);
    }
  }
  return {dire, idx + 1};
}

void solve(vector<P> &ps, ostream &out) {
  P c = {0, 0};
  P v = {0, 0};
  while (!ps.empty()) {
    // cerr << "-------------------" << endl;
    // for (auto &p : ps) {
    //   cerr << p << endl;
    // }
    // cerr << "-------------------" << endl;
    P nearest = findNearest(ps, c, v);
    // cerr << "current: " << c << endl;
    // cerr << "velocity: " << v << endl;
    // cerr << "nearest: " << nearest << endl;
    auto [d, i] = getDirection(c, v, nearest);
    // cerr << i << endl;
    out << i;
    v += d;
    c += v;
    // cerr << "moved" << endl;
    // cerr << "current: " << c << endl;
    // cerr << "velocity: " << v << endl;
    if (nearest == c) {
      cerr << "reached: " << c << ", rest: " << ps.size() - 1 << endl;
      remove(ps, nearest);
    }
  }
  out << endl;
}
}  // namespace

void solve(istream& in, ostream& out) {
  int x, y;
  vector<P> ps;
  while(in >> x && in >> y) {
    ps.emplace_back(x, y);
  }
  solve(ps, out);
}
