# vre-core
<https://github.com/vreui/vre-core>

威惹: 核心引擎

[![CI](https://github.com/vreui/vre-core/actions/workflows/ci.yml/badge.svg)](https://github.com/vreui/vre-core/actions)

文档请见: <https://github.com/vreui/vre-doc>


## 编译开发

+ 代码格式化:

  ```
  cargo fmt
  ```

+ 编译:

  ```
  cargo build
  ```

+ 测试:

  ```
  cargo test
  ```


## cargo features

+ `api` (默认启用)

  仅用于跨越 wasm 的接口.
  (`vre-ui` 使用)

+ `server` (依赖 `api`, 默认启用)

  服务进程.
  主要包含组件 `wasmer`.

+ `window`

  窗口进程.
  主要包含组件 `WebRender`, `glw`.

+ `wr_debug` (依赖 `window`)

  用于 WebRender 调试.


## LICENSE

`Mozilla Public License 2.0`
