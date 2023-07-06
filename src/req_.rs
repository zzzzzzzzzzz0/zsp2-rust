use zhscript2::{u_ as zs_, u2_::clpars_, as_ref__, as_mut_ref__};
use super::{t_};
use actix_web::{HttpRequest};
use std::{collections::HashMap};

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, env:&zs_::code_::Env_) -> zs_::Result2_ {
	let mut ret2 = zs_::ok__();
	if let Some(req) = &as_ref__!(env.q).obj__::<HttpRequest>(0) {
		let cp = clpars_::List_::new2(vec![
			clpars_::Item_::new1z("参|餐"),
			clpars_::Item_::new2c("遍历参", 1),
			clpars_::Item_::new("url"),
			clpars_::Item_::new1z("地址"),
		]);
		let _ = cp.for3__(&mut args.clone().into_iter(), |tag, argv, _, _, _, _| {
			match tag {
				"参" | "餐" | "遍历参" => {
					let s = req.query_string();
					if tag == "遍历参" {
						let hm: HashMap<_, _> = url::form_urlencoded::parse(s.as_bytes()).into_owned().collect();
						for (k, v) in &hm {
							let q = zs_::Qv_::new2(as_ref__!(env.q).up_.clone());
							{
								let args = &mut as_mut_ref__!(q.args_);
								args.add__(k);
								as_ref__!(env.w).dunhao__(args);
								args.add__(v);
							}
							ret2 = zs_::eval_::hello__(&argv[0], &mut zs_::code_::Env_::new2(zs_::t__(q.clone()), env));
							if ret2.is_err() {return 1}
						}
						return 0
					}
					if argv.is_empty() {
						t_::add__(env, s)
					} else {
						let hm: HashMap<_, _> = url::form_urlencoded::parse(s.as_bytes()).into_owned().collect();
						for i in argv {
							t_::add__(env, if let Some(s4) = hm.get(i) {s4} else {""})
						}
					}
				}
				"url" => t_::add__(env, req.uri()),
				"地址" => {
					if let Some(pa) = req.peer_addr() {
						let mut s = pa.to_string();
						if !argv.is_empty() {
							let s2 = &argv[0];
							let i2 = if let Some(i) = s2.find("://") {
								s.insert_str(0, &s2[0..i + 3]);
								i
							} else {
								usize::MAX
							};
							if let Some(i) = s2.rfind(':') {
								if i != i2 {
									if let Some(i) = s.rfind(':') {
										s = s[0..i].to_string();
									}
									s.push_str(&s2[i..]);
								}
							}
						}
						t_::add__(env, s);
					};
					//req.connection_info().realip_remote_addr()
				}
				_ => {}
			}
			0
		}, |s| {
			args2.push(s.to_string());
			0
		});
	} else {
		for i in args {
			args2.push(i.to_string())
		}
	}
	ret2
}