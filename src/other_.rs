use zhscript2::{u_::{self as zs_, }, u2_::clpars_};
use zs2_l4_::{regexpr4_, forqv_};
use super::{t_};
use std::{env, thread, time::Duration};
use rand::Rng;

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, env:&zs_::code_::Env_) -> zs_::Result2_ {
	let mut err = String::new();
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new2cz("正则配", 2),
		clpars_::Item_::new2c("正则代", 3),
		clpars_::Item_::new2c("正则迭", 3),
		clpars_::Item_::new2c("正则替", 3),
		clpars_::Item_::new2c("遍历正则", 3),
		clpars_::Item_::new2c("随机数", 2),
		clpars_::Item_::new2c("遍历区", 2),
		clpars_::Item_::new2c("遍历区2", 3),
		clpars_::Item_::new2c("环境变量", 1),
		clpars_::Item_::new2c("等待", 1),
	]);
	let mut ret2 = zs_::ok__();
	let ret3 = cp.for3__(&mut args.clone().into_iter(), |tag, argv, _, _, _, _| {
		match tag {
			"正则配" => {
				match regexpr4_::test__(argv) {
					Ok(b) =>
						if b {
							t_::add__(env, "1")
						},
					Err(e) => {ret2 = e; return 3}
				}
			}
			"随机数" => {
				fn gen__<F: std::str::FromStr + rand::distributions::uniform::SampleUniform +
				std::fmt::Display + PartialOrd + std::ops::AddAssign>(argv:&[String], er:bool, is_i:bool,
				env:&zs_::code_::Env_) -> Result<bool, String> {
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
										t_::add__(env, rand::thread_rng().gen_range(min, max));
										Ok(true)
									}
								}
								Err(_) => if er {Err(argv[1].to_string())} else {Ok(false)}
							}
						}
						Err(_) => if er {Err(argv[0].to_string())} else {Ok(false)}
					}
				}
				if let Ok(true) = gen__::<u64>(argv, false, true, env) {return 0}
				if let Ok(true) = gen__::<i64>(argv, false, true, env) {return 0}
				if let Err(e) = gen__::<f64>(argv, true, false, env) {err = e; return 3}
			}
			"遍历区" | "遍历区2" => ret2 = forqv_::z__(env, 2),
			"环境变量" =>
				if let Ok(s) = env::var(&argv[0]) {
					t_::add__(env, s)
				},
			"等待" =>
				match argv[0].parse::<f32>() {
					Ok(i) =>
						thread::sleep(Duration::from_secs_f32(i)),
					Err(e) => {err = e.to_string(); return 3}
				},
			_ => {
				ret2 = regexpr4_::for__(argv, match tag {
					"正则代" => 1,
					"正则迭" => 11,
					"正则替" => 10,
					_ => 0
				}, env)
			}
		}
		0
	}, |s| {
		args2.push(s.to_string());
		0
	});
	if ret2.is_err() {
		return ret2
	}
	t_::ierr__(if let Err((i, _)) = ret3 {i} else {0}, args, &err)
}