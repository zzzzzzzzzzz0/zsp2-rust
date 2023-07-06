赋予标题前以控制面板。

赋予mplayer、env以加载env.zs、mplayer。

别名标题以视频‘号’标题【号】。

如果‘noui’否则先
%>
<title><%“‘标题前’ - ‘功用’ - ‘网站标题’”%></title>
<style>
<%我的文件、1.css%>
</style>
<script>
<%我的文件、1.js%>
function cmd__(s) {
	ajax__('ctl.zsp?c=' + s);
	switch(s) {
		case 'quit':
			window.setTimeout('window.close()', 100);
			break;
	}
}
</script>
<div class="item2 item-1 item-2">
<div><span class="txt size2"><%‘标题前’%></span></div>
<div class=info>
<%
如果不存在标题那么返回。
%>
<div><span class="txt size2"><%‘标题’%></span></div>
<%
如果不‘mplayer’那么先
	“<pre class=txt>须服务端安装 mplayer 方能‘功用’</pre>”。
	返回。
了。

赋予解以我的命令行加回调、、
	-、、0、““<div class=bar>”换行”、
	/、、0、““</div>”换行”、
	-拖条、、5、下代码
		别名签、最小、最大、默认值、js以参数1、参数2、参数3、参数4、参数5。
		赋予js以cmd__(‘js’)。
		“<span class=txt>‘签’ ‘最小’</span>
		<input type="range" min="‘最小’" max="‘最大’" step="1" value="‘默认值’" oninput="‘js’" onchange="‘js’" />
		<span class=txt>‘最大’</span>”
	上代码、
	-a、、a、下代码
		赋予量以算术‘参数数目’/2。
		赋予class、前、后以class=play如果‘量’大于1那么先
			赋予“ ”以&nbsp;。
			赋予填以“<span class=sp> ‘ ’ </span>”。
			2、“‘ ’‘ ’‘填’”、“‘填’‘ ’‘ ’”
		了否则、。
		‘前’循环【‘量’】【次】先
			如果‘次’大于1那么“<span class=sp>‘ ’ / ‘ ’</span>”。
			别名 文 以参数算术(‘次’-1)*2+1。
			别名命令以参数算术(‘次’-1)*2+2。
			“<a ‘class’ href="javascript:cmd__('‘命令’')">‘文’</a>”
		了‘后’
		换行
	上代码、
	-切换、、2、下代码
		别名命令、文以参数2、参数1。
		我的命令行解析、‘解’、-a、‘文’、“‘命令’ 1”、取消、“‘命令’ 0”。
	上代码、
	、、2、下代码
		别名命令、文以参数1、参数0。
		我的命令行解析、‘解’、-a、‘文’、‘命令’。
	上代码。
我的命令行解析、‘解’、-、
	“暂停 / 取消”、pause、
	-切换、全屏、vo_fullscreen、
	关闭、quit、
	/、-、
	-拖条、音量、0、100、‘默认音量’、“'volume ' + this.value + ' 1'”、
	/、-、
	音量减10%、“volume -10”、
	音量加10%、“volume +10”、
	切换音轨、switch_audio、
	切换字幕、sub_select、
	/、-、
	后退10分钟、“seek -600”、
	前跳10分钟、“seek +600”、
	/、-、
	后退1分钟、“seek -60”、
	前跳1分钟、“seek +60”、
	/、-、
	后退10秒、“seek -10”、
	前跳10秒、“seek +10”、
	/、-、
	速度减半、“speed_mult 0.5”、
	速度增倍、“speed_mult 2”、
	原速播放、“speed_set 1”、
	/。
%>
</div>
</div>
<%了。

如果存在器柄【桌面】那么先
	显示器柄‘器柄’换行。
	如果‘器柄’那么返回。
了。
赋予号【桌面】、器号【桌面】、功用【桌面】、器柄【桌面】以‘号’、、‘功用’、执行解释下代码
	如果‘-h2’那么显示‘参数’换行。
	‘参数栈’
上代码、“‘mplayer’ -slave -ontop -quiet -subcp UTF-8 -ao pulse -title "‘网站标题’ 《‘标题’》" ”分叉‘功用’先
		桌面全屏播放“-fs ”。
		先“-geometry "‘默认位置’" ”了
	了"‘视频‘号’’"
	先解释‘env’了、
	-被动者2、““volume ‘默认音量’ 1”换行”。
%>