use mlua::{
    Error as LuaError, LuaSerdeExt, MetaMethod, Table, UserData, UserDataFields, UserDataMethods,
    Value as LuaValue,
};

pub struct Json;

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

impl UserData for Json {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("null", |lua, _| Ok(lua.null()));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("array", |lua, table: Option<Table>| {
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
            MetaMethod::Call,
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
