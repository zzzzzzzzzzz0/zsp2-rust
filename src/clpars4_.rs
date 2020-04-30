use zhscript2::{u_ as zs_, u2_::clpars_, as_ref__};
use super::{t_};

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, gd:&zs_::code_::Opt_,
		q:zs_::qv_::T_, w:zs_::world_::T_, wm:&mut zs_::WorldMut_, ret:&mut zs_::result_::List_) -> zs_::Result2_ {
	let mut err = String::new();
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new1z("命令行加回调"),
		clpars_::Item_::new2cz("命令行解析", 1),
		clpars_::Item_::new("命令行帮助"),
	]);
	let mut ret2 = 0;
	ret2 = cp.for__(&mut args.clone().into_iter(), |tag, argv, _item, _i3| {
		match tag {
			"命令行加回调" => {
				if argv.len() % 4 != 1 {
					err = as_ref__!(w).text__(tag);
					return 4
				}
				let mut cp2 = clpars_::List_::new();
				cp2.rem_ = argv[0].to_string();
				let mut i = 1;
				while i < argv.len() {
					let mut typ = clpars_::Typ_::Full;
					let s2 = &argv[i + 2];
					let mut argc2 = 0;
					let argc = if let Ok(i) = s2.parse::<usize>() {
						i
					} else {
						     if s2.starts_with('b') {typ = clpars_::Typ_::Starts;}
						else if s2.starts_with('e') {typ = clpars_::Typ_::Ends;}
						else if s2.starts_with('c') {typ = clpars_::Typ_::Has;}
						else if s2.starts_with('a') {
							argc2 = clpars_::ARGC_Z_
						}
						else {
							err = s2.to_string();
							return 23
						}
						let s2 = &s2[1..];
						if !s2.is_empty() {
							if let Ok(i) = s2.parse::<usize>() {
								i
							}
							else {
								err = s2.to_string();
								return 23
							}
						} else {
							0
						}
					};
					let rem = &argv[i + 1];
					let mut code = &argv[i + 3];
					if code.is_empty() {
						code = rem
					}
					cp2.a_.push(clpars_::Item_::new__(&argv[i], argc + argc2, typ,
						clpars_::Cb_::S(code.to_string()), rem));
					i += 4
				}
				if !ret.is_empty() {
					as_ref__!(w).dunhao__(ret)
				}
				ret.add_obj__(Box::new(cp2));
			}
			"命令行解析" => {
				if !get__(q.clone(), |cp2| {
					ret2 = cp2.for__(&mut args.clone().into_iter().skip(2), |tag, argv, item, _i3| {
						if let clpars_::Cb_::S(src) = &item.cb_ {
							if !src.is_empty() {
								let mut q = zs_::Qv_::new2(Some(q.clone()));
								{
									let args = &mut q.args_;
									for i in argv {
										if !args.is_empty() {
											as_ref__!(w).dunhao__(args);
										}
										args.add__(i)
									}
								}
								q.src_ = tag.to_string();
								if let Err((i, s, s2)) = t_::eval__(&src, zs_::qv_::t__(q), w.clone(), wm, ret) {
									t_::err__(i, s, s2, &mut err);
									return 3
								}
								return 0
							}
						}
						match tag {
							"-h" | "--help" => {
								println!("{}", cp2.help__());
								t_::exit__(251);
							}
							_ => {}
						}
						0
					}, |_| 3);
				}) {return 2}
			}
			"命令行帮助" => {
				if !get__(q.clone(), |cp2| {
					t_::add__(ret, gd, w.clone(), cp2.help__());
				}) {return 2}
			}
			_ => {}
		}
		ret2
	}, |s| {
		args2.push(s.to_string());
		0
	});
	t_::ierr__(ret2, args, &err)
}

fn get__(q:zs_::qv_::T_, f:impl FnMut(&clpars_::List_)) -> bool {
	let args2 = &as_ref__!(q).args_;
	let mut i2 = 0;
	for i in args2.iter() {
		i2 += 1;
		if as_ref__!(i).dunhao__() {
			break
		}
	}
	args2.obj__(i2, f)
}

