use std::sync::Mutex;
use fnv::FnvHashMap;
use once_cell::sync::Lazy;
use crate::core::sugoi_client::StringInfo;
use crate::il2cpp::{ext::StringExt, symbols::get_method_addr, types::*};

// 文本替换层已移除（clean 版）
// 原 set_text_hook 会做机翻替换并登记组件；现在只做纯转调。
// ACTIVE_TEXT_MESH_COMPONENTS / apply_translations 保留（其它模块引用），但不会再被填充。

pub static ACTIVE_TEXT_MESH_COMPONENTS: Lazy<Mutex<FnvHashMap<usize, StringInfo>>> = Lazy::new(|| {
    Mutex::new(FnvHashMap::default())
});

type SetTextFn = extern "C" fn(this: *mut Il2CppObject, value: *mut Il2CppString);
pub extern "C" fn set_text_hook(this: *mut Il2CppObject, value: *mut Il2CppString) {
    get_orig_fn!(set_text_hook, SetTextFn)(this, value);
}

pub fn apply_translations(completed: &[(String, String)]) {
    let mut updates_to_apply = Vec::new();
    {
        let mut tracker = ACTIVE_TEXT_MESH_COMPONENTS.lock().unwrap();

        tracker.retain(|_, info| !info.object().is_null());

        for (orig, trans) in completed {
            let unity_string = trans.to_il2cpp_string();

            for (&ptr, saved_orig) in tracker.iter() {
                if &saved_orig.str == orig {
                    updates_to_apply.push((ptr, unity_string));
                }
            }
        }
    }

    for (ptr, unity_string) in updates_to_apply {
        get_orig_fn!(set_text_hook, SetTextFn)(ptr as *mut Il2CppObject, unity_string);
    }
}

pub fn init(UnityEngine_TextRenderingModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_TextRenderingModule, UnityEngine, TextMesh);

    let set_text_addr = get_method_addr(TextMesh, c"set_text", 1);
    new_hook!(set_text_addr, set_text_hook);
}
