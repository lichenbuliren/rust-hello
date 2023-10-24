use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/gcd")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n" />
            <input type="text" name="m" />
            <button type="submit">Compute GCD</button>
        </form>
    "#)
}

// 计算两个整数的最大公约数
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // 这里的 n 作为结果返回，不需要以分号结尾
    n
}

#[derive(Deserialize)]
struct GcdParameters {
    n:u64,
    m:u64,
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Computing the GCD with zero is boing.");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok().content_type("text/html").body(response)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(post_gcd)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
