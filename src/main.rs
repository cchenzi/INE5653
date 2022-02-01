use ine5653::server::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("STARTING SERVER!");
    run()?.await
}
