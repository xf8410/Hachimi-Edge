use std::collections::BTreeMap;

use crate::{
    core::{Hachimi, game::Region},
    il2cpp::{ext::Il2CppStringExt, symbols::{get_method_overload_addr, unbox}, types::*}
};

use super::TextId;

// 文本替换层已移除（clean 版）
//
// 原来 Gallop::Localize::Get 是游戏内 UI 文本的主要替换入口：
//   1) 先查 localize_dict（按 TextId 名查表）→ 命中就直接返回替换文本；
//   2) 未命中且 auto_translate_localize 打开时，走 SugoiClient 机翻（含异步回填）。
//
// 现在 Get 只做纯转调，一行替换都不做。
// 保留 TextId 相关的 dump_strings：它只读原始文本、不写任何东西，供导出/对表使用。

/**
 * Gallop::Localize::Get
 * Used by the game to get localized strings for builtin text (mostly UI).
 *
 * clean 版：直接返回游戏原始文本。
 */
type GetFn = extern "C" fn(id: i32) -> *mut Il2CppString;
pub extern "C" fn Get(id: i32) -> *mut Il2CppString {
    get_orig_fn!(Get, GetFn)(id)
}

pub fn dump_strings() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();

    for obj in TextId::get_values().enumerator().map(|e| e.iter()).unwrap_or_default().expect("enum values enumerator") {
        let value: i32 = unsafe { unbox(obj) };
        let name = TextId::get_name(value);
        let name_str = unsafe { (*name).as_utf16str() };

        let res = get_orig_fn!(Get, GetFn)(value);
        if !res.is_null() {
            let res_str = unsafe { (*res).as_utf16str() };
            map.insert(name_str.to_string(), res_str.to_string());
        }
    }

    map
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, Localize);

    let Get_addr = if Hachimi::instance().game.region == Region::Taiwan {
        get_method_overload_addr(Localize, "Get", &[Il2CppTypeEnum_IL2CPP_TYPE_VALUETYPE])
    } else {
        find_nested_class_or_return!(Localize, JP);
        get_method_overload_addr(JP, "Get", &[Il2CppTypeEnum_IL2CPP_TYPE_VALUETYPE])
    };

    new_hook!(Get_addr, Get);
}
