# INE5653

## How to

Install [it](https://www.rust-lang.org/tools/install) and then `cargo run`.

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