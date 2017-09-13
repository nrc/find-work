# Find me something Rusty to work on!

A web app for finding Rust issues to work on.

For documentation on the backing data, see [schema.md](data/schema.md).

## Architecture

The backend is written in Rust, it presents as a web server. The frontend is a
single-page React app. It is served by the backend as a static file (requires
pre-compilation). The frontend queries the backend for all its data on loading
(this data is also available at the `data` endpoint for other applications).
After the initial data load, the frontend can run offline.


### Backend

The backend is configured by the JSON files in the `data` directory. On startup
and after a timeout, the backend queries the GitHub API to get data about
relevant issues. The backend then keeps this in memory and makes it available in
a convenient JSON form on the `data` endpoint. The backend also serves static
data - it will serve anything in the `static` directory verbatim, and any other
URL it will serve `static/index.html` (configurable).

The backend is configurable via `data/config.json`.


### Frontend

A single-page React app, it pulls the data from the backend and renders the app
in a fairly straightforward, hierarchical manner. There is very little state.
The frontend should be independent once it has loaded the data from the server.

html and css are in the `static` directory. If you run webpack it will compile
the src into `static` too, so it can be served by the backend.


## Setup

### Frontend

```
npm install

npm install --save marked
npm install --save react react-dom react-router-dom
npm install --save-dev babel-loader babel-core
npm install --save-dev babel-preset-react
npm install --save-dev babel-preset-es2015
npm install --save-dev babel-preset-env
npm install --save-dev babel-plugin-transform-object-rest-spread
npm install --save-dev webpack webpack-dev-server
npm install --save-dev uglifyjs-webpack-plugin
```

### Backend

You will need to make a `data/config.json`, you can copy `data/config.json.example`
and fill out the blank fields.

To test you will need to make a `back/test-token.txt`, it just needs a valid
GitHub auth token.


## Building

### Backend

```
cd back
cargo build
```

### Frontend

```
cd front
./node_modules/.bin/webpack --watch
```

## Testing

```
cd back
cargo test
```

You will need a GitHub auth token in `back/test-token.txt` and internet access.


## Running

```
cd back
cargo run --release
```
