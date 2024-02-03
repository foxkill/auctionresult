#[cfg(test)]
use mockito;

// ...

// The host to be used for non-test (production) compilation
#[cfg(not(test))]
let host = "http://example.com";

// The host to be used in test com ilation
#[cfg(test)]
let host = &mockito::server_url();

let url = format!("{}/endpoint", host);
let text = reqwest::get(&url)?.text()?;