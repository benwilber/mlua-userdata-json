# Encode/Decode JSON

## Encoding

```
local s = json.encode {
	name = "Ben",
	age = 39,
	empty_array = json.array(),
	null = json.null
}
```

## Decoding

```
local t = json.decode(s)
```

## API

### `json.null`

This is a `userdata` a value that indicates to the JSON serializer that this field should be represented as a JSON `null` instead of absent.



```lua
json.encode {
  null = nil
}
```

Result

```json
{}
```

```lua
json.encode {
  null = json.null
}
```

Result

```json
{"null": null}
```
