use ine5653::{config::APP_LANGUAGE, server::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("STARTING SERVER! APP_LANG={}!", APP_LANGUAGE.as_str());
    run()?.await
}
