use zhscript2::{u_ as zs_, u2_::clpars_, as_ref__};
use super::{t_};
use actix_web::{HttpRequest};
use std::{collections::HashMap};

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, env:&zs_::code_::Env_, ret:&mut zs_::result_::List_) -> zs_::Result2_ {
	if let Some(req) = &as_ref__!(env.q).obj__::<HttpRequest>(0) {
		let cp = clpars_::List_::new2(vec![
			clpars_::Item_::new1z("参|餐"),
			clpars_::Item_::new("url"),
		]);
		t_::ierr__(cp.for__(&mut args.clone().into_iter(), |tag, argv, _item, _i3| {
			match tag {
				"参" | "餐" => {
					let s = req.query_string();
					if argv.is_empty() {
						t_::add__(ret, env, s)
					} else {
						let hm: HashMap<_, _> = url::form_urlencoded::parse(s.as_bytes()).into_owned().collect();
						for i in argv {
							t_::add__(ret, env, if let Some(s4) = hm.get(i) {s4} else {""})
						}
					}
				}
				"url" => t_::add__(ret, env, req.uri()),
				_ => {}
			}
			0
		}, |s| {
			args2.push(s.to_string());
			0
		}), args, "")
	} else {
		for i in args {
			args2.push(i.to_string())
		}
		zs_::ok__()
	}
}