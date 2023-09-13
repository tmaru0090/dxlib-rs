# dxlib-rs。。
・ "tanukimaru0090/dxlib-ffi" をさらにラップしてSFML風にラップしたもの。
・　まだ開発中なので機能が著しく少なく、安全性やパフォーマンスを保証できません。

#使い方

myproject/Cargo.toml
```toml
[dependencies]
dxlib-rs = { git = "https://github.com/tanukimaru0090/dxlib-rs", branch = "master", version = "0.1.0", features = ["dxlib-ffi"], dxlib-ffi = { git = "https://github.com/tanukimaru0090/dxlib-ffi-rs", branch = "master"} }
```

src / main.rs。
```Rust
extern crate dxlib_rs;
dxlib_rs :: *;
dxlib_rs :: dx_window :: *;
fn main(){
unsafe
{
 mut ref_window = DxWindow :: new();。 .
 window = ref_window.create_window(DxWindow :: videomode(640,480,32).unwrap()、 "hello world window。。!");
 window.is_open(){
          
      }
}
}
```
上記はWindowを表示するだけです。
最後に、 「DxLib_x64.dll」を実行時のディレクトリに置くことで実行できるはずです。
DxLib_x64.dllはDXライブラリの公式サイトのC#版DXライブラリをダウンロードすることで使うことができます。

## フォークさせてもらった、ライブラリ
elipmoc/rust_dxlib
## License
MIT License

Copyright (c) 2018 らいパン粉

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


## License
MIT
