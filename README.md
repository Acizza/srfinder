# srfinder

This is a web-based program to find direct routes for flight simulators that match specified filters. It can be useful when you are not sure where you want to fly from and/or to.

The frontend is made with [Svelte](https://svelte.dev/) and the backend with [Rust](https://www.rust-lang.org/) and [Rocket](https://rocket.rs/).

# Screenshots

Homepage with dark theme:

![Screenshot showing dark homepage](res/homepage_dark.png?raw=true)

Homepage with light theme:

![Screenshot showing light homepage](res/homepage_light.png?raw=true)

Showing a found route on a map:

![Screenshot showing a found route](res/route.png?raw=true)

Showing the runways of an airport and its frequency / runway information:

![Screenshot showing airport runways](res/airport_info.png?raw=true)

# Building

**Note that these instructions are intended for Linux / OSX users. Windows users may need to adjust the commands a bit.**

This project requires the following dependencies:

* A recent stable version of Rust
* A recent version of npm
* pkg-config

The following sections describe how to build each part of the project. Either step can be done first.

## Building the backend

Open a terminal in the project directory and run `cargo build --release`. Once complete, you will be able to run the program from within the `target/release/` folder.

## Building the frontend

From within the project directory, navigate to the `frontend` folder and run the following commands:

1. `npm install`
2. `npm run build`

# Usage

First, launch executable named `srfinder` in the `target/release/` folder to start the web server, and navigate to `localhost:8000` in your web browser.

Navigation within the site should be fairly straightfoward. You can hover over most filters to find out more about which each one does, and can change the site theme / which units to use by clicking the cog icon in the bottom right corner.

# Data Source

All airport data comes from [OurAirports](https://ourairports.com/).