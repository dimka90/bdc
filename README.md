# BDC CLI

**BDC (Bureau de Change)** is a simple command-line tool that fetches the real-time USD to NGN (Naira) exchange rate, just like checking the rates at a bureau de change. It’s built with Rust for speed and reliability.

---

## Features

- Get the current USD → NGN exchange rate in real-time
- Simple CLI interface using `clap`
- Fetches data from a reliable exchange rate API
- Built with Rust and asynchronous support using `tokio`
- Can be run locally or installed globally as a binary

---

## Installation
Using Release Binary
1. Build the release binary:
```
cargo build --release
```
2. Copy the binary to a folder in your system PATH:
   
```
# Linux/macOS
sudo cp target/release/bdc /usr/local/bin/
sudo chmod +x /usr/local/bin/
```
3. Test the installation:
```
bdc --rate
```
4. Example output:
   ```
   1 USDT == 1487.0045 NGN
   ```

Environment Variables

The CLI uses an environment variable EXCHANGE_RATE_API_KEY to fetch the exchange rate.

Create a .env file in the project root:
```

EXCHANGE_RATE_API_KEY="https://open.er-api.com/v6/latest/USD"
```
The application automatically loads the API key using the dotenv crate.

Make sure the URL or key is enclosed in double quotes to avoid parsing errors.


