use zhscript2::{u_ as zs_, u2_::clpars_, as_ref__, as_mut_ref__};
use super::{t_};
use std::{env, path::{Path, PathBuf}, fs, fs::File, io::Read, thread};
use regex::Regex;

pub fn i__(args:&Vec<String>, args2:&mut Vec<String>, env:&zs_::code_::Env_) -> zs_::Result2_ {
	let mut err = String::new();
	let cp = clpars_::List_::new2(vec![
		clpars_::Item_::new2t("链接有", clpars_::Typ_::Starts),
		clpars_::Item_::new2t("文件有", clpars_::Typ_::Starts),
		clpars_::Item_::new2c("文件", 1),
		clpars_::Item_::new2c("主名", 1),
		clpars_::Item_::new2c("目录", 1),
		clpars_::Item_::new2cz("遍历目录", 2),
		clpars_::Item_::new2c("改变目录", 1),
	]);
	t_::ierr__(if let Err((i, _)) = cp.for3__(&mut args.clone().into_iter(), |tag, argv, _, item, _, _| {
		match item.tag_.as_str() {
			"链接有" => {
				if Path::new(&tag[item.tag_.len()..]).is_symlink() {
					t_::add__(env, "1")
				}
				return 0
			}
			"文件有" => {
				if Path::new(&tag[item.tag_.len()..]).exists() {
					t_::add__(env, "1")
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
							Ok(_) => t_::add__(env, buf),
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
							t_::add__(env, &s[i..i2]);
							return 0
						}
					}
					t_::add__(env, &s[i..])
				}
			}
			"目录" => {
				let p = Path::new(&argv[0]);
				if let Some(p) = p.parent() {
					t_::add__(env, p.display())
				}
			}
			"遍历目录" => {
				#[derive(Default)]
				struct O {
					path_:Vec<String>,
					src_:String,
					match_:Option<Regex>,
					match_code_:Option<String>,
					other_:Vec<String>,
					title_len_:usize,
					title_skip_:Vec<String>,
					title_skip2_:Vec<String>,
					path2_:Vec<(String, String)>,
					src2_:String,
					out_src_:String,
					eoe_src_:String,
					no_child_:bool,
					bg_:bool,
				}
				let mut o:O = Default::default();
				o.title_len_ = 15;
				o.title_skip_.push("index".to_string());
				let cp = clpars_::List_::new2(vec![
					clpars_::Item_::new2c("-路径", 1),
					clpars_::Item_::new2c("-配", 1),
					clpars_::Item_::new2c("-配码", 1),
					clpars_::Item_::new("-标题"),
					clpars_::Item_::new2c("-标题适长", 1),
					clpars_::Item_::new2c("-标题略头", 1),
					clpars_::Item_::new2c("-标题略尾", 1),
					clpars_::Item_::new3ct("-同名", 1, clpars_::Typ_::Starts),
					clpars_::Item_::new2c("-总装", 1),
					clpars_::Item_::new2c("-见外", 1),
					clpars_::Item_::new2c("-尾声", 1),
					clpars_::Item_::new("-仅此"),
					clpars_::Item_::new("-背后"),
					clpars_::Item_::new0(),
				]);
				if cfg!(debug_assertions) {
					println!("{:?}", argv)
				}
				if let Err((i, _)) = cp.for3__(&mut argv.clone().into_iter(), |tag, argv, _, item, _, _| {
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
						"-配码" => o.match_code_ = Some(argv[0].to_string()),
						"-标题" => o.other_.push(tag.to_string()),
						"-标题适长" => if let Some(i) = zs_::t_::s2n__::<usize>(&argv[0]) {
								o.title_len_ = i;
							} else {
								err.push_str(tag);
								err.push_str(" 非数字");
								return 3
							}
						"-标题略头" | "-标题略尾" => {
							let v: Vec<_> = argv[0].split_whitespace().collect();
							let mut v = v.iter().map(|i| i.to_string()).collect();
							if tag == "-标题略尾" {
								o.title_skip2_.append(&mut v);
							} else {
								o.title_skip_.append(&mut v);
							}
						}
						"-总装" => o.src2_ = argv[0].to_string(),
						"-见外" => o.out_src_ = argv[0].to_string(),
						"-尾声" => o.eoe_src_ = argv[0].to_string(),
						"-仅此" => o.no_child_ = true,
						"-背后" => o.bg_ = true,
						_ => o.path_.push(tag.to_string())
					}
					0
				}, |_| 0) {
					match i {
						i@clpars_::ARG_NE_ | i@2..=3 => return i,
						_ => {}
					} 
				}
				use std::cmp::{Ordering};
				#[derive(Eq)]
				struct Path3 {
					s_:String,
					i_:usize,
				}
				impl Ord for Path3 {
					fn cmp(&self, other: &Self) -> Ordering {
						//self.s_.cmp(&other.s_)
						//println!("{:?} {:?}", self.s_,other.s_);
						super::cmp_::bb__(self.s_.as_bytes(), other.s_.as_bytes())
					}
				}
				impl PartialOrd for Path3 {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
				impl PartialEq for Path3 {fn eq(&self, other: &Self) -> bool {self.s_.eq(&other.s_)}}
				fn add__(path:&str, path4:&str, o:&O, ret3:&mut Vec<Path3>) -> bool {
					match fs::read_dir(path) {
						Ok(paths) => {
							for path in paths {
								let path2 = path.unwrap().path();
								let path3 = path2.display().to_string();
								if path2.is_dir() {
									if !o.no_child_ {
										if !add__(&path3, path4, o, ret3) {return false}
									}
								} else {
									let i_ = if path3.starts_with(path4) {path4.len()} else {0}
											+if path4.ends_with("/") {0} else {1};
									ret3.push(Path3 {s_:path3, i_})
								}
							}
							true
						}
						Err(_e) => {/* *err = e.to_string(); false*/true}
					}
				}
				fn eval2__(src:&str, path3:&Path3, other:impl Fn(&String, &mut zs_::result_::List_),
						ret :zs_::T_<zs_::result_::List_>, env:&zs_::code_::Env_, err:&zs_::code_::MS_) -> Result<(), i32> {
					if src.is_empty() {return Ok(())}
					let q = zs_::Qv_::new2(Some(env.q.clone()));
					{
						let args = &mut as_mut_ref__!(q.args_);
						let path = &path3.s_;
						args.add__(path);
						other(path, args);
					}
					//if o.bg_ {as_mut_ref__!(ret).clear();}
					if let Err((i, i2, s, s2)) = t_::eval__(src,
					&zs_::code_::Env_::new9(zs_::t__(q), ret, env)) {
						match i {
							zs_::jump_::BREAK_ |
							zs_::jump_::CONTINUE_ => {if s.is_empty() {return Err(i)}}
							_ => {}
						}
						t_::err__(i, i2, s, s2, &mut err.lock().unwrap());
						return Err(i)
					}
					Ok(())
				}
				fn add2__(path3:&Path3, o:&O, env:&zs_::code_::Env_, err:&zs_::code_::MS_) -> Result<(), i32> {
					let main_ret = zs_::t__(zs_::result_::List_::new());
					let eval__ = |src:&str, path3:&Path3, other,
							ret :zs_::T_<zs_::result_::List_>,
							ret2:zs_::T_<zs_::result_::List_>| -> Result<(), i32> {
						eval2__(src, path3, |path, args| {
							if other {
								for i in &o.other_ {
									as_ref__!(env.w).dunhao__(args);
									match i.as_str() {
										"-标题" => {
											let ss:Vec<&str> = path[path3.i_..].split("/").collect();
											let end = ss.len() - 1;
											let mut s = String::new();
											let mut idx = ss.len();
											let mut skip = false;
											'l1: loop {
												if idx == 0 {break}
												idx -= 1;
												let i = ss[idx];
												for i2 in &o.title_skip_ {
													if i.starts_with(&*i2) {
														skip = true;
														continue 'l1;
													}
												}
												for i2 in &o.title_skip2_ {
													if i.ends_with(&*i2) {
														skip = true;
														continue 'l1;
													}
												}
												let mut start = 0;
												for idx2 in 0..idx {
													let i2 = ss[idx2];
													if i.starts_with(i2) && start < i2.len() {
														start = i2.len()
													}
												}
												let mut end2 = i.len();
												if start >= end2 {continue}
												if !skip && !s.is_empty() {s.insert(0, '-')}
												if idx == end {
													if let Some(idx) = i.rfind('.') {
														end2 = idx;
													}
												}
												skip = false;
												s.insert_str(0, &i[start..end2]);
												if s.len() >= o.title_len_ {
													if start > 0 {s.insert_str(0, &i[0..start]);}
													break
												}
											}
											args.add__(s);
										}
										"" => {}
										_ => {}
									}
								}
							} else {
								as_ref__!(env.w).dunhao__(args);
								for i in as_ref__!(ret2).iter() {
									args.add4__(i.clone())
								}
							}
						}, ret, env, err)
					};
					if let Some(re) = &o.match_ {
						if !re.is_match(&path3.s_) {
							return eval__(&o.out_src_, path3, false, env.ret.clone(), main_ret.clone())
						}
					}
					if let Some(code) = &o.match_code_ {
						let ret = zs_::t__(zs_::result_::List_::new());
						eval2__(code, path3, |_, _| {}, ret.clone(), env, err)?;
						let v = as_ref__!(ret).to_vec__();
						if v.is_empty() || !zs_::t_::true__(&v[0]) {
							return eval2__(&o.out_src_, path3, |_, args| {
								for s in v.iter().skip(1) {
									as_ref__!(env.w).dunhao__(args);
									args.add__(s);
								}
							}, main_ret.clone(), env, err)
						}
					}
					eval__(&o.src_, path3, true, main_ret.clone(), env.ret.clone())?;
					for (ext, src) in &o.path2_ {
						let f__ = |ok:&mut bool, path4:&mut PathBuf| -> Result<(), i32> {
							if path4.set_extension(&ext) {
								let path5 = path4.as_path();
								if path5.exists() {
									*ok = true;
									eval__(&src, &Path3 {s_:path5.display().to_string(), i_:0}, false,
										env.ret.clone(), main_ret.clone())?;
								}
							}
							Ok(())
						};
						let mut path4 = PathBuf::from(&path3.s_);
						let mut ok = false;
						f__(&mut ok, &mut path4)?; if ok {continue}
						while !ok {
							if let Some(s) = path4.clone().file_stem() {
								path4.set_file_name(s);
								if path4.extension().is_none() {break}
								f__(&mut ok, &mut path4)?;
							} else {break}
						}
						if ok {continue}
						/*path4.set_file_name("index.ext");
						f__(&mut ok, &mut path4)?; if ok {continue}*/
					}
					eval__(&o.src2_, path3, false, env.ret.clone(), main_ret)
				}
				let eoe_eval = |o:&O, env:&zs_::code_::Env_| {
					if !o.eoe_src_.is_empty() {
						/*let q = zs_::Qv_::new2(Some(env.q.clone()));
						let env = &zs_::code_::Env_::new2(zs_::t__(q), env);*/
						if let Err((i, i2, s, s2)) = t_::eval__(&o.eoe_src_, env) {
							zs_::result2_::eprtn__(i, i2, &s, &s2);
							return 1
						}
					}
					0
				};
				if o.bg_ {
					let env2 = zs_::code_::Env_::new10(env);
					thread::spawn(move || {
						let mut err = String::new();
						let err = zs_::code_::MS_::new(&mut err);
						for i in &o.path_ {
							if cfg!(debug_assertions) {
								println!("{:?}", i)
							}
							let mut ret3 = vec![];
							if !add__(i, i, &o, &mut ret3) {continue}
							ret3.sort();
							for i in ret3.iter() {
								if let Err(i) = add2__(i, &o, &env2, &err) {
									if i == zs_::jump_::CONTINUE_ {
										continue
									}
									eprintln!("{}", err.lock().unwrap());
									return
								}
							}
						}
						eoe_eval(&o, &env2);
					});
				} else {
					let mut ret3 = vec![];
					for i in &o.path_ {
						if !add__(i, i, &o, &mut ret3) {return 1}
					}
					ret3.sort();
					for i in &ret3 {
						if let Err(i) = add2__(i, &o, env, &zs_::code_::MS_::new(&mut err)) {
							if i != zs_::jump_::CONTINUE_ {
								return 1
							}
						}
					}
					match eoe_eval(&o, env) {
						0 => {}
						i@_ => return i
					}
				}
			}
			"改变目录" => {
				let p = Path::new(&argv[0]);
				if env::set_current_dir(&p).is_ok() {
					t_::add__(env, "1")
				}
			}
			_ => {}
		}
		0
	}, |s| {
		args2.push(s.to_string());
		0
	}) {i} else {0}, args, &err)
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