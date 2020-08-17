mod req_;
mod file_;
mod clpars4_;
mod prgm_;
mod other_;
mod t_;

use zhscript2::{u_ as zs_, u2_::clpars_, as_mut_ref__, as_ref__};
use actix_web::{/*middleware,*/ web, App, HttpRequest, HttpServer, HttpResponse, Responder, http::header};
use actix_files::{NamedFile, Files};
use std::{env};

fn i__(env:&zs_::code_::Env_) -> zs_::Result2_ {
	let mut args;
	{
		let q = as_ref__!(env.q);
		let a = as_ref__!(q.args_);
		if as_ref__!(env.w).dbg_.arg_ {
			as_ref__!(env.w).dbg_.arg__(&a);
		}
		args = a.to_vec__();
	}
	let mut args2 = vec![];
	fn b__(args:&mut Vec<String>, args2:&mut Vec<String>) -> bool {
		if args2.is_empty() {return true}
		args.clear();
		while !args2.is_empty() {
			args.push(args2.remove(0));
		}
		false
	};
	req_	::i__(&args, &mut args2, env)?; if b__(&mut args, &mut args2) {return zs_::ok__()}
	file_	::i__(&args, &mut args2, env)?; if b__(&mut args, &mut args2) {return zs_::ok__()}
	clpars4_::i__(&args, &mut args2, env)?; if b__(&mut args, &mut args2) {return zs_::ok__()}
	prgm_	::i__(&args, &mut args2, env)?; if b__(&mut args, &mut args2) {return zs_::ok__()}
	other_	::i__(&args, &mut args2, env)?; if b__(&mut args, &mut args2) {return zs_::ok__()}
	t_::ierr1__(&args, "!")
}

async fn index__(req: HttpRequest) -> impl Responder {
	let mut body = String::new();
	let mut content_type = String::from("text/html; charset=utf-8");
	let bad__ = |body| {
		HttpResponse::BadRequest().content_type(&content_type).body(body)
	};
	let bad2__ = |v:&Vec<String>| {
		let mut body = String::new();
		for i in v {
			body.push_str(i);
			body.push_str("<hr>\n");
		}
		body.push_str("未实现\n");
		bad__(body)
	};
	let namedfile__ = |path| {
		match NamedFile::open(path) {
			Ok(nf) => nf.into_response(&req).unwrap(),
			Err(e) => bad__([path, " ", &e.to_string()].concat())
		}
	};
	let path = req.uri().path();
	if path.ends_with(".zsp") || path.ends_with('/') {
		let q = zs_::t__(zs_::Qv_::new2(Some(t_::MAIN_QV_.clone())));
		let src = if path.ends_with('/') {
			[&path[1..], "index.zsp"].concat()
		} else {path[1..].to_string()};
		zs_::eval_::ok_src__(&src, q.clone(), t_::ZSW_.clone());
		let mut src2 = String::new();
		let ret = zs_::eval_::src__(&mut src2, q.clone(), t_::ZSW_.clone());
		match ret {
			Ok(()) => {
				let mut my = zs_::def_::Item_::new("我的", zs_::def_::Val_::F(i__), core::usize::MAX, None);
				my.objs_add__::<HttpRequest>(&req);
				as_mut_ref__!(q).defs_.add__(my);

				as_mut_ref__!(q).src_ = src;
				let ret2 = zs_::t__(zs_::result_::List_::new());
				let ret = zs_::eval_::hello2__(&src2, |it| {it.yuanyang_ = 1},
					&zs_::code_::Env_::new(q, t_::ZSW_.clone(), ret2.clone()));
				match ret {
					Ok(()) => {
						let v = as_ref__!(ret2).to_vec__();
						match v.len() {
							1 => body = v[0].to_string(),
							2 => match v[0].as_str() {
									"file" => return namedfile__(&v[1]),
									"url" => return HttpResponse::Found()
											.header(header::LOCATION, v[1].as_str())
											.finish(),
									_ => {
										content_type = v[0].to_string();
										body = v[1].to_string();
									}
								},
							3 => {
								match v[0].as_str() {
									"file" =>
										if let Some(buf) = file_::get__(&v[2]) {
											return HttpResponse::Ok()
												.content_type(v[1].to_string())
												.body(buf)
										},
									"stream" => {
										/*let body = once(ok::<_, Error>(Bytes::from_static(STR.as_ref())));
										return Response::Ok()
											.header(header::TRANSFER_ENCODING, "chunked")
											.streaming(body)*/
									}
									_ => {}
								}
								return bad2__(&v)
							}
							0 => {}
							_ => return bad2__(&v)
						}
						HttpResponse::Ok().content_type(content_type).body(body)
					}
					Err((i, s, s2)) => {
						t_::err__(i, s, s2, &mut body);
						bad__(body)
					}
				}
			}
			Err((i, s, s2)) => {
				t_::err__(i, s, s2, &mut body);
				bad__(body)
			}
		}
	} else {
		namedfile__(&path[1..])
	}
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	/*std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();*/

	let w = || as_ref__!(t_::ZSW_);
	let wm = || as_mut_ref__!(t_::ZSW_);
	{
		let kws = &mut wm().kws_;
		kws.add__("<%", zs_::keyword_::Id_::EndYuanyang);
		kws.add2__("%>", vec![zs_::keyword_::Id_::Jvhao, zs_::keyword_::Id_::BeginYuanyang]);
	}
	let addr;
	let mut use_ret2 = false;
	{
		let main_q = || as_mut_ref__!(t_::MAIN_QV_);
		main_q().defs_.val2__("我的", zs_::def_::Val_::F(i__), core::usize::MAX, None, None).unwrap();

		let mut conf_q = zs_::Qv_::new2(Some(t_::MAIN_QV_.clone()));
		{
			let ret = zs_::world_::clpars__(&mut wm(), &mut env::args(),
				true, false, false, &mut conf_q);
			match ret {
				Ok(()) => {
					let ret = w().ret__(ret);
					if ret != 0 {
						t_::exit__(ret);
					}

					let top_q = &w().top_q_;
					let mut top_q = as_mut_ref__!(top_q);
					top_q.val__("外壳", &w().cfg_.shl_);
					top_q.val__("窗口", "linux");
				}
				Err((i, s, s2)) => t_::errexit__(i, s, s2),
			}
		}
		let mut args2 = vec![];
		const HELP:&str = "-zsp-help";
		{
			let cp = clpars_::List_::new3(vec![
				clpars_::Item_::new3("-zsp-addr", 1, "绑定地址"),
				clpars_::Item_::new3("-zsp-conf", 1, "由配置文件"),
				clpars_::Item_::new(HELP),
				clpars_::Item_::new0(),
			], concat!("ZhServerPage2 v", env!("CARGO_PKG_VERSION"), "\n"));
			/*let argv = vec![
				concat!("ZhServerPage2 v", env!("CARGO_PKG_VERSION"), "\n"),
				"-zsp-addr", "绑定地址", "1", "",
				"-zsp-conf", "由配置文件", "1", "",
				HELP, "", "0", "",
				"", "", "1", "",
			];
			*/
			let v = as_ref__!(conf_q.args_).to_vec__();
			cp.for__(&mut v.into_iter(), |tag, argv, _item, _i3| {
				match tag {
					"-zsp-addr" => main_q().val__("绑定地址", &argv[0]),
					"-zsp-conf" => conf_q.src_ = argv[0].to_string(),
					"-zsp-help" => {
						print!("{}", cp.help__());
						if zs_::world_::clpars__(&mut wm(),
							&mut vec![zs_::world_::HELP_.to_string()].into_iter(),
							false, false, false, &mut conf_q).is_err() {}
						t_::exit__(251);
					}
					_ => args2.push(tag.to_string())
				}
				0
			}, |_| 0);
		}
		let ret2 = zs_::t__(zs_::result_::List_::new());
		if !conf_q.src_.is_empty() {
			let conf_q2 = zs_::t__(conf_q.clone());
			let mut src = String::new();
			let ret;
			if w().cfg_.src_is_file_ {
				zs_::eval_::ok_src__(&conf_q.src_, conf_q2.clone(), t_::ZSW_.clone());
				ret = zs_::eval_::src__(&mut src, conf_q2.clone(), t_::ZSW_.clone());
			} else {
				src.push_str(&conf_q.src_);
				ret = zs_::ok__()
			}
			match ret {
				Ok(()) => {
					{
						let conf_q2 = as_ref__!(conf_q2);
						let args = &mut as_mut_ref__!(conf_q2.args_);
						args.clear();
						for i in args2 {
							if !args.is_empty() {
								w().dunhao__(args)
							}
							args.add__(i)
						}
					}
					if let Err((i, s, s2)) = t_::eval__(&src,
					&zs_::code_::Env_::new(conf_q2, t_::ZSW_.clone(), ret2.clone())) {
						t_::errexit__(i, s, s2);
					}
					use_ret2 = true;
				}
				Err((i, s, s2)) => {
					t_::errexit__(i, s, s2);
				}
			}
		} else {
			main_q().args_ = conf_q.args_;
		}
		addr = t_::main_var__("绑定地址");
		if addr.is_empty() {
			if use_ret2 {
				for i in as_ref__!(ret2).to_vec__() {
					println!("{}", i);
				}
			} else {
				println!("{}", HELP);
			}
			return Ok(())
		}
	}
	HttpServer::new(move || {
		let app =
		App::new()
			//.wrap(middleware::Logger::default())
			.service(web::resource("/zsp-ver").to(|| async { env!("CARGO_PKG_VERSION") }))
			;
		if use_ret2 {
			app.default_service(web::to(index__))
		} else {
			app.service(Files::new("/", ".").show_files_listing())
		}
	})
	.bind(addr)?
	.run()
	.await
}
