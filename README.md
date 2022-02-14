# INE5653

## How to

Install [it](https://www.rust-lang.org/tools/install) and then `cargo run`.

## APIs

### Add

```bash
curl -H "Content-type: application/json" -d '{"day": "1", "month": "1", "year": "1", "description": "bla", "value": "1", "size": 1}' 'localhost:6666/add'
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