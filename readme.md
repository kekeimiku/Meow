多平台内存扫描器 linux windows macos android/ios

暂时业余时间瞎几把写中

距离可用还有很远的距离，目前还比较混乱。。

优先级先后顺序：linux android windows macos ios

v0.1.1 功能列表 (暂时只有linux实现了):

- 快速内存过滤

- 内存写入

- 内存冻结

- dump内存

- 注入动态库

目前拥有的功能：

内存冻结和dump，以及兼顾cpu和内存占用低，并且性能非常好的内存过滤。

todo: 指针查找

![img2](img/file.gif)

非常方便的直接注入so到目标进程。
![img1](img/cnm.gif)

如果有人想贡献代码，请注意，尽量不要再引入第三方库。

## Thanks for free JetBrains Open Source license

<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo (Main) logo." height="200"/>
