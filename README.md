# dxlib-rs。。
・ "tanukimaru0090/dxlib-ffi" をさらにラップしてSFML風にラップしたもの。
・　まだ開発中なので機能が著しく少なく、安全性やパフォーマンスを保証できません。

#使い方

myproject/Cargo.toml
```。toml。。。。。
[dependencies]
dxlib-rs = { git = "https://github.com/tanukimaru0090/dxlib-rs", branch = "master", version = "0.1.0", features = ["dxlib-ffi"], dxlib-ffi = { git = "https://github.com/tanukimaru0090/dxlib-ffi-rs", branch = "master"} }
```。。。。。。

src / main.rs。
```。。。。。まぁ。。
extern crate dxlib_rs;。。
dxlib_rs :: *;吸います。。
dxlib_rs :: dx_window :: *;使いやすくなります。。
fn main(){。。
 安完全で無い{。。
 mut ref_window = DxWindow :: new();。 .
 window = ref_window.create_window(DxWindow :: videomode(640,480,32).unwrap()、 "hello world window。。!");。。
 window.is_open(){。。
          
      }
}
}
```。。。。。。
上記 ⁇ 作成、閉会後の ⁇ 子も今の。。

最後に、「貨物製造-放出」は、実行 ⁇ と同 ⁇ 「DxLib_x64.dll」は、これと実行の追跡と今の ⁇ です。。
DxLib_x64.dllはDXライブラリの公式サイトのC#版DXライブラリをダウンロードすることで使うことができます。


## License
MIT
