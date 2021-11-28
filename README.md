# beta.gouv jobs push notification

Get a push notification when a new offers pops up on  [beta.gouv.fr/recrutement](https://beta.gouv.fr/recrutement)! :vibration_mode:

---

## About

As I am looking out to join a project at [beta.gouv](https://beta.gouv.fr), I wanted to automate the job offers discovery, to avoid having to reload the same page over and over again.

To send a push notification to my mobile device, I chose [PushNotifier](https://pushnotifier.de). With that free service, it is possible to send notifications to an Android or iOS device, to Telegram, or even to a Webhook for more flexibility.

---

## Getting started

### Prerequisites

In order to run it locally, you will need to:

1. Install [Rust](https://www.rust-lang.org/tools/install) (or [Docker](https://docs.docker.com/get-docker/) if you want to run a containerized version).
2. Create a [PushNotifier](https://pushnotifier.de) account [here](https://pushnotifier.de/signup).
3. Generate a PushNotifier API Token [here](https://pushnotifier.de/account/api) (only once you logged in) (this value will be called **`API_TOKEN`**).
4. Create an application on the same page and get:
   1. An app package name (this value will be called **`APP_PACKAGE_NAME`**)
   2. An App Token (this value will be called **`APP_TOKEN`**)
5. Install the PushNotifier app on your devices and log in (this will automatically add your device to the device list on [PushNotifier](https://pushnotifier.de/devices)).

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/omnitrogen/beta-gouv-jobs-notif
   ```
2. Create a `.env` file at the root of the project. The file should look like this:
   ```sh
    API_TOKEN="<your_api_token>"
    APP_PACKAGE_NAME="<your_app_package_name>"
    APP_TOKEN="<your_app_token>"
   ```
3. Compile and run the code:
    - With Rust:
        1. Create an optimized build with:
            ```sh
            cargo build --release
            ```
        2. Run the executable:
            ```sh
            ./target/release/beta-gouv-jobs-notif
            ```
    - With Docker:
        1. Build the image:
            ```sh
            docker build -t beta-gouv-jobs-notif .
            ```
        2. Run the container:
            ```sh
            docker run --rm -it beta-gouv-jobs-notif
            ```
            (This is just one way to run the container, you might have a better way ¯\\\_(ツ)\_/¯)

## License

Distributed under the Unlicense License. See `LICENSE` for more information.

## TODO:

- [ ] Add tests
- [ ] Unify error names

<p align="center">(<a href="#top">back to top</a>)</p>