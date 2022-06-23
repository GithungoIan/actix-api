## Building and api with actix web and securing it with Auth0


### Prerequisites

1. Ensure you have rust installed on your system if not head over to https://rust-lang.org to find instructions on how to setup rust.

2. Create a .env file
```bash
    touch .env
```
Add the following to the .env file

        DATABASE_URL=# this is the url to your postgres database
        AUTHORITY=#your auth0 application domain

3. Build and Run the project
```
    cargo build 
    cargo run
```


