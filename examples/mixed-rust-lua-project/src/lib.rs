use mlua::prelude::*;

fn sum_as_string(_: &Lua, (a, b): (usize, usize)) -> LuaResult<String> {
    Ok((a + b).to_string())
}

#[mlua::lua_module]
fn sum_as_string_rs(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("sum_as_string", lua.create_function(sum_as_string)?)?;
    Ok(exports)
}
