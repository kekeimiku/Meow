linux（包括android）版 cheat engine （支持图形界面，web和命令行操作并且公开abi） 龟速开发中

暂时业余时间瞎几把写中

距离可用还有很远的距离

目前还比较混乱。。

如果有人想贡献代码，请注意，不要出现任何unwrap,尽量不要引入第三方库，除了rust自身依赖的，目前本项目没有使用任何第三方库。

目录结构

	`elf` 二进制文件解析器
	
	`decode` arm64反汇编器

	`memscan` 内存读写与模糊搜索
	
	`gui` 包含web gui tui

	`...` 待定...

## Thanks for free JetBrains Open Source license

<img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_beam.png" alt="JetBrains Logo (Main) logo." height="200"/>
