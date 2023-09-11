#![allow(unused_imports, unused_variables, unused_mut)]

use zhscript2::{u_::{self as zs_, }, u2_::clpars_, as_ref__, as_mut_ref__};
use zs2_l4_::thread4_;
use super::{t_};
use std::{thread, io::{BufReader, BufRead}, process::ChildStdout};

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, env:&zs_::code_::Env_) -> zs_::Result2_ {
	let mut err = String::new();
	let f__ = |ret2| {
		if let Err((i, i2, s, s2)) = ret2 {
			zs_::result2_::eprtn__(i, i2, &s, &s2);
			true
		} else {false}
	};
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new2cz("程序1|程序2|背解|解背", 1),
	]);
	let ret3 = cp.for3__(&mut args.clone().into_iter(), |tag, argv, _, _, _, _| {
		match tag {
			"程序1" => {
				let mut argv = argv.to_vec();
				let env2 = zs_::code_::Env_::new10(env);
				thread::spawn(move || {
					let over_src = argv.remove(0);
					let exec = zhscript2::exec_::Item_::new(&as_ref__!(env2.w).kws_);
					if f__(exec.hello2__(argv, &env2)) {return}
					if f__(t_::eval__(&over_src, &env2)) {return}
				});
			}
			"程序2" => {
				let mut argv = argv.to_vec();
				let over_src = argv.remove(0);
				let exec = zhscript2::exec_::Item_::new(&as_ref__!(env.w).kws_);
				if let Err((i, i2, s, s2)) = exec.hello2__(argv, env) {
					t_::err__(i, i2, s, s2, &mut err);
					return 3
				}
				match exec.obj__(&as_ref__!(env.ret), 0, |obj, _, _| {
					#[cfg(debug_assertions)]
					{
						let mut o = obj.o_.take();
						let o = o.as_mut();
						/*if let Some(mut stdout1) = o {
							let mut stdout = BufReader::new(stdout1);
							let env2 = zs_::code_::Env_::new10(env);
							let over_src = over_src.clone();
							thread::spawn(move || {
								let mut s = String::new();
								loop {
									match stdout.read_line(&mut s) {
										Ok(siz) => {
											if siz == 0 {
												break
											}
											if f__(t_::eval__(&over_src, &env2)) {}
										}
										Err(e) => eprintln!("{}", e)
									}
								}
							});
							return zs_::ok__()
						}*/
					}
					zs_::result2_::err2__("! stdout")
				}) {
					Some(Ok(_)) => {
					}
					Some(Err((i, i2, s, s2))) => {
						t_::err__(i, i2, s, s2, &mut err);
						return 3
					}
					None => {
						err = "须有 -被动者2".to_string();
						return 3
					}
				}
			}
			"背解" | "解背" =>
				if thread4_::start__(argv, env).is_ok() {}
			_ => {}
		}
		0
	}, |s| {
		args2.push(s.to_string());
		0
	});
	t_::ierr__(if let Err((i, _)) = ret3 {i} else {0}, args, &err)
}
