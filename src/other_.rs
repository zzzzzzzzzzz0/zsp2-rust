use zhscript2::{u_::{self as zs_, }, u2_::clpars_, as_ref__};
use super::{t_};
use std::{env, thread, time::Duration, io::{BufReader, BufRead}, process::ChildStdout};
use regex::Regex;
use rand::Rng;

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>,
		env:&zs_::code_::Env_, wm:&mut zs_::WorldMut_, ret:&mut zs_::result_::List_) -> zs_::Result2_ {
	let mut err = String::new();
	let f__ = |ret2| {
		if let Err((i, s, s2)) = ret2 {eprintln!("{} {} {}", i, s, s2); return true}
		false
	};
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new2cz("正则配", 2),
		clpars_::Item_::new2c("正则代", 3),
		clpars_::Item_::new2c("随机数", 2),
		clpars_::Item_::new2c("环境变量", 1),
		clpars_::Item_::new1z("程序1 | 程序2"),
		clpars_::Item_::new2c("等待", 1),
	]);
	let i = cp.for__(&mut args.clone().into_iter(), |tag, argv, _item, _i3| {
		match tag {
			"正则配" => {
				let end = argv.len() - 1;
				match Regex::new(&argv[end]) {
					Ok(re) =>
						for idx in 0..end {
							let i = &argv[idx];
							if re.is_match(i) {
								t_::add__(ret, env, "1");
								return 0
							}
						},
					Err(e) => {err = e.to_string(); return 3}
				}
			}
			"正则代" => {
				match Regex::new(&argv[1]) {
					Ok(re) => {
						let txt = &argv[0];
						let src = &argv[2];
						for cm in re.captures_iter(txt) {
							let mut q = zs_::Qv_::new2(Some(env.q.clone()));
							let args = &mut q.args_;
							for idx in 1..cm.len() {
								if !args.is_empty() {
									as_ref__!(env.w).dunhao__(args);
								}
								args.add__(&cm[idx]);
							}
							if let Err((i, s, s2)) = t_::eval__(&src,
							&zs_::code_::Env_::new2(zs_::qv_::t__(q), env), wm, ret) {
								t_::err__(i, s, s2, &mut err);
								return 3
							}
						}
					}
					Err(e) => {err = e.to_string(); return 3}
				}
			}
			"随机数" => {
				fn gen__<F: std::str::FromStr + rand::distributions::uniform::SampleUniform +
				std::fmt::Display + PartialOrd + std::ops::AddAssign>(argv:&[String], er:bool, is_i:bool,
				env:&zs_::code_::Env_, ret:&mut zs_::result_::List_) -> Result<bool, String> {
					match argv[0].parse::<F>() {
						Ok(min) => {
							match argv[1].parse::<F>() {
								Ok(mut max) => {
									//let max = max + 1;
									if is_i {
										if let Ok(i) = F::from_str("1") {
											max += i;
										}
									}
									if min >= max {
										Err([&max.to_string(), "不大于", &min.to_string()].concat())
									} else {
										t_::add__(ret, env, rand::thread_rng().gen_range(min, max));
										Ok(true)
									}
								}
								Err(_) => if er {Err(argv[1].to_string())} else {Ok(false)}
							}
						}
						Err(_) => if er {Err(argv[0].to_string())} else {Ok(false)}
					}
				}
				if let Ok(true) = gen__::<u64>(argv, false, true, env, ret) {return 0}
				if let Ok(true) = gen__::<i64>(argv, false, true, env, ret) {return 0}
				if let Err(e) = gen__::<f64>(argv, true, false, env, ret) {err = e; return 3}
			}
			"环境变量" =>
				if let Ok(s) = env::var(&argv[0]) {
					t_::add__(ret, env, s)
				},
			"程序1" => {
				let mut argv = argv.to_vec();
				thread::spawn(move || {
					let (mut ret, env2, /*wm*/) = t_::env2__();
					let wm = &mut t_::ZSWM_.lock().unwrap();
					let over_src = argv.remove(0);
					let exec = zhscript2::exec_::Item_::new(&as_ref__!(env2.w).kws_);
					if f__(exec.hello2__(argv, &env2, wm, &mut ret)) {return}
					if f__(t_::eval__(&over_src, &env2, wm, &mut ret)) {return}
				});
			}
			"程序2" => {
				let mut argv = argv.to_vec();
				let over_src = argv.remove(0);
				let exec = zhscript2::exec_::Item_::new(&as_ref__!(env.w).kws_);
				if let Err((i, s, s2)) = exec.hello2__(argv, env, wm, ret) {
					t_::err__(i, s, s2, &mut err);
					return 3
				}
				match exec.obj__(&ret, 0, |obj, _ret2, _end| {
					if let Some(mut stdout) = obj.o_.as_mut() {
						let mut stdout = BufReader::new(stdout);
						/*thread::spawn(move || {
							let mut s = String::new();
							loop {
								match stdout.read_line(&mut s) {
									Ok(siz) => {
										if siz == 0 {
											let (mut ret, env2, /*wm*/) = t_::env2__();
											let wm = &mut t_::ZSWM_.lock().unwrap();
											if f__(t_::eval__(&over_src, &env2, wm, &mut ret)) {}
											break
										}
									}
									Err(e) => eprintln!("{}", e)
								}
							}
						});
						return zs_::ok__()*/
					}
					zs_::result2_::err2__("! stdout")
				}) {
					Some(Ok(_)) => {}
					Some(Err((i, s, s2))) => {
						t_::err__(i, s, s2, &mut err);
						return 3
					}
					None => {
						err = "须有 -被动者2".to_string();
						return 3
					}
				}
			}
			"等待" =>
				match argv[0].parse::<f32>() {
					Ok(i) =>
						thread::sleep(Duration::from_secs_f32(i)),
					Err(e) => {err = e.to_string(); return 3}
				},
			_ => {}
		}
		0
	}, |s| {
		args2.push(s.to_string());
		0
	});
	t_::ierr__(i, args, &err)
}