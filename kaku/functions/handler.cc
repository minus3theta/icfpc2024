// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <cstddef>
#include <google/cloud/functions/http_request.h>
#include <google/cloud/functions/http_response.h>

#include <iostream>
#include <istream>
#include <ostream>
#include <sstream>

using ::google::cloud::functions::HttpRequest;
using ::google::cloud::functions::HttpResponse;
using namespace std;

void solve(istream& in, ostream& out);
void solve(istream&& in, ostream& out) {
  solve(in, out);
}

static string url_decode(const string& c);

HttpResponse handler(HttpRequest req) {  // NOLINT
  string p = url_decode(req.payload());
  ostringstream oss;
  solve(istringstream(p), oss);
  string output = oss.str();
  return HttpResponse{}
      .set_header("Content-Type", "text/plain")
      .set_payload(output);
}

static const char table[]={0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,4,5,6,7,8,9,0,0,0,0,0,0,0,10,11,12,13,14,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,11,12,13,14,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
static char decode_hex_to_char(const char* c) {
  // XXX: [0-9a-zA-Z]以外の全ての文字は、0として扱われる (e.g. "%@X" => 0)
  return (table[static_cast<unsigned char>(c[1])]<<4)+
          table[static_cast<unsigned char>(c[2])];
}

static string url_decode(const string& c) {
  std::string dist = "";
  for(size_t i=0; i<c.size(); i++) {
    switch(c[i]) {
    case '%': dist += decode_hex_to_char(&c[i]); i+=2; break;  // XXX: 末尾に'%'が来る不正な文字列が渡された場合の挙動は未定義(ex. "abc%")
    case '+': dist += ' ';                         break;
    default:  dist += c[i];                        break;
    }
  }
  return dist;
}
