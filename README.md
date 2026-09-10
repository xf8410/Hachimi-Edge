<div align="center">

# 🧬 Hachimi-Edge（上游 fork）

**Hachimi 资源注入框架的增强分支基座**

![分支](https://img.shields.io/badge/分支-9-10B981?style=flat-square) ![版本](https://img.shields.io/badge/版本-0-F59E0B?style=flat-square) ![CI](https://img.shields.io/badge/CI-0-3B82F6?style=flat-square)

</div>

---

> 📌 **一句话定位**：Hachimi 资源注入框架的增强分支基座

## 🧭 项目定位

<b>Hachimi-Edge</b> 是上游 <a href="https://github.com/kairusds/Hachimi-Edge">kairusds/Hachimi-Edge</a> 的 fork：游戏资源替换注入框架的增强分支。本仓在本项目组中的角色是 <b>gala 系列改造仓（hachimi-edge-gala 等）的上游基座</b>——上游演进先落在这里，再由改造仓适配到 BestSoccer（galasports）等目标应用。

## ✨ 核心功能

- 游戏启动期资源替换与注入框架（Hachimi 系）
- 上游 Hachimi-Edge 的增强功能分支
- 含多语言说明文档（英/简中/繁中）与社区渠道

## 🌿 分支导览（共 9 类分支全览）

<details open>
<summary><b>点击收起/展开全部分支用途说明</b></summary>

| 分支 | 用途说明 |
|---|---|
| `main` | 主干：跟随上游 Hachimi-Edge 增强分支 |
| `staging` | 上游集成分支 |
| `auto-security-fix` | 自动安全修复线 |
| `downloader-integrity` | 资源下载器完整性校验线 |
| `fix-same-chara` | 同角色资源冲突修复线 |
| `revert-152-beyond_live_crash_fix` | 回滚 #152（Beyond Live 崩溃修复）的回退线 |
| `tl-meta` | 翻译元数据线 |
| `webview-edits` | WebView 编辑支持线 |
| `workbench/game-init-always` | 游戏初始化常驻实验线 |

</details>

## 🏷️ 版本历史

无 release（上游 fork，发版在上游与改造仓进行）。

完整版本列表 ➡️ [Releases 页](../../releases)

## ⚙️ CI 流水线（共 1 条）

| 流水线 | 用途说明 |
|---|---|
| （本 fork 未启用 CI） | 构建与发布在上游仓库与 gala 改造仓进行 |


---

## 📜 历史介绍存档

> 以下为仓库原有介绍，**内容未删改**，仅移入存档区（新版介绍以本页上方为准）。

<details>
<summary><b>点击展开原 README</b></summary>


<img align="left" width="80" height="80" src="assets/icon.png">

# Hachimi Edge

English | [简体中文](README-zh_cn.md) | [繁體中文](README-zh_tw.md)

[![Discord server](https://dcbadge.limes.pink/api/server/https://discord.gg/YjBgmuqqYr)](https://discord.gg/YjBgmuqqYr)

Game enhancement and translation mod for UM:PD.

<img width="100%" height="100%" src="assets/screenshot-1.png">
<img width="100%" height="100%" src="assets/screenshot-2.png">

# ⚠️ Please don't link to this repo or Hachimi's website
We understand that you want to help people install Hachimi and have a better experience playing the game. However, this project is inherently against the game's TOS and The Game Developer most definitely wants it gone if they were ever to learn about it.

While sharing in your self-managed chat services and through private messaging is fine, we humbly ask that you refrain from sharing links to this project on public facing sites, or to any of the tools involved.

Or share them and ruin it for the dozens of Hachimi users. It's up to you.

### If you're going to share it anyways
Do what you must, but we would respectfully request that you try to label the game as "UM:PD" or "The Honse Game" instead of the actual name of the game, to avoid search engine parsing.

# Features
- **High quality translations:** Hachimi comes with advanced translation features that help translations feel more natural (plural forms, ordinal numbers, etc.) and prevent introducing jank to the UI. It also supports translating most in-game components; no manual assets patching needed!

    Supported components:
    - UI text
    - master.mdb (skill name, skill desc, etc.)
    - Race story
    - Main story/Home dialog
    - Lyrics
    - Texture replacement
    - Sprite atlas replacement

    Additionally, Hachimi does not provide translation features for only a single language; it has been designed to be fully configurable for any language.

- **Easy setup:** Just plug and play. All setup is done within the game itself, no external application needed.
- **Translation auto update:** Built-in translation updater lets you play the game as normal while it updates, and reloads it in-game when it's done, no restart needed!
- **Built-in GUI:** Comes with a config editor so you can modify settings without even exiting the game!
- **Graphics settings:** You can adjust the game's graphics settings to make full use of your device's specs, such as FPS unlocking and resolution scaling.
- **Cross-platform:** Designed from the ground up to be portable, with Windows and Android support.

# Installation
Please see the [Getting started](https://hachimi.noccu.art/docs/hachimi/getting-started.html) page.

# Special thanks
These projects have been the basis for Hachimi's development; without them, Hachimi would never have existed in its current form:

- [Trainers' Legend G](https://github.com/MinamiChiwa/Trainers-Legend-G)
- [umamusume-localify-android](https://github.com/Kimjio/umamusume-localify-android)
- [umamusume-localify](https://github.com/GEEKiDoS/umamusume-localify)
- [Carotenify](https://github.com/KevinVG207/Uma-Carotenify)
- [umamusu-translate](https://github.com/noccu/umamusu-translate)
- [frida-il2cpp-bridge](https://github.com/vfsfitvnm/frida-il2cpp-bridge)

# License
[GNU GPLv3](LICENSE)



</details>