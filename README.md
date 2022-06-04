# Encode/Decode JSON


## Usage

### Encoding

```
local s = json.encode {
	name = "Ben",
	age = 39,
	empty_array = json.array(),
	null = json.null
}
```

### Decoding

```
local t = json.decode(s)
```