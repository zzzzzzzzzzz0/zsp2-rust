别名定时器.1、定时器.2以参数1、参数2。

赋予框宽、图宽、图高以1、150。
赋予项宽以算术‘图宽’+‘框宽’*2。
赋予项高以算术‘图高’+‘框宽’*2。
赋予图宽、图高、项宽、项高以‘图宽’px、‘图高’px、‘项宽’px、‘项高’px。
%>
<style>
.item-1 {
	width: <%‘项宽’%>;
	height: <%‘项高’%>;
	font-size:4px; color:#000;
}
.item-2 {
	width: <%‘项宽’%>;
	height: <%‘项高’%>;
	display:table-cell;
}
.item img {
	max-width: <%‘图宽’%>;
	max-height: <%‘图高’%>;
	border:none;
}
</style>
<script>
<%我的文件、1.js%>
</script>
<div id=list></div>
<script>
function item__(id, href, htm1, htm2, att) {
	var d = new__("div");
	d.className = "item";
	d.id = id;
	htm__(d,
		'<div class=item-1>' +
		'<a href="' + href + '" ' + att + '>' +
		'<div class=item-2>' + htm1 + '</div>' +
		'</a>' +
		(htm2 ? htm2 : '') + '</div>'
		);
	return d;
}

add__(item__('loading', 'javascript:', '<img src="loading.gif">'), id__("list"));

var int_i_ = 1;
var int_old_line_;
<%‘定时器.2’%>
var int_ = setInterval(function() {
	var url = "get.zsp?i=" + <%‘定时器.1’%>;
	ajax__(url, function(line) {
		if("" == line || int_old_line_ == line)
			return;
		int_old_line_ = line;

		var o = eval("(" + line + ")");
		switch(o.type) {
		case "x":
			del__(id__("loading"));
			clearInterval(int_);
			int_old_line_ = undefined;
			return;
		case "no":
			return;
		}

		var htm1, htm2;
		if(o.img) {
			htm1 = '<img src="get.zsp?c=img&i=' + o.id + '">';
			htm2 = o.title;
		} else {
			htm1 = '<span class=txt>' + o.title + '</span>';
		}
		ins__(item__(int_i_, 'show.zsp?i=' + o.id, htm1, htm2, 'target=_blank'), id__("loading"));
		int_i_++;
	});
}, 200);
</script>
