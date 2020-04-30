use zhscript2::{u_ as zs_, u2_::clpars_, as_ref__};
use super::{t_};
use std::{env, path::{Path, PathBuf}, fs, fs::File, io::Read, thread};
use regex::Regex;

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, gd:&zs_::code_::Opt_,
		q:zs_::qv_::T_, w:zs_::world_::T_, wm:&mut zs_::WorldMut_, ret:&mut zs_::result_::List_) -> zs_::Result2_ {
	let mut err = String::new();
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new2t("文件有", clpars_::Typ_::Starts),
		clpars_::Item_::new2c("文件", 1),
		clpars_::Item_::new2c("主名", 1),
		clpars_::Item_::new2c("目录", 1),
		clpars_::Item_::new2cz("遍历目录", 2),
		clpars_::Item_::new2c("改变目录", 1),
	]);
	t_::ierr__(cp.for__(&mut args.clone().into_iter(), |tag, argv, item, _i3| {
		match item.tag_.as_str() {
			"文件有" => {
				if Path::new(&tag[item.tag_.len()..]).exists() {
					t_::add__(ret, gd, w.clone(), "1")
				}
				return 0
			}
			_ => {}
		}
		match tag {
			"文件" => {
				match File::open(&argv[0]) {
					Ok(mut f) => {
						let mut buf = String::new();
						match f.read_to_string(&mut buf) {
							Ok(_) => t_::add__(ret, gd, w.clone(), buf),
							Err(e) => {
								err = e.to_string();
								return 3
							}
						}
					}
					Err(e) => {
						err = e.to_string();
						return 3
					}
				}
			}
			"主名" => {
				let s = &argv[0];
				if let Some(mut i) = s.rfind('/') {
					i += 1;
					if let Some(i2) = s.rfind('.') {
						if i2 > i {
							t_::add__(ret, gd, w.clone(), &s[i..i2]);
							return 0
						}
					}
					t_::add__(ret, gd, w.clone(), &s[i..])
				}
			}
			"目录" => {
				let p = Path::new(&argv[0]);
				if let Some(p) = p.parent() {
					t_::add__(ret, gd, w.clone(), p.display())
				}
			}
			"遍历目录" => {
				#[derive(Default)]
				struct O_ {
					path_:Vec<String>,
					src_:String,
					match_:Option<Regex>,
					other_:Vec<String>,
					path2_:Vec<(String, String)>,
					src2_:String,
					out_src_:String,
					no_child_:bool,
					bg_:bool,
				}
				let mut o:O_ = Default::default();
				let cp = clpars_::List_::new2(vec![
					clpars_::Item_::new2c("-路径", 1),
					clpars_::Item_::new2c("-配", 1),
					clpars_::Item_::new("-标题"),
					clpars_::Item_::new3ct("-同名", 1, clpars_::Typ_::Starts),
					clpars_::Item_::new2c("-总装", 1),
					clpars_::Item_::new2c("-见外", 1),
					clpars_::Item_::new("-仅此"),
					clpars_::Item_::new("-背后"),
					clpars_::Item_::new0(),
				]);
				match cp.for__(&mut argv.clone().into_iter(), |tag, argv, item, _i3| {
					match item.tag_.as_str() {
						"-同名" => {
							o.path2_.push((tag[item.tag_.len()..].to_string(), argv[0].to_string()));
							return 0
						}
						_ => {}
					}
					match tag {
						"-路径" => o.src_ = argv[0].to_string(),
						"-配" => {
							let s = &argv[0];
							match Regex::new(s) {
								Ok(re) => o.match_ = Some(re),
								Err(e) => {
									err = e.to_string();
									return 3
								}
							}
						}
						"-标题" => o.other_.push(tag.to_string()),
						"-总装" => o.src2_ = argv[0].to_string(),
						"-见外" => o.out_src_ = argv[0].to_string(),
						"-仅此" => o.no_child_ = true,
						"-背后" => o.bg_ = true,
						_ => o.path_.push(tag.to_string())
					}
					0
				}, |_| 0) {
					i@250 | i@2..=3 => return i,
					_ => {}
				}
				fn add__(path:&str, o:&O_, ret3:&mut Vec<String>) -> bool {
					match fs::read_dir(path) {
						Ok(paths) => {
							for path in paths {
								let path2 = path.unwrap().path();
								let path3 = path2.display().to_string();
								if path2.is_dir() {
									if !o.no_child_ {
										if !add__(&path3, o, ret3) {return false}
									}
								} else {
									ret3.push(path3)
								}
							}
							true
						}
						Err(_e) => {/* *err = e.to_string(); false*/true}
					}
				}
				fn add2__(path3:&String, o:&O_, q:zs_::qv_::T_, w:zs_::world_::T_, wm:&mut zs_::WorldMut_,
						ret:&mut zs_::result_::List_, err:&mut String) -> Result<(), i32> {
					let mut main_ret = zs_::result_::List_::new();
					let mut eval__ = |src:&str, path3:&String, other,
							ret:&mut zs_::result_::List_, ret2:&zs_::result_::List_| -> Result<(), i32> {
						if src.is_empty() {return Ok(())}
						let mut q = zs_::Qv_::new2(Some(q.clone()));
						{
							let args = &mut q.args_;
							args.add__(path3);
							if other {
								for i in &o.other_ {
									as_ref__!(w).dunhao__(args);
									match i.as_str() {
										"-标题" => {
											let mut i2 = path3.len() - 1;
											let mut start = 0;
											let mut end = i2;
											let mut i3 = 0;
											loop {
												if let Some(c) = path3.get(i2..=i2) {
													match c {
														"/" => {
															match &path3[i2 + 1..=end] {
																"index" => end = i2 - 1,
																s if s.starts_with("index.") => end = i2 - 1,
																_ => {
																	let i4 = i2 + i3;
																	if end > i4 && end - i4 > 3 {
																		start = i2 + 1;
																		break
																	}
																}
															}
															i3 += 1;
														}
														"." =>
															if i3 == 0 {
																i3 += 1;
																end = i2 - 1;
															},
														_ => {}
													}
												}
												if i2 == 0 {break}
												i2 -= 1;
											}
											args.add__(&path3[start..=end].replace("/", "-"));
										}
										"" => {}
										_ => {}
									}
								}
							} else {
								as_ref__!(w).dunhao__(args);
								for i in ret2.iter() {
									args.add4__(i.clone())
								}
							}
						}
						q.args2_ = q.args_.to_vec__();
						if o.bg_ {
							ret.clear();
						}
						if let Err((i, s, s2)) = t_::eval__(src, zs_::qv_::t__(q), w.clone(), wm, ret) {
							match i {
								zs_::jump_::BREAK_ |
								zs_::jump_::CONTINUE_ => {if s.is_empty() {return Err(i)}}
								_ => {}
							}
							t_::err__(i, s, s2, err);
							return Err(i)
						}
						Ok(())
					};
					if let Some(re) = &o.match_ {
						if !re.is_match(&path3) {
							return eval__(&o.out_src_, &path3, false, ret, &main_ret)
						}
					}
					eval__(&o.src_, &path3, true, &mut main_ret, &ret)?;
					for (ext, src) in &o.path2_ {
						let mut f__ = |ok:&mut bool, path4:&mut PathBuf| -> Result<(), i32> {
							if path4.set_extension(&ext) {
								let path5 = path4.as_path();
								if path5.exists() {
									*ok = true;
									eval__(&src, &path5.display().to_string(), false, ret, &main_ret)?;
								}
							}
							Ok(())
						};
						let mut path4 = PathBuf::from(path3);
						let mut ok = false;
						f__(&mut ok, &mut path4)?; if ok {continue}
						path4.set_file_name("index.ext");
						f__(&mut ok, &mut path4)?; if ok {continue}
					}
					eval__(&o.src2_, &path3, false, ret, &main_ret)
				}
				if o.bg_ {
					thread::spawn(move || {
						let mut err = String::new();
						let mut ret = zs_::result_::List_::new();
						for i in &o.path_ {
							let mut ret3 = vec![];
							if !add__(i, &o, &mut ret3) {continue}
							ret3.sort();
							for i in ret3.iter() {
								let q = zs_::qv_::t__(zs_::Qv_::new2(Some(t_::MAIN_QV_.clone())));
								let w = t_::ZSW_.clone();
								let wm = &mut t_::ZSWM_.lock().unwrap();
								if let Err(i) = add2__(i, &o, q, w, wm, &mut ret, &mut err) {
									if i != zs_::jump_::CONTINUE_ {
										break
									}
								}
							}
						}
					});
				} else {
					let mut ret3 = vec![];
					for i in &o.path_ {
						if !add__(i, &o, &mut ret3) {return 1}
					}
					ret3.sort();
					for i in &ret3 {
						if let Err(i) = add2__(i, &o, q.clone(), w.clone(), wm, ret, &mut err) {
							if i != zs_::jump_::CONTINUE_ {
								return 1
							}
						}
					}
				}
			}
			"改变目录" => {
				let p = Path::new(&argv[0]);
				if env::set_current_dir(&p).is_ok() {
					t_::add__(ret, gd, w.clone(), "1")
				}
			}
			_ => {}
		}
		0
	}, |s| {
		args2.push(s.to_string());
		0
	}), args, &err)
}

pub fn get__(name:&str) -> Option<Vec<u8>> {
	match File::open(name) {
		Ok(mut f) => {
			let mut buf = Vec::new();
			match f.read_to_end(&mut buf) {
				Ok(_) => return Some(buf),
				Err(_) => None
			}
		}
		Err(_) => None
	}
}