use zhscript2::{u_ as zs_, u2_::clpars_, as_ref__};
use zs2_l4_::clpars4_;
use super::{t_};

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>,
		env:&zs_::code_::Env_, wm:&mut zs_::WorldMut_, ret:&mut zs_::result_::List_) -> zs_::Result2_ {
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new1z("命令行加回调"),
		clpars_::Item_::new2cz("命令行解析", 1),
		clpars_::Item_::new1z("命令行帮助"),
	]);
	let mut ret2 = zs_::ok__();
	cp.for__(&mut args.clone().into_iter(), |tag, argv, _item, _i3| {
		match tag {
			"命令行加回调" => {
				match clpars4_::set2__(argv) {
					Ok(cp2) => {
						if !ret.is_empty() {
							as_ref__!(env.w).dunhao__(ret)
						}
						ret.add_obj__(Box::new(cp2));
					}
					Err(s2) => {
						ret2 = zs_::result2_::err__(s2);
						return /*23*/4
					}
				}
			}
			"命令行解析" => {
				if let Err((i, ret3)) = clpars4_::par__(2, 2, env, wm, ret) {
					match i {
						251 => t_::exit__(i),
						_ => ret2 = ret3,
					}
				}
			}
			"命令行帮助" =>
				clpars4_::help__(2, env.q.clone(), ret),
			_ => {}
		}
		0
	}, |s| {
		args2.push(s.to_string());
		0
	});
	ret2
}
