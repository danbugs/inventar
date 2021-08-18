# Hello, there 👋

Welcome to Inventar!

Inventar is an app designed to help you keep track of your things (i.e., inventory them) be it while you are moving or, really, just in general — for some, it can even aid sustain a minimalistic lifestyle!

Inventar was developed with Svelte for the front-end and with Rust (Rocket 🚀 + Diesel ⛽) for the back-end.

All the data stored in the app is being hosted on the free-tier of a cloud-based PostgreSQL database provider (see [ElephantSQL](https://www.elephantsql.com/)), which supports a maximum of 5 concurrent connections and 20MB of data maximum.

## How I Setup the DB

1) Install `diesel` cli w/ postgres specifications: `cargo install diesel_cli --no-default-features --features postgres`
2) Create a `.env` file specifying a `DATABASE_URL` as per `.env.example` file.

> Note: Your `DATABASE_URL` should look somewhat like: `postgres://<username>:<password>@<host>/<database_name>`

> Check out [Diesel's Getting Started Guide](https://diesel.rs/guides/getting-started) for more info on setup and next steps (i.e., migrations and whatnot).

## How I Deployed on Heroku

1) Install the [Heroku CLI](https://devcenter.heroku.com/articles/heroku-cli).
2) In your command-line, log into Heroku (`heroku login`).
3) Now, on Heroku's dashboard, create a new app.
4) In your existing Rust project, run `heroku buildpacks:set emk/rust`
5) Check out my `Procfile` and `rust-toolchain` files — that's also needed 👌.
6) To finish off, if you want to avoid having to push to the Heroku remote all the time, go to the your app's deployment settings and connect to your GitHub repo — we're done! Now, whenever you push to `main/master`, you'll be deploying to Heroku!

## How to run this project locally

- For the front-end, make sure you're in the sub-root inventar folder and run: `npm run dev`.
- For the back-end, make sure you're in the root inventar folder and run: `cargo run` or `cargo watch -x run` (i.e., if you use cargo watch).




