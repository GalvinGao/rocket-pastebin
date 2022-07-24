extern crate diesel;
extern crate dotenv;
extern crate rocket;

mod test {
    use rocket::{
        http::{ContentType, Status},
        local::blocking::Client,
    };

    #[test]
    fn index() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");

        let response = client.get(uri!(crate::index)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }

    #[test]
    fn retrieve() {
        let client = Client::tracked(crate::rocket()).expect("valid rocket instance");

        let response = client
            .get(uri!(crate::retrieve("hello".to_string())))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Hello, World!".into()));
    }
}
