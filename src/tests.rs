use rocket::local::blocking::Client;

#[test]
fn ping() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/ping").dispatch();
    assert_eq!(response.into_string().unwrap(), "PONG");
}
