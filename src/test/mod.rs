use super::*;
use httpmock::prelude::*;
use std::fs;

#[test]
fn prepare_service_url_test() {
    let expected_url = "https://api.weatherapi.com/v1/current.json?key=ABCDE&q=London&aqi=no";
    let created_url = prepare_service_url("London", "ABCDE".to_string());
    assert_eq!(*expected_url, created_url);
}

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[test]
fn get_city_info_for_valid_city() {
    let server = MockServer::start();

    let filename = "src/test/city_payload.json";
    let payload = fs::read_to_string(filename).expect("Couldn't find payload file.");

    let path = "/v1/current.json?key=ABCDE&q=London&aqi=no";

    let get_city_info_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/current.json")
            .query_param("key", "ABCDE")
            .query_param("q", "London")
            .query_param("aqi", "no");
        then.status(200)
            .header("content-type", "application/json")
            .body(payload);
    });

    let response = aw!(get_city_info(server.url(path))).unwrap();

    get_city_info_mock.assert();

    assert_eq!(response.status(), 200);
}
