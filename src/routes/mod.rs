pub mod config;
pub mod daemon;
pub mod info;
pub mod traffic;

pub mod index {
    use rocket::get;
    #[get("/")]
    pub fn index() -> String {
        String::from("Hello, World!")
    }
}
