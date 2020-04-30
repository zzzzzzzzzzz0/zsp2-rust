mod req_;
mod file_;
mod clpars4_;
mod other_;
mod t_;

use zhscript2::{u_ as zs_, u2_::clpars_, as_mut_ref__, as_ref__};
use actix_web::{/*middleware,*/ web, App, HttpRequest, HttpServer, HttpResponse, Responder, http::header};
use actix_files::NamedFile;
use std::{env};

fn i__(gd:&zs_::code_::Opt_, q:zs_::qv_::T_, w:zs_::world_::T_, wm:&mut zs_::WorldMut_, ret:&mut zs_::result_::List_) -> zs_::Result2_ {
	let mut args = as_ref__!(q).args2_.clone();
	let mut args2 = vec![];
	fn b__(args:&mut Vec<String>, args2:&mut Vec<String>) -> bool {
		if args2.is_empty() {return true}
		args.clear();
		while !args2.is_empty() {
			args.push(args2.remove(0));
		}
		false
	};
	req_::i__(&args, &mut args2, gd, q.clone(), w.clone(), ret)?;
	if b__(&mut args, &mut args2) {return zs_::ok__()}
	file_::i__(&args, &mut args2, gd, q.clone(), w.clone(), wm, ret)?;
	if b__(&mut args, &mut args2) {return zs_::ok__()}
	clpars4_::i__(&args, &mut args2, gd, q.clone(), w.clone(), wm, ret)?;
	if b__(&mut args, &mut args2) {return zs_::ok__()}
	other_::i__(&args, &mut args2, gd, q, w, wm, ret)?;
	if b__(&mut args, &mut args2) {return zs_::ok__()}
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
			body.push_str("<hr>");
		}
		body.push_str("未实现");
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
		let wm = || t_::ZSWM_.lock().unwrap();
		let mut q = zs_::Qv_::new2(Some(t_::MAIN_QV_.clone()));
		let src = if path.ends_with('/') {
			[&path[1..], "index.zsp"].concat()
		} else {path[1..].to_string()};
		let mut src2 = String::new();
		let ret = zs_::eval_::src__(&src, &mut src2, &mut q, &as_ref__!(t_::ZSW_.clone()), &mut wm().dbg_);
		match ret {
			Ok(()) => {
				let mut my = zs_::def_::Item_::new("我的", zs_::def_::Val_::F(i__), core::usize::MAX, None);
				my.objs_add__::<HttpRequest>(&req);
				q.defs_.add__(my);

				q.src_ = src;
				let mut ret2 = zs_::result_::List_::new();
				let ret = zs_::eval_::hello2__(&src2, |it| {it.yuanyang_ = 1},
					Default::default(), zs_::qv_::t__(q), t_::ZSW_.clone(), &mut wm(), &mut ret2);
				match ret {
					Ok(()) => {
						let v = ret2.to_vec__();
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

	{
		let kws = &mut as_mut_ref__!(t_::ZSW_).kws_;
		kws.add__("<%", zs_::keyword_::Id_::EndYuanyang);
		kws.add2__("%>", vec![zs_::keyword_::Id_::Jvhao, zs_::keyword_::Id_::BeginYuanyang]);
	}
	{
		let main_q = || as_mut_ref__!(t_::MAIN_QV_);
		main_q().name_.push("主".to_string());
		main_q().val__("绑定地址", "127.0.0.1:8084");
		main_q().defs_.val2__("我的", zs_::def_::Val_::F(i__), core::usize::MAX, None, None).unwrap();

		let w = as_ref__!(t_::ZSW_).clone();
		let wm = || t_::ZSWM_.lock().unwrap();
		let mut conf_q = zs_::Qv_::new2(Some(t_::MAIN_QV_.clone()));
		{
			let ret = w.clone().hello2__(&mut env::args(), true, false, false, &mut conf_q, &mut wm());
			match ret {
				Ok(()) => {
					let ret = w.ret__(ret);
					if ret != 0 {
						t_::exit__(ret);
					}
				}
				Err((i, s, s2)) => t_::errexit__(i, s, s2),
			}
		}
		let mut args2 = vec![];
		let cp = clpars_::List_::new3(vec![
			clpars_::Item_::new3("-zsp-addr", 1, "绑定地址"),
			clpars_::Item_::new3("-zsp-conf", 1, "由配置文件"),
			clpars_::Item_::new("-zsp-help"),
			clpars_::Item_::new0(),
		], concat!("ZhServerPage2 v", env!("CARGO_PKG_VERSION"), "\n"));
		cp.for__(&mut conf_q.args_.to_vec__().into_iter(), |tag, argv, _item, _i3| {
			match tag {
				"-zsp-addr" => main_q().val__("绑定地址", &argv[0]),
				"-zsp-conf" => conf_q.src_ = argv[0].to_string(),
				"-zsp-help" => {
					print!("{}", cp.help__());
					if w.clone().hello2__(&mut vec!["-zhscript-help".to_string()].into_iter(),
						false, false, false, &mut conf_q, &mut wm()).is_err() {}
					t_::exit__(251);
				}
				_ => args2.push(tag.to_string())
			}
			0
		}, |_| 0);
		if !conf_q.src_.is_empty() {
			let mut src = String::new();
			let ret = zs_::eval_::src__(&conf_q.clone().src_, &mut src, &mut conf_q, &w, &mut wm().dbg_);
			match ret {
				Ok(()) => {
					{
						let args = &mut conf_q.args_;
						args.clear();
						for i in args2 {
							if !args.is_empty() {
								w.dunhao__(args)
							}
							args.add__(i)
						}
					}
					let mut ret2 = zs_::result_::List_::new();
					if let Err((i, s, s2)) = t_::eval__(&src, zs_::qv_::t__(conf_q),
							t_::ZSW_.clone(), &mut wm(), &mut ret2) {
						t_::errexit__(i, s, s2);
					}
				}
				Err((i, s, s2)) => {
					t_::errexit__(i, s, s2);
				}
			}
		} else {
			main_q().args_ = conf_q.args_;
		}
	}
	HttpServer::new(|| {
		App::new()
			//.wrap(middleware::Logger::default())
			.service(web::resource("/zsp-ver").to(|| async { env!("CARGO_PKG_VERSION") }))
			.default_service(web::to(index__))
	})
	.bind(t_::main_var__("绑定地址"))?
	.run()
	.await
}
