use zhscript2::{u_ as zs_, as_ref__};
use std::{process, sync::Mutex};
use lazy_static::*;

lazy_static! {
	pub static ref ZSW_: zs_::world_::T_ = zs_::world_::t__(zs_::World_::new());
	pub static ref ZSWM_:Mutex<zs_::WorldMut_> = Mutex::new(Default::default());
	pub static ref MAIN_QV_: zs_::qv_::T_ = zs_::qv_::t__(zs_::Qv_::new2(Some(as_ref__!(ZSW_).top_q_.clone())));
}

pub fn main_var__(name:&str) -> String {
	let mut ret2 = zs_::result_::List_::new();
	as_ref__!(MAIN_QV_).vars_.get__(name, false, true, &mut ret2, &mut zs_::result_::List_::new());
	let v = ret2.to_vec__();
	{if !v.is_empty() {&v[0]} else {""}}.to_string()
}

pub fn add__<S: ToString>(ret: &mut zs_::result_::List_, gd:&zs_::code_::Opt_, w:zs_::world_::T_, s: S) {
	if gd.vals_ && !ret.is_empty() {
		as_ref__!(w).dunhao__(ret)
	}
	ret.add__(s)
}

pub fn eval__(src:&str, q:zs_::qv_::T_, w:zs_::world_::T_, wm:&mut zs_::WorldMut_, ret: &mut zs_::result_::List_) -> zs_::Result2_ {
	zs_::eval_::hello2__(src, |_| {}, Default::default(), q, w, wm, ret)
}

pub fn exit__(i:i32) {
	process::exit(i);
}
pub fn if_quit__(i:i32, s:&str) {
	if i == zs_::jump_::QUIT_ {
		if s.is_empty() {
			exit__(0);
		}
		if let Some(i) = zs_::t_::s2n__(s) {
			exit__(i);
		}
	}
}

pub fn err__(i:i32, s:String, s2:String, ret:&mut String) {
	if_quit__(i, &s);
	*ret = ["<pre style='font-size:12px'>", &s, &s2, "\n", &i.to_string(), "码</pre>"].concat()
}

pub fn errexit__(i:i32, s:String, s2:String) {
	if_quit__(i, &s);
	eprintln!("{}{}", s, s2);
	exit__(zs_::result2_::exitcode__(i));
}

pub fn ierr__(i:i32, args:&Vec<String>, err:&str) -> zs_::Result2_ {
	match i {
		1 => ierr1__(args, err),
		2 => ierr3__(err, "!"),
		22 => ierr1__(args, "!"),
		23 => ierr2__(args, err, "!"),
		3 => ierr4__(err.to_string()),
		4 => ierr3__(err, "<2"),
		250 => ierr1__(args, "<"),
		_ => zs_::ok__(),
	}
}
pub fn ierr1__(args:&Vec<String>, typ:&str) -> zs_::Result2_ {
	ierr2__(args, "", typ)
}
pub fn ierr2__(args:&Vec<String>, s2:&str, typ:&str) -> zs_::Result2_ {
	let mut s = String::new();
	for i in args {
		s.push_str(&i);
		s.push_str(" ");
	}
	if !s2.is_empty() {
		s.push_str(s2);
		s.push_str(" ");
	}
	ierr3__(&s, typ)
}
pub fn ierr3__(s2:&str, typ:&str) -> zs_::Result2_ {
	let mut s = String::new();
	s.push_str(s2);
	match typ {
		"<" => s.push_str("参数不足"),
		"<2" => s.push_str("参数不配对"),
		"!" => s.push_str("不支持"),
		_ => s.push_str(typ),
	}
	ierr4__(s)
}
pub fn ierr4__(s:String) -> zs_::Result2_ {
	eprintln!("{}", s);
	zs_::result2_::err__(s)
}
