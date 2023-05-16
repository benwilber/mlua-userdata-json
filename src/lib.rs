use mlua::prelude::*;
use serde_json;

fn json_encode(value: LuaValue, pretty: Option<bool>) -> Result<Option<String>, LuaError> {
    match pretty {
        Some(true) => match serde_json::to_string_pretty(&value) {
            Ok(s) => Ok(Some(s)),
            Err(e) => Err(LuaError::SerializeError(e.to_string())),
        },
        _ => match serde_json::to_string(&value) {
            Ok(s) => Ok(Some(s)),
            Err(e) => Err(LuaError::SerializeError(e.to_string())),
        },
    }
}

pub struct Json;

impl Json {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Json {
    fn default() -> Self {
        Self::new()
    }
}

impl LuaUserData for Json {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("null", |lua, _| Ok(lua.null()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("array", |lua, table: Option<LuaTable>| {
            let array = match table {
                Some(table) => table,
                None => lua.create_table()?,
            };

            array.set_metatable(Some(lua.array_metatable()));

            Ok(array)
        });

        methods.add_function(
            "encode",
            |_lua, (value, pretty): (LuaValue, Option<bool>)| json_encode(value, pretty),
        );

        methods.add_meta_method(
            LuaMetaMethod::Call,
            |_lua, _this, (value, pretty): (LuaValue, Option<bool>)| json_encode(value, pretty),
        );

        methods.add_function("decode", |lua, value: String| {
            match serde_json::from_str::<serde_json::Value>(&value) {
                Ok(value) => Ok(lua.to_value(&value)?),
                Err(e) => Err(LuaError::DeserializeError(e.to_string())),
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_lua() -> Result<Lua, LuaError> {
        let lua = Lua::new();
        lua.globals().set("json", Json::new())?;
        Ok(lua)
    }

    #[test]
    fn encode() -> Result<(), LuaError> {
        let lua = init_lua()?;

        let s: String = lua.load(r#"json.encode {a = "b"}"#).eval()?;
        assert_eq!(s, r#"{"a":"b"}"#);

        let s: String = lua.load(r#"json {a = "b"}"#).eval()?;
        assert_eq!(s, r#"{"a":"b"}"#);

        let s: String = lua.load(r#"json.encode {a = {}}"#).eval()?;
        assert_eq!(s, r#"{"a":{}}"#);

        let s: String = lua.load(r#"json {a = {}}"#).eval()?;
        assert_eq!(s, r#"{"a":{}}"#);

        let s: String = lua.load(r#"json.encode {a = json.array()}"#).eval()?;
        assert_eq!(s, r#"{"a":[]}"#);

        let s: String = lua.load(r#"json {a = json.array()}"#).eval()?;
        assert_eq!(s, r#"{"a":[]}"#);

        lua.load(
            r#"
            local arr = {}
            assert(arr == json.array(arr))
            assert(arr ~= json.array())
            assert(json.array() ~= json.array())
        "#,
        )
        .set_name("testing array metatable identity")?
        .exec()?;

        let s: String = lua
            .load(r#"json.encode {a = json.array {1, 2, 3}}"#)
            .eval()?;
        assert_eq!(s, r#"{"a":[1,2,3]}"#);

        let s: String = lua.load(r#"json {a = json.array {1, 2, 3}}"#).eval()?;
        assert_eq!(s, r#"{"a":[1,2,3]}"#);

        let s: String = lua.load(r#"json.encode {a = nil}"#).eval()?;
        assert_eq!(s, r#"{}"#);

        let s: String = lua.load(r#"json.encode {[0] = 1}"#).eval()?;
        assert_eq!(s, r#"{"0":1}"#);

        let s: String = lua.load(r#"json.encode {[1] = 1}"#).eval()?;
        assert_eq!(s, r#"[1]"#);

        let s: String = lua.load(r#"json.encode {a = json.null}"#).eval()?;
        assert_eq!(s, r#"{"a":null}"#);

        let s: String = lua.load(r#"json {a = json.null}"#).eval()?;
        assert_eq!(s, r#"{"a":null}"#);

        let s: String = lua.load(r#"json.encode({a = "b"}, true)"#).eval()?;
        assert_eq!(s, "{\n  \"a\": \"b\"\n}");

        let s: String = lua.load(r#"json({a = "b"}, true)"#).eval()?;
        assert_eq!(s, "{\n  \"a\": \"b\"\n}");

        Ok(())
    }

    #[test]
    fn decode() -> Result<(), LuaError> {
        let lua = init_lua()?;

        let t: LuaTable = lua.load(r#"json.decode '{"a": true}'"#).eval()?;
        assert_eq!(
            t.clone()
                .pairs()
                .collect::<Result<Vec<(String, bool)>, _>>()?,
            vec![("a".to_string(), true)]
        );

        let t: LuaTable = lua.load(r#"json.decode '{"a": "b"}'"#).eval()?;
        assert_eq!(
            t.clone()
                .pairs()
                .collect::<Result<Vec<(String, String)>, _>>()?,
            vec![("a".to_string(), "b".to_string())]
        );

        let t: LuaTable = lua.load(r#"json.decode '{"a": 1}'"#).eval()?;
        assert_eq!(
            t.clone()
                .pairs()
                .collect::<Result<Vec<(String, i64)>, _>>()?,
            vec![("a".to_string(), 1)]
        );

        let t: LuaTable = lua.load(r#"json.decode '{"a": null}'"#).eval()?;
        assert_eq!(
            t.clone()
                .pairs()
                .collect::<Result<Vec<(String, LuaValue)>, _>>()?,
            vec![("a".to_string(), lua.null())]
        );

        Ok(())
    }
}
