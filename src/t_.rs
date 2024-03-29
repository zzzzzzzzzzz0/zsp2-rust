use zhscript2::{u_ as zs_, as_ref__, as_mut_ref__};
use zs2_l4_;
use std::{process, };
use lazy_static::*;

lazy_static! {
	pub static ref ZSW_: zs_::world_::T_ = zs2_l4_::i_::w__();
	pub static ref MAIN_QV_: zs_::qv_::T_ = zs_::t__(zs_::Qv_::new3("主", Some(as_ref__!(ZSW_).top_q_.clone())));
}

pub fn main_var__(name:&str) -> String {
	let mut ret2 = zs_::result_::List_::new();
	as_ref__!(MAIN_QV_).vars_.get__(name, false, true, &mut ret2);
	let v = ret2.to_vec__();
	{if !v.is_empty() {&v[0]} else {""}}.to_string()
}

pub fn add__<S: ToString>(env:&zs_::code_::Env_, s: S) {
	let mut ret = as_mut_ref__!(env.ret);
	if env.gd.vals_ && !ret.is_empty() {
		as_ref__!(env.w).dunhao__(&mut ret)
	}
	ret.add__(s)
}

pub fn eval__(src:&str, env:&zs_::code_::Env_) -> zs_::Result2_ {
	zs_::eval_::hello2__(src, |_| {}, env)
}

pub fn exit__(i:i32) {
	process::exit(i);
}
pub fn if_quit__(i:i32, i2:i32, s:&String) {
	if i == zs_::jump_::QUIT_ && i2 != zs_::jump_::NO_ {
		eprint!("{}", s);
		exit__(i2);
	}
}

pub fn err__(i:i32, i2:i32, s:String, s2:String, ret:&mut String) {
	if_quit__(i, i2, &s);
	*ret = ["<pre style='font-size:12px'>", &s, &s2, "</pre>"].concat()
}

pub fn errexit__(i:i32, i2:i32, s:String, s2:String) {
	if_quit__(i, i2, &s);
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
	//eprintln!("{}", s);
	zs_::result2_::err__(s)
}
