use actix_web::{web::get, App, HttpResponse, HttpServer};

fn main() {
    let server = HttpServer::new(|| App::new().route("/", get().to(index())));
    println!("Serving on http://localhost:8080...");
    server
        .bind("127.0.0.1:8080")
        .expect("fuck,something went worng")
        .run()
        .expect("fuck ,could not run a server");
}
fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD </button>
        </form>
        "#,
    )
}
