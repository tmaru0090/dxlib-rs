# dxlib-rs
・ "tanukimaru0090/dxlib-ffi" をさらにラップしてSFML風にラップしたもの。
・　まだ開発中なので機能が著しく少なく、安全性やパフォーマンスを保証できません。

#使い方

myproject/Cargo.toml
```toml
[dependencies.dxlib_rs]
git = "https://github.com/tanukimaru0090/dxlib-rs.git"
```

src/main.rs
```Rust
extern crate dxlib_rs;
use dxlib_rs::*;
use dxlib_rs::dx_window::*;
fn main(){
  unsafe{
      let mut ref_window = DxWindow::new();
      let window = ref_window.create_window(DxWindow::videomode(640,480,32).unwrap(),"hello world window!");
      while window.is_open(){
          
      }
}
}
```
上記はウィンドウを作成し、閉じられるまでループを続けるものです。

最後に、 "cargo build --release" などでビルドをして、実行ファイルと同じディレクトリに "DxLib_x64.dll" を置くことで実行することができます。
DxLib_x64.dllはDXライブラリの公式サイトのC#版DXライブラリをダウンロードすることで使うことができます。


## License
MIT
