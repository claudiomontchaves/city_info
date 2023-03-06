# city_info

A Rust console app to show weather information of a city. It makes an API call to the service [weatherapi.com](https://www.weatherapi.com/) to get the data.

### Usage

```shell
$ cargo run London
City:        London
Region:      City of London, Greater London
Country:     United Kingdom
Latitude:    51.52
Longitude:   -0.11
Local Time:  2023-03-06 13:11
Condition:   Partly cloudy
Temperature: 9 Celcius
Feels like:  6.6 Celcius
Wind:        24.1 km/h
Wind Dir:    W km/h
```

To be able to run the app you need to create an account at [weatherapi.com](https://www.weatherapi.com/) and get your **API Key**. Then edit the file **app.properties** and replace <PUT_YOUR_API_KEY_HERE> with your key.

[Install Rust in your machine.](https://www.rust-lang.org/tools/install)

To create a binary run:

```shell
$ cargo build --release
```

Then copy the file **target/release/city_info** and **app.properties** to any folder you want. From that folder run:

```shell
$ city_info <CITY_NAME>
```

**Examples:**

```shell
$ city_info Rome
City:        Rome
Region:      Lazio
Country:     Italy
Latitude:    41.9
Longitude:   12.48
Local Time:  2023-03-06 14:32
Condition:   Partly cloudy
Temperature: 15 Celcius
Feels like:  13.7 Celcius
Wind:        19.1 km/h
Wind Dir:    SSW km/h

$ city_info 'Rio de Janeiro'
City:        Rio De Janeiro
Region:      Rio de Janeiro
Country:     Brazil
Latitude:    -22.9
Longitude:   -43.23
Local Time:  2023-03-06 10:33
Condition:   Overcast
Temperature: 26 Celcius
Feels like:  27 Celcius
Wind:        31 km/h
Wind Dir:    W km/h
```

In this project you can find Rust code examples for:

- Read application properties from a file using the [app_propeties](https://crates.io/crates/app_properties) crate.
- Make HTTP GET request to a REST API endpoint using [reqwest](https://crates.io/crates/reqwest) crate.
- Using [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) to parse the JSON response to a Rust struct.
- Using [httpmock](https://crates.io/crates/httpmock), a HTTP mocking library, to mock the weather service API and test the code.
- Using [tokio-test](https://crates.io/crates/tokio-test) to test async functions.
