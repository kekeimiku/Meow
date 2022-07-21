## 多平台内存扫描器

> **开发中** 暂时不建议使用，开发缓慢，业余时间瞎几把写中

## ⚠️ WIP

## TL;DR

一个轻量级跨平台内存调试器，主要功能为在内存中查找数据。

优势：

- 非常小巧，尽量不使用任何第三方库

- 速度快，兼顾内存与cpu占用尽量低

- 易于使用，方便二次开发，核心可导出为动态库

## TODO

meow 分为 core 以及 plugin 两部分，core 为各平台提供 读/写/查找 相关内存的功能，plugin 基于动态库，公开了一些 core api 用于方便用户自定义一些插件，插件允许闭源分发。

### 核心功能

- [x] 查找数据地址

- [x] 过滤找到的地址

- [x] read/write/dump 内存

- [x] 解析进程内存信息

- [ ] 查找指针 (进行中)

- [ ] 插件管理器 (进行中)

### 争议性核心功能

以下功能可能作为插件或编译时features提供

- [ ] 模糊搜索

- [ ] 将结果保存到磁盘

- [ ] 历史记录

### 插件API

- [x] read/write/dump 内存

- [x] 获取进程内存信息

- [ ] 获取地址列表

- [ ] 其它语言开发meow插件的支持

### 附带插件

- [ ] injection，用于注入动态库 (进行中)

- [ ] demo，编写插件的文档 (进行中)

- [ ] tview 一个终端视图，用于直接可视化查看与编辑指定地址附近区域

## Thanks for free JetBrains Open Source license

<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo (Main) logo." height="200"/>