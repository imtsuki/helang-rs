# helang-rs

> Have you considered rEwRiTiNg iT iN rUsT?

## 介绍

- 众所周知，一切漏洞都是**内存安全**问题。
  - "**~70%** of the vulnerabilities addressed through a security update each year continue to be memory safety issues." - [Microsoft Security Response Center](https://github.com/Microsoft/MSRC-Security-Research/blob/master/presentations/2019_02_BlueHatIL/2019_01%20-%20BlueHatIL%20-%20Trends%2C%20challenge%2C%20and%20shifts%20in%20software%20vulnerability%20mitigation.pdf)
- 没有一个 C 语言项目实现了完全的内存安全。
- 要不是 GCC 不努力，何同学又怎么会发现不了数组越界呢？
- 使用 Rust 重写，你就可以自动获得 **✨100%✨** 的内存安全。
- 如果一段程序可以被 Rust 重写，那它就一定会被 Rust 重写。

## 使用例

不加任何参数，你就可以与何同学直接对话：

```bash
cargo run
```

```helang
helang> u8 forceCon = [68]
helang> forceCon
0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
helang> forceCon[1 | 2 | 6 | 7 | 11 | 52 | 57 | 58 | 65] = 10
helang> print forceCon
10 | 10 | 0 | 0 | 0 | 10 | 10 | 0 | 0 | 0 | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 0 | 0 | 0 | 10 | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 0 | 0
helang> forceCon[127]
index 127 out of bounds 68     // 极为先进的内存安全检测
helang> test5g
很残念，你的电脑并没有配备 5G 芯片。
helang>
```

或者，你可以指定源文件：

```bash
cargo run example.he
```

> 我们保证，在开发过程中，没有一个野指针受到伤害。
