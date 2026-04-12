# 需求与方案设计：博客详情页 LinkedIn 分享功能

## 一、需求描述

在 `BlogDetailPage` 博客详情页面的底部，新增一个 **"分享到 LinkedIn"** 按钮，允许用户一键将当前博客文章分享到自己的 LinkedIn 动态。

### 功能要求

- 按钮位于文章正文内容的最底部（`<article>` 标签内）
- 点击后在**新标签页**中打开 LinkedIn 分享对话框
- 分享内容包含当前博客文章的页面 URL
- 样式与整体页面风格保持一致（Tailwind CSS）
- 纯前端实现，无需服务端支持

### 不在本次范围内

- 分享计数统计
- 其他社交平台（Twitter、Facebook 等）
- LinkedIn JavaScript SDK 集成（使用更简单的 URL scheme 即可）

---

## 二、技术方案

### 2.1 LinkedIn 分享原理

LinkedIn 提供了一个无需 SDK 的标准分享 URL：

```
https://www.linkedin.com/sharing/share-offsite/?url={encoded_page_url}
```

只需将当前页面 URL 做 URL encode 后拼入此链接，用户点击后即可跳转至 LinkedIn 的分享页面，自动填充分享内容。

### 2.2 获取当前页面 URL

在 Leptos 的 SSR + WASM 架构中，当前页面 URL 通过 `web_sys::window()` 在客户端获取：

```rust
#[cfg(feature = "hydrate")]
fn get_current_url() -> String {
    web_sys::window()
        .and_then(|w| w.location().href().ok())
        .unwrap_or_default()
}
```

> **注意**：`web_sys` 仅在客户端（hydrate feature）可用，服务端渲染阶段无法调用，需加 `#[cfg]` 守卫或使用 Leptos 的 `window()` 工具函数。

### 2.3 实现方式选型

| 方案                         | 描述                                          | 优点               | 缺点                                   |
| ---------------------------- | --------------------------------------------- | ------------------ | -------------------------------------- |
| **方案 A：纯 `<a>` 标签**    | href 由 `window.location.href` 动态构造       | 最简单，无 JS 依赖 | SSR 阶段 href 为空，hydration 后才有值 |
| **方案 B：`on:click` 事件**  | 点击时用 `window.open()` 打开分享链接         | 控制灵活，可加埋点 | 需要 JS 事件处理                       |
| **方案 C：Signal 绑定 href** | 用 `RwSignal<String>` 存储 URL，Effect 中赋值 | Leptos 原生响应式  | 稍复杂                                 |

**推荐选择方案 A**（`<a>` 标签 + `target="_blank"`），理由：

- 实现最简，符合"不过度设计"原则
- SSR 阶段 href 可以用空字符串或仅用 blog_id 路径占位
- Hydration 后 `href` 属性通过 `move ||` 闭包动态计算，Leptos 会自动更新

### 2.4 URL 编码

Leptos/WASM 环境中可使用 `js_sys::encode_uri_component()` 做 URL 编码：

```rust
let encoded_url = js_sys::encode_uri_component(&page_url);
let linkedin_url = format!(
    "https://www.linkedin.com/sharing/share-offsite/?url={}",
    encoded_url
);
```

或者在纯 Rust 中手动编码（引入 `urlencoding` crate）：

```rust
let encoded_url = urlencoding::encode(&page_url);
```

**推荐使用 `js_sys::encode_uri_component()`**，无需额外依赖，已有 `js_sys` 在 WASM 环境中可用。

---

## 三、代码变更方案

### 3.1 修改文件

**唯一改动文件**：`src/components/blog/blog_detail_page.rs`

### 3.2 变更内容

在 `<article>` 标签底部，紧接 Markdown 内容区域之后，新增分享区域：

```
<article>
    <h1>...</h1>
    <div>日期 | 阅读时长</div>
    <img 封面图 />
    <div Markdown 内容 />

    ← 新增：分享区域 →
    <div class="分享按钮容器">
        <a href=linkedin_share_url target="_blank">
            <LinkedIn 图标> Share on LinkedIn
        </a>
    </div>
</article>
```

### 3.3 示意代码

```rust
// 在 blog_detail_page.rs 中，article 底部追加：

let page_url = window().location().href().unwrap_or_default();
let encoded = js_sys::encode_uri_component(&page_url);
let linkedin_url = format!(
    "https://www.linkedin.com/sharing/share-offsite/?url={}",
    encoded
);

view! {
    // ... 原有内容 ...

    // 分享区域
    <div class="mt-12 pt-8 border-t border-gray-200 flex items-center gap-3">
        <span class="text-sm text-gray-500">"Share this article:"</span>
        <a
            href=linkedin_url
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-2 px-4 py-2 rounded-lg
                   bg-[#0A66C2] text-white text-sm font-medium
                   hover:bg-[#004182] transition-colors duration-200"
        >
            // LinkedIn 图标（SVG）
            <svg .../>
            "LinkedIn"
        </a>
    </div>
}
```

### 3.4 LinkedIn 官方品牌色

| 颜色          | HEX       | 用途       |
| ------------- | --------- | ---------- |
| 主色          | `#0A66C2` | 按钮背景   |
| 深色（Hover） | `#004182` | 鼠标悬停   |
| 白色          | `#FFFFFF` | 文字和图标 |

---

## 四、UI 设计

### 位置

文章底部，Markdown 内容结束后，一条分割线之下。

### 布局示意

```
────────────────────────────────────────
Share this article:   [🔗 LinkedIn]
```

### 样式说明

- 使用 `border-t border-gray-200` 细分割线与正文区分
- 按钮使用 LinkedIn 官方蓝色 `#0A66C2`
- 按钮左侧展示 LinkedIn Logo SVG
- `hover:bg-[#004182]` 提供悬停反馈
- `target="_blank" rel="noopener noreferrer"` 安全地在新标签页打开

---

## 五、边界情况处理

| 场景                    | 处理方式                                                                                                                  |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| SSR 阶段（无 `window`） | `window().location().href()` 返回 `Err`，`unwrap_or_default()` 兜底为空字符串，按钮渲染但 href 为空；Hydration 后自动更新 |
| 博客加载失败            | 错误状态下不渲染文章内容，分享按钮也不会出现                                                                              |
| URL 含特殊字符          | `js_sys::encode_uri_component()` 完整编码                                                                                 |

---

## 六、依赖变更

**无需新增 Cargo 依赖**。

- `js_sys` 已通过 `wasm-bindgen` 间接引入（项目已配置 `wasm-bindgen`）
- `web_sys` 已在 `Cargo.toml` 中声明（含 `Window`, `Location` 特性）
- LinkedIn SVG 图标内联，无需图片资源

唯一需确认：`web_sys` 的 features 中是否包含 `Location`。若未包含，需在 `Cargo.toml` 中补充：

```toml
[dependencies.web-sys]
features = ["Window", "Location"]  # 确保这两个已存在
```
