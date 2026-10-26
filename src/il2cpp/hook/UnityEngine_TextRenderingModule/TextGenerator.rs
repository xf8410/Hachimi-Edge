// 文本替换层已移除（clean 版）
//
// 原始实现会在 PopulateWithErrors 里用 localized_data.hashed_dict 做 FNV-64 哈希查表替换，
// 并且**不受 auto_translate_localize / auto_translate_stories 开关控制**。
// 现象：关闭翻译后 UI 数字仍被替换成占位值（如 123,456,789）、部分文字被替换成空白。
//
// 现在这里只做纯转调，不改写任何字符串。
// TemplateContext / IgnoreTGFiltersContext 保留：它们是 $(...) 模板过滤器，被 GUI 侧引用，
// 且不含任何文本替换逻辑，删掉会破坏其它模块编译。

use crate::{core::template, il2cpp::{symbols::get_method_addr, types::*}};

type PopulateWithErrorsFn = extern "C" fn(
    this: *mut Il2CppObject, str: *mut Il2CppString,
    settings: TextGenerationSettings_t, context: *mut Il2CppObject
) -> bool;

extern "C" fn PopulateWithErrors(
    this: *mut Il2CppObject, str_: *mut Il2CppString,
    settings: TextGenerationSettings_t, context: *mut Il2CppObject
) -> bool {
    // 纯转调：不查字典、不套模板、不改 settings。
    get_orig_fn!(PopulateWithErrors, PopulateWithErrorsFn)(this, str_, settings, context)
}

struct TemplateContext<'a> {
    settings: &'a mut TextGenerationSettings_t
}

impl<'a> template::Context for TemplateContext<'a> {
    fn on_filter_eval(&mut self, name: &str, args: &[template::Token]) -> Option<String> {
        // Extra filters to modify the text generation settings
        match name {
            "nb" => {
                self.settings.horizontalOverflow = TextOverflow_Allow;
                self.settings.generateOutOfBounds = true;
                self.settings.resizeTextForBestFit = false;
            }

            "anchor" => {
                // Anchor values:
                // 1  2  3
                // 4  5  6
                // 7  8  9
                // Example: $(anchor 6) = middle right
                let value = args.get(0)?;
                let template::Token::NumberLit(anchor_num) = *value else {
                    return None;
                };
                let anchor = (anchor_num as i32) - 1;
                if anchor < 0 || anchor > 8 {
                    return None;
                }
                self.settings.textAnchor = anchor;
            }

            "scale" => {
                // Example: $(scale 80) = scale font size to 80%
                let value = args.get(0)?;
                let template::Token::NumberLit(percentage) = value else {
                    return None;
                };
                self.settings.fontSize = (self.settings.fontSize as f64 * (percentage / 100.0)) as i32;
                self.settings.resizeTextForBestFit = false;
            }

            "ho" => {
                // $(ho 0) or $(ho 1)
                let value = args.get(0)?;
                let template::Token::NumberLit(overflow_num) = *value else {
                    return None;
                };
                let overflow = overflow_num as i32;
                if overflow != 0 && overflow != 1 {
                    return None;
                }
                self.settings.horizontalOverflow = overflow;
            }

            "vo" => {
                // $(vo 0) or $(vo 1)
                let value = args.get(0)?;
                let template::Token::NumberLit(overflow_num) = *value else {
                    return None;
                };
                let overflow = overflow_num as i32;
                if overflow != 0 && overflow != 1 {
                    return None;
                }
                self.settings.verticalOverflow = overflow;
            }

            "ls" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(ls) = value else {
                    return None;
                };
                self.settings.lineSpacing = ls as f32;
            }

            "ub" => {
                self.settings.updateBounds = true;
            }

            "afit" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(state) = *value else {
                    return None;
                };
                self.settings.resizeTextForBestFit = state != 0.0;

                if self.settings.resizeTextForBestFit {
                    self.settings.generateOutOfBounds = false;
                    self.settings.resizeTextMaxSize = self.settings.fontSize;

                    // Optional args.
                    if let Some(template::Token::NumberLit(min_size)) = args.get(1) {
                        if *min_size > 0.0 {
                            self.settings.resizeTextMinSize = *min_size as _;
                        }
                    }
                    if let Some(template::Token::NumberLit(max_size)) = args.get(2) {
                        if *max_size > 0.0 {
                            self.settings.resizeTextMaxSize = *max_size as _;
                        }
                    }
                }
            }

            "minw" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(minwidth_num) = *value else {
                    return None;
                };
                let minwidth = minwidth_num as f32;
                if minwidth > self.settings.generationExtents.x {
                    self.settings.generationExtents.x = minwidth;
                }
            }

            "minh" => {
                let value = args.get(0)?;
                let template::Token::NumberLit(minheight_num) = *value else {
                    return None;
                };
                // let extents = &mut self.settings.generationExtents;
                let minheight = minheight_num as f32;
                if minheight > self.settings.generationExtents.y {
                    self.settings.generationExtents.y = minheight;
                }
            }

            _ => return None
        }

        Some(String::new())
    }
}

// Context that ignores TextGenerator filters
pub struct IgnoreTGFiltersContext();

impl template::Context for IgnoreTGFiltersContext {
    fn on_filter_eval(&mut self, _name: &str, _args: &[template::Token]) -> Option<String> {
        match _name {
            "nb" | "anchor" | "scale" | "ho" | "vo" | "ls" | "ub" | "afit" | "minw" | "minh" => Some(String::new()),
            _ => None
        }
    }
}

pub fn init(UnityEngine_TextRenderingModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_TextRenderingModule, UnityEngine, TextGenerator);

    let PopulateWithErrors_addr = get_method_addr(TextGenerator, c"PopulateWithErrors", 3);

    new_hook!(PopulateWithErrors_addr, PopulateWithErrors);
}
