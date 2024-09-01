Async Cached Example in Rust
============================

This repository contains the code for an example project demonstrating asynchronous programming in Rust with the `tokio` runtime and caching using the `cached` library. This project is designed to complement the YouTube tutorial from Aarambh Dev Hub.

## Watch the Tutorial

[![Master Async Programming with Caching in Rust](https://img.youtube.com/vi/2MQdEklDFX4/hqdefault.jpg)](https://youtu.be/2MQdEklDFX4)

Click the image above to watch the detailed tutorial on YouTube!


Table of Contents
-----------------
-   [Introduction](#introduction)
-   [Project Structure](#project-structure)
-   [Setup Instructions](#setup-instructions)
-   [Running the Project](#running-the-project)
-   [Understanding the Code](#understanding-the-code)
    -   [Asynchronous Function](#asynchronous-function)
    -   [Implementing Caching](#implementing-caching)
    -   [Customizing the Cache](#customizing-the-cache)
-   [License](#license)

Introduction
------------

Asynchronous programming is crucial for building performant applications, especially when dealing with I/O-bound tasks. In this project, we demonstrate how to use Rust's async capabilities with the `tokio` runtime and boost performance with caching using the `cached` library.

Project Structure
-----------------

Here's a brief overview of the project structure:

plaintext

Copy code

```
async_cached_example/
├── Cargo.toml
└── src/
    └── main.rs
```

-   `Cargo.toml`: Contains the dependencies (`tokio` and `cached`).
-   `src/main.rs`: The main file where the asynchronous function and caching logic are implemented.

Setup Instructions
------------------

To set up this project on your local machine, follow these steps:

1.  **Clone the repository**:

    ```
    git clone https://github.com/AarambhDevHub/async_cached_example.git
    cd async-cached-example
    ```

2.  **Install the dependencies**:
    ```
    cargo build
    ```

3.  **Run the project**:
    ```
    cargo run
    ```

Running the Project
-------------------

To see the asynchronous and caching code in action, simply run the project:

```
cargo run
```

You should see output that demonstrates how the `fetch_data` function works asynchronously and how the cached version speeds up repeated calls.

Understanding the Code
----------------------

### Asynchronous Function

In `main.rs`, the `fetch_data` function is an async function that simulates fetching data with a delay:

```
async fn fetch_data(id: u32) -> String {
    println!("Fetching data for ID: {}", id);
    sleep(Duration::from_secs(2)).await; // Simulate network delay
    format!("Data for ID: {}", id)
}
```

### Implementing Caching

The `cached` library is used to cache the results of `fetch_data`, preventing repeated calculations for the same input:

```
#[cached]
async fn fetch_data_cached(id: u32) -> String {
    println!("Fetching data for ID: {}", id);
    sleep(Duration::from_secs(2)).await;
    format!("Data for ID: {}", id)
}
```

### Customizing the Cache

We can further customize the cache with a time-to-live (TTL) setting:

```
#[cached(time = 10)]
async fn fetch_data_cached_ttl(id: u32) -> String {
    println!("Fetching data for ID: {}", id);
    sleep(Duration::from_secs(2)).await;
    format!("Data for ID: {}", id)
}
```

This setting ensures that cached data is invalidated after 10 seconds, forcing the function to fetch fresh data if called again after this period.

License
-------

This project is licensed under the MIT [License](#). See the LICENSE file for details.
