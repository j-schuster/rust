use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    // this is a closure. if it took in argument there would be something between the vertical lines
    let server = HttpServer::new(|| {
        // since there's no semicolon after the line below, App is the return value
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("App running on localhost:3000...");

    server 
        .bind("127.0.0.1:3000").expect("Error binding server to address")
        .run().expect("error running server");
}

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
    n
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing GCD with zero is boring");
    }

    let response = 
        format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>", 
            form.n, form.m, gcd(form.n, form.m)
        );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}