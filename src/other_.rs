use zhscript2::{u_ as zs_, u2_::clpars_, as_ref__};
use super::{t_};
use std::{env, thread, time::Duration};
use regex::Regex;
use rand::Rng;

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, gd:&zs_::code_::Opt_, q:zs_::qv_::T_,
		w:zs_::world_::T_, wm:&mut zs_::WorldMut_, ret:&mut zs_::result_::List_) -> zs_::Result2_ {
	let mut err = String::new();
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new2cz("正则配", 2),
		clpars_::Item_::new2c("正则代", 3),
		clpars_::Item_::new2c("随机数", 2),
		clpars_::Item_::new1z("命令行加回调"),
		clpars_::Item_::new2cz("命令行解析", 1),
		clpars_::Item_::new2c("环境变量", 1),
		clpars_::Item_::new2c("等待", 1),
	]);
	t_::ierr__(cp.for__(&mut args.clone().into_iter(), |tag, argv, _item, _i3| {
		match tag {
			"正则配" => {
				let end = argv.len() - 1;
				match Regex::new(&argv[end]) {
					Ok(re) =>
						for idx in 0..end {
							let i = &argv[idx];
							if re.is_match(i) {
								t_::add__(ret, gd, w.clone(), "1");
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
							let mut q = zs_::Qv_::new2(Some(q.clone()));
							let args = &mut q.args_;
							for idx in 1..cm.len() {
								if !args.is_empty() {
									as_ref__!(w).dunhao__(args);
								}
								args.add__(&cm[idx]);
							}
							if let Err((i, s, s2)) = t_::eval__(&src, zs_::qv_::t__(q), w.clone(), wm, ret) {
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
				gd:&zs_::code_::Opt_, w:zs_::world_::T_, ret:&mut zs_::result_::List_) -> Result<bool, String> {
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
										t_::add__(ret, gd, w, rand::thread_rng().gen_range(min, max));
										Ok(true)
									}
								}
								Err(_) => if er {Err(argv[1].to_string())} else {Ok(false)}
							}
						}
						Err(_) => if er {Err(argv[0].to_string())} else {Ok(false)}
					}
				}
				if let Ok(true) = gen__::<u64>(argv, false, true, gd, w.clone(), ret) {return 0}
				if let Ok(true) = gen__::<i64>(argv, false, true, gd, w.clone(), ret) {return 0}
				if let Err(e) = gen__::<f64>(argv, true, false, gd, w.clone(), ret) {err = e; return 3}
			}
			"环境变量" =>
				if let Ok(s) = env::var(&argv[0]) {
					t_::add__(ret, gd, w.clone(), s)
				},
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
	}), args, &err)
}