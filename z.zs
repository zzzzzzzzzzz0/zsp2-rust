#!/usr/lib/zhscript2/l --。
赋予调试【顶】以。
定义提示、内容以下代码
	显示“# ‘内容’”换行。
上代码。
定义运行、命令、饶以下代码
	提示‘命令’。
	如果‘调试’那么提示未‘参数0’，退出。
	赋予错以执行‘命令’。
	提示‘错’。
	如果‘错’并且不‘饶’那么结束10。
上代码。
定义行解以下代码
	调用‘命令行解析’、‘参数栈’。
上代码。

加载lib/clpars4。
调用‘命令行加回调’、
	r|r1|r2|r0|d1|d2|d3、下原样
		运行分支‘参数0’先
			d3“RUST_BACKTRACE=1 ”。
		了“rust.zs2 ---- r -- ”分支‘参数0’先
			d1“-zhscript-d-bp-r_v-”。
			d2“-zhscript-d-tree”。
			d3“-zhscript-d-lc”。
		了“ -zsp-”分支‘参数0’先
			r：help。
			先
				管道堵执行reset。
				“conf ”分支‘参数0’先
					r1、r0、d1先
						demo/分支‘参数0’先
							movbrow。
							r0：lose。
						了/conf.zs
					了。
					d3：/zzzzzzzzzzz4/home/zzzzzzzzzzz/test/rust-zsp/3.zs。
				了
			了
		了“ ‘参数’”上原样、a、、
	t4、、0、下代码
		循环10000次先
			显示‘次’如果算术‘次’%10等于0那么换行否则制表符。
			执行“curl 127.0.0.1:8084 > /dev/null 2>&1 &”。
		了。
	上代码、
	br、下代码
		运行“rust.zs2 ‘参数0’”
	上代码、0、、
	mvr、下代码
		运行“mv ./target/release/zsp2 /zzzzzzzzzzz4/usr/lib/zhscript2-rust/zsp”
	上代码、0、、
	-h2、“赋予调试【顶】以1。”、0、、
	#、、h、。
行解‘参数栈’。
