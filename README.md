# Rust Stock API 

A REST API built in Rust for fetching and analyzing stock market data. 
This project serves as a learning sandbox for mastering the Rust ecosystem, handling asynchronous external data fetching, and eventually integrating machine learning models.
(This is my first rust api so bear with me)

---

##  Getting Started

### Prerequisites
Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine.

### Running the API
To start the development server locally, run the following command in your terminal:

```bash
cargo run
```

As pr. 13-03-2026 its hosted on: "https://oskar-rust-api.onrender.com" hence you just append the `/is_alive` or other to the url. 

Once running, the API will be available at: `http://localhost:3000` (for now its not hosted somewhere)

---

##  Endpoints

### 1. Health Check
Verify that the service is running correctly.
* **URL:** `/is_alive`
* **Method:** `GET`
* **Example:** `http://localhost:3000/is_alive`

### 2. Get Ticker Data
Fetch historical price data for a specific stock ticker.
* **URL:** `/data/{ticker}`
* **Parameters:**
    * `interval`: Time interval (e.g., `m`, `d`, `wk`, `mo`)
    * `range`: Period (e.g., `d`, `mo`, `y`, `max`)
* **Example:** `http://localhost:3000/data/AAPL?interval=1d&range=1d`

### 3. Exponential Moving Average 
Calculates the EMA for a given stock based on selected parameters.
* **URL:** `/ema/{ticker}`
* **Parameters:**
    * `interval`: Time interval (e.g., `1d`)
    * `range`: Period (e.g., `6mo`)
    * `smoothing_constant`: Weighting of the most recent data (e.g., `0.2`)
* **Example:** `http://localhost:3000/ema/AAPL?interval=1d&range=6mo&smoothing_constant=0.2`

### 4. `GET /var/{ticker}`

Calculates the Historical Value at Risk (VaR) for a given stock ticker based on negative log returns. This endpoint fetches historical closing prices over a specified range and interval, calculates the continuous returns, and estimates the empirical quantile for the specified confidence level.

**Query Parameters:**
* `interval` (string, required): The data interval (e.g., `1d` for daily data).
* `range` (string, required): The historical time range to fetch (e.g., `1y` for one year, `6mo` for six months).
* `alpha` (float, required): The confidence level for the Value at Risk calculation (e.g., `0.95` for 95% confidence, `0.99` for 99% confidence).

**Example Request:**
`GET /var/AAPL?interval=1d&range=1y&alpha=0.99`

**Example Response:**
```json
0.0452

---

## 🗺️ Roadmap & Learning Goals
- [x] Initial API structure setup (Axum/Actix).
- [x] Integration with external data sources for stock prices.
- [x] Implementation of a technical indicator.
- [x] Implementation of a risk measure.
- [ ] Integration of **XGBoost** for predictive analysis.
- [ ] Containerization with Docker?.

---

*Developed as part of technical skill development in Rust.*