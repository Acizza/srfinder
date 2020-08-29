# srfinder

This is a web-based program to find direct flight routes for flight simulators that match specified filters. It can be useful when you are not sure where you want to fly from and/or to.

Homepage:

![Screenshot showing homepage](res/homepage.png?raw=true)

Showing a found route on a map:

![Screenshot showing a found route](res/route.png?raw=true)

Showing the runways of an airport and its frequency / runway information:

![Screenshot showing airport runways](res/airport_info.png?raw=true)

The frontend is made with [Svelte](https://svelte.dev/) and the backend with [Rust](https://www.rust-lang.org/) and [Rocket](https://rocket.rs/).

# Building

**Note that these instructions are intended for Linux / OSX users. Windows users may need to adjust the commands a bit.**

This project requires the following dependencies:

* A recent stable version of Rust
* A recent version of NodeJS
* pkg-config

The following sections describe how to build each part of the project. Either step can be done first.

## Building the backend

Open a terminal in the project directory and run `cargo build --release`. Once complete, you will be able to run the program from within the `target/release/` folder.

## Building the frontend

From within the project directory, navigate to the `frontend` folder and run the following commands:

1. `npm install`
2. `npm run build`

# Usage

First, launch the web server executable named `srfinder` located in the `target/release/` folder.

You can then open `localhost:8000` in your web browser. Please note that the project has only been tested in Firefox.

# Data Source

All airport data comes from [OurAirports](https://ourairports.com/).