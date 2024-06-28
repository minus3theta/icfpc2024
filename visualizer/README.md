# Usage

`wasm-pack build --target web --out-dir ./vis` で Web Assembly のビルド。

Web Assembly は `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh` でインストール。

使うときは主に lib.rs 内の gen, vis, get_max_turn を実装する。atcoder.rsは公式実装のほぼコピー。shape.rsにsvgのラッパ関数を入れてある。

`python -m http.server` を実行してから `http://localhost:8000/vis.html` にアクセスするとビジュアライザが使える。
