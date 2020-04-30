function id__(id) {
	return document.getElementById(id);
}

function htm__(e, val, op) {
	if(val == undefined)
		return e.innerHTML;
	switch(op) {
	case undefined:
		e.innerHTML = val;
		return;
	case true:
		e.innerHTML += val;
		return;
	case "top":
		e.innerHTML = val + e.innerHTML;
		return;
	}
}

function new__(s) {
	return document.createElement(s);
}
function add__(e, e2) {
	e2.appendChild(e); 
}
function del__(e) {
	e.parentNode.removeChild(e); 
};
function ins__(e, e2, i) {
	if(i == undefined)
		e2.parentNode.insertBefore(e, e2); 
	else
		e2.insertBefore(e, e2.childNodes[i]);
};

function ajax__(s, f, o) {
	var hr = new XMLHttpRequest();
	if(f) {
		hr.onreadystatechange = function() {
			if(hr.readyState == 4 && hr.status == 200) {
				f(hr.responseText, o);
			}
		}
	}
	hr.open("GET", s, true);
	hr.send();
}
