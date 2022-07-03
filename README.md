## 多平台内存扫描器

> **开发中** 暂时不建议使用，业余时间瞎几把写中

## ⚠️ wip

优先级先后顺序：linux android windows macos ios

meow 分为 core 以及 plugin 两部分，core 只提供 读/写/查找 相关内存的功能，plugin 基于动态库，公开了一些 core api 用于方便用户自定义一些插件，插件允许闭源分发。

以下是 linux v0.1.1 版本 功能列表，所有功能都不需要依赖 ptrace 。

### 核心功能

- [x] 查找数据地址

- [x] 过滤找到的地址

- [x] read/write/dump 内存

- [x] 解析maps

- [ ] 可选内存区域

- [ ] 插件管理器 (进度%70,进行中)

### 争议性核心功能

以下功能可能作为附带插件提供

- [ ] 模糊搜索

- [ ] 将结果保存到磁盘

- [ ] 历史记录

### 插件API

- [x] read/write/dump 内存

- [x] 获取maps

- [x] 获取pid

- [ ] 获取地址列表

- [ ] 其它语言开发meow插件的 sdk

### 附带插件

- [ ] tline，简单的终端用户readline界面 (进行中)

- [ ] injection，用于注入动态库 (进行中)

- [ ] demo，编写插件的文档 (进行中)

- [ ] pointer，查找地址指针

## Thanks for free JetBrains Open Source license

<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo (Main) logo." height="200"/>