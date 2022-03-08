# INE5653

## How to

First, install [it](https://www.rust-lang.org/tools/install).

Currently, two languages are supported by the app: `pt-BR`(the default at .env) and `en-US`. In order to choose the language, you can change it at the .env file or pass it when running the app.

To run it, simply `cargo run` or `APP_LANGUAGE=your_option cargo run`. Then, to interact with the APIs, open another terminal session/tab/whatever and send requests to the app!

## APIs

### Add

```bash
curl -XPOST -H "Content-type: application/json" -d '{"day": 14, "month": 2, "year": 1862, "description": "The House of the Dead ", "value": "66666", "size": 368, "country": "BRA"}' 'localhost:6666/add'
```


### Delete

```bash
curl -d '00000000-0000-0000-0000-000000000000' 'localhost:6666/delete' 
```

### Get

```bash
curl -d '00000000-0000-0000-0000-000000000000' 'localhost:6666/get' 
```

### List

```bash
curl localhost:6666/list 
```