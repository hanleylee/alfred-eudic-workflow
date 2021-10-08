# alfred-eudic-workflow

![GitHub last commit](https://img.shields.io/github/last-commit/hanleylee/alfred-eudic-workflow)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/hanleylee/alfred-eudic-workflow)
![GitHub](https://img.shields.io/github/license/hanleylee/alfred-eudic-workflow)

通过 **Alfred** 与 **Eudic** 快速查询 **当前已选择单词** 或 **搜索单词**

## 目标群体

同时使用 **Eudic** 及 **Alfred Power Pack** 的用户

## 安装

1. 进入 [Releases](https://github.com/HanleyLee/alfred-eudic-workflow/releases) 界面下载最新版本
2. 双击 `Eudic Search.alfredworkflow` 文件导入 Alfred
3. 进入`Alfred Preference` → `Workflow` → `Eudic Search`, 双击 `Hotkey` 设置启动本 Workflow 的快捷键, 建议设置为 `双击 ⌥ 键`

    ![hot-key](img/hot-key-settings.png)

4. 安装完成

## 使用

### 查询释义

1. **双击 ⌥ 键, 在有选择文本情况下直接进入 `Eudic` 释义界面, 无选择文本情况下进入自定义搜索界面**

    ![toggle](img/toggle-to-input.gif)

    ![search-selected](img/search-selected.gif)

2. `Alfred` 搜索框中输入关键字 `e` 进行搜索

### 朗读发音

搜索框激活情况下按下 `⌘` `⏎` 将由 `Eudic` 朗读单词发音

### 手动更新

`Alfred` 搜索框中输入 `update` 进行更新本 Workflow

## 待完善

- 设置自动更新
- 支持 App Store 版本的 Lite 版本 (目前支持 **官网版本** 与 **App Store 专业版**)

## 参考

- [wensonsmith/YoudaoTranslate](https://github.com/wensonsmith/YoudaoTranslate)

## 开源许可

本仓库的所有代码基于 [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0) 进行分发与使用. 协议全文见
[LICENSE](https://github.com/HanleyLee/alfred-eudic-workflow/blob/master/LICENSE) 文件.

Copyright 2021 HanleyLee

---

欢迎使用, 有任何 bug, 希望给我提 issues. 如果对你有用的话请标记一颗星星 ⭐️
