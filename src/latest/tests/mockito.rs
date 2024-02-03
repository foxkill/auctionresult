#[cfg(test)]
use mockito;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const CATS_URL: &str = "https://cat-fact.herokuapp.com";

#[cfg(not(test))]
const TODO_URL: &str = "https://jsonplaceholder.typicode.com";

fn get_cats_url() -> String {
    #[cfg(not(test))]
    let url = format!("{}/facts/random", CATS_URL);
    #[cfg(test)]
    let url = format!("{}/facts/random", mockito::server_url());
    url
}

fn get_todo_url() -> String {
    #[cfg(not(test))]
    let url = format!("{}/todos/1", TODO_URL);
    #[cfg(test)]
    let url = format!("{}/todos/1", mockito::server_url());
    url
}