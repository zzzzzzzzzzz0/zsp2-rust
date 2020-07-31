ZhServerPage2 中文动态网页第二版。

由 rustlang 实现。
与 [zsp-go](https://github.com/zzzzzzzzzzz0/zsp-go/blob/master/readme.md) 目标相同，服务器端直接解释，
亦多种模式直接执行其上二进制程序（-重定向、-被动者、-被动者2等）。
web framework 部分基于现行的 [actix.rs](https://actix.rs)，并借力 
[lazy_static.rs](https://github.com/rust-lang-nursery/lazy-static.rs)、
[regex](https://github.com/rust-lang/regex) 等。

语法可先感受下示范《我的影院》中的 [1个源码](demo/movbrow/root/play2.zsp)……（有人不懂写文档）

[youku 上演示的《我的影院》](https://v.youku.com/v_show/id_XNDY1MDIwMTczMg==.html) 

### 开三工鸟

```bash
$ git clone "https://github.com/zzzzzzzzzzz0/zhscript2-rust.git"
$ git clone "https://github.com/zzzzzzzzzzz0/zs2-l4-rust.git"
$ git clone "https://github.com/zzzzzzzzzzz0/zsp2-rust.git"
$ cd zsp2-rust
$ rustup override set nightly
$ cargo run -- -zsp-conf demo/movbrow/conf.zs
```

如果输出最终类似于

```
1-/zzzzzzzzzzz4/video
  /zzzzzzzzzzz4/video-shot (1
2-/zzzzzzzzzzz6/zzzzzzzzzzz4/opt/opt/video
3-/zzzzzzzzzzz6/zzzzzzzzzzz4/opt/opt/video2
4-/zzzzzzzzzzz7/zzzzzzzzzzz4/opt/opt/video
  /zzzzzzzzzzz7/zzzzzzzzzzz4/opt/opt/video-shot (2
5-/zzzzzzzzzzz7/zzzzzzzzzzz4/opt/opt/video2
6-‘~’/视频
7-‘~’/Videos
8-‘~’/ビデオ
根 demo/movbrow/root
绑定地址 127.0.0.1:8084
```

那么吾甚欣慰否则头痛吧

再努力一点，中文脚本的语法请前往 [zhscript2/readme.md](https://github.com/zzzzzzzzzzz0/zhscript2/blob/master/readme.md) 查看，
具则具之，异则……