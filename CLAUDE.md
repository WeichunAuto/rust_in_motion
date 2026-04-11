# BobbyBlog 项目开发规范

## 项目概述

基于 **Leptos 0.8.0** 开发的全栈个人博客系统，采用 SSR（服务端渲染）+ WASM（客户端 Hydration）架构。

- **前端**：Leptos 组件 + Tailwind CSS
- **后端**：Axum HTTP 服务器
- **数据库**：PostgreSQL + Sea-ORM
- **内容**：Markdown 格式，pulldown-cmark 渲染

---

## 技术栈

| 层次        | 技术           | 版本   |
| ----------- | -------------- | ------ |
| Web 框架    | Leptos         | 0.8.0  |
| HTTP 服务器 | Axum           | 0.8.0  |
| 异步运行时  | Tokio          | 1.x    |
| ORM         | Sea-ORM        | 1.1.17 |
| 数据库      | PostgreSQL     | -      |
| 样式        | Tailwind CSS   | 4.x    |
| Markdown    | pulldown-cmark | 0.13.0 |
| 序列化      | Serde          | 1.x    |
| 日志        | Tracing        | 0.1.x  |
| 错误处理    | Anyhow         | 1.x    |
| 配置        | Config (YAML)  | 0.15.x |

---

## 目录结构

```
src/
├── lib.rs                  # 库入口，注册路由和服务
├── main.rs                 # 服务端入口（SSR）
├── app.rs                  # 路由定义和应用外壳
├── constant.rs             # 全局常量（上传路径等）
├── components/             # UI 组件（按功能模块划分）
│   ├── common.rs           # 共享组件（If、Tag 等通用组件）
│   ├── header/             # 导航头部
│   ├── blog/               # 博客相关组件
│   ├── about_me/           # 关于页面组件
│   └── icons/              # SVG 图标组件
├── server_fn/              # 服务端函数（#[server] 宏标注）
│   ├── blog.rs             # 博客 CRUD
│   ├── about_me.rs         # 关于页面操作
│   ├── menu.rs             # 菜单管理
│   └── common.rs           # 公共操作（Markdown 渲染、图片上传）
├── entity/                 # Sea-ORM 自动生成的数据库模型（勿手动修改）
│   ├── blog.rs
│   ├── blog_category.rs
│   ├── about_me.rs
│   ├── tab_menu.rs
│   └── prelude.rs          # 统一导出
├── dto/                    # 数据传输对象
│   ├── blog_request_dto.rs
│   ├── blog_response_dto.rs
│   ├── about_me_dto.rs
│   ├── menu_dto.rs
│   └── blog_category_dto.rs
├── config/                 # 配置管理
│   ├── mod.rs              # 配置加载（YAML + 环境变量）
│   ├── database.rs         # 数据库配置结构
│   └── initialize.rs       # 日志和数据库初始化
└── state/
    └── app_state.rs        # 全局状态（数据库连接）

style/
├── main.scss               # Markdown 渲染样式
└── tailwind.css            # Tailwind CSS 入口

config/
├── dev.yaml                # 开发环境配置
└── prod.yaml               # 生产环境配置

migrations/                 # SQLx 数据库迁移文件
public/                     # 静态资源（favicon 等）
```

---

## 组件开发规范

### 基本结构

```rust
#[component]
pub fn MyComponent(
    #[prop(optional)] title: Option<String>,
    #[prop(default = false)] is_active: bool,
) -> impl IntoView {
    view! {
        <div class="...">...</div>
    }
}
```

- 组件函数名使用 **PascalCase**
- 使用 `#[prop(optional)]` 声明可选 prop
- 使用 `#[prop(default = value)]` 声明有默认值的 prop

### 响应式状态

```rust
// 可读写信号（本地可变状态）
let count = RwSignal::new(0);

// 只读信号（对外暴露）
let read_only: ReadSignal<i32> = count.read_only();

// 从 Context 获取共享状态
let is_mobile = use_context::<ReadSignal<Option<bool>>>()
    .expect("未找到屏幕尺寸 context");
```

### 数据加载

```rust
// 一次性加载（不会因信号变化重新请求）
let data = OnceResource::new(fetch_data());

// 响应式资源（随依赖信号变化重新请求）
let data = Resource::new(move || category.get(), |cat| fetch_blogs(cat));

// 在视图中使用 Suspense 包裹
view! {
    <Suspense fallback=move || view! { <SkeletonLoader /> }>
        {move || data.get().map(|d| view! { <Content data=d /> })}
    </Suspense>
}
```

### 副作用

```rust
// 监听信号变化执行副作用
Effect::new(move |_| {
    let val = some_signal.get();
    // 执行操作...
});
```

### Context 使用

```rust
// 提供 Context（在父组件中）
provide_context(my_signal);

// 消费 Context（在子组件中）
let val = use_context::<ReadSignal<T>>().expect("context 说明");
```

---

## 服务端函数规范

### 基本结构

```rust
#[server]
pub async fn my_server_fn(param: String) -> Result<MyDto, ServerFnError> {
    let state = expect_context::<AppState>();
    let db = &state.db;

    // 数据库操作...
    let result = MyEntity::find()
        .filter(my_entity::Column::Id.eq(id))
        .one(db)
        .await?;

    Ok(MyDto::from(result))
}
```

- 所有服务端函数返回 `Result<T, ServerFnError>`
- 通过 `expect_context::<AppState>()` 获取数据库连接
- 仅在 `ssr` feature 下执行，客户端仅调用接口

### 错误处理

```rust
// 业务逻辑错误
if param.is_empty() {
    return Err(ServerFnError::ServerError("参数不能为空".to_string()));
}

// 数据库错误（直接用 ? 传播）
let record = Entity::find().one(db).await?;
```

### 仅服务端代码

```rust
#[cfg(feature = "ssr")]
fn helper_fn() {
    // 仅在服务端编译的代码
}
```

---

## 数据层规范

### Entity（实体）

- 位于 `src/entity/`，由 **Sea-ORM CLI 自动生成**，**不要手动修改**
- 如需变更数据库结构，先在 `migrations/` 中创建迁移文件，再重新生成 entity

### DTO（数据传输对象）

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlogResponseDto {
    id: i32,
    title: String,
    // ...
}

impl BlogResponseDto {
    pub fn get_id(&self) -> i32 { self.id }
    pub fn get_title(&self) -> &str { &self.title }
}
```

- 请求 DTO：`*_request_dto.rs`，用于接收用户输入
- 响应 DTO：`*_response_dto.rs`，用于向客户端返回数据
- 所有 DTO 必须派生：`Serialize, Deserialize, Clone, Debug`
- 字段访问通过 getter 方法，不直接暴露字段

### 数据库查询模式

```rust
// 查询单条
Entity::find_by_id(id).one(db).await?

// 查询多条（带过滤）
Entity::find()
    .filter(entity::Column::CategoryId.eq(category_id))
    .order_by_desc(entity::Column::CreatedAt)
    .all(db)
    .await?

// 插入
let model = entity::ActiveModel {
    field: Set("value".to_string()),
    ..Default::default()
};
Entity::insert(model).exec(db).await?

// 级联删除（关联表会自动清理）
Entity::delete_by_id(id).exec(db).await?
```

---

## 命名规范

| 类型           | 规范                                     | 示例                       |
| -------------- | ---------------------------------------- | -------------------------- |
| 文件名         | snake_case                               | `blog_list.rs`             |
| 组件名         | PascalCase                               | `BlogList`                 |
| 信号/变量      | snake_case                               | `is_loading`               |
| 常量           | SCREAMING_SNAKE_CASE                     | `UPLOAD_DIR`               |
| DTO 文件       | `*_request_dto.rs` / `*_response_dto.rs` | `blog_request_dto.rs`      |
| 服务端函数文件 | 功能名                                   | `blog.rs`, `about_me.rs`   |
| 迁移文件       | `{timestamp}_{description}.sql`          | `20240101_create_blog.sql` |

---

## 样式规范

### Tailwind CSS（主要）

- 使用 Tailwind 工具类作为主要样式方案
- 移动端优先：`sm:`, `md:`, `lg:` 响应式前缀
- 骨架屏加载：使用 `animate-pulse` 类

```rust
view! {
    // 响应式网格
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        ...
    </div>

    // 骨架屏
    <div class="animate-pulse bg-gray-200 h-4 w-full rounded"></div>
}
```

### SCSS（Markdown 渲染专用）

- Markdown 内容外层包裹 `.markdown-body` 类
- 样式定义在 `style/main.scss`
- 支持：表格、代码块、脚注、删除线、任务列表

### 移动端适配

- 通过 Context 共享屏幕尺寸状态，不直接访问 DOM
- PC 和移动端使用独立组件（`PcHeader` / `MobileHeader`）

---

## 路由规范

路由定义在 `src/app.rs`：

```rust
// 嵌套路由
<ParentRoute path=StaticSegment("blog") view=BlogLayout>
    <Route path=StaticSegment("") view=BlogList />
    <Route path=param_segment("blog_id") view=BlogDetail />
</ParentRoute>
```

- 根路径 `/` 重定向到默认分类（如 `/Web`）
- 路径参数使用 `:param_name` 格式
- 管理后台路由与公开路由分离

---

## 配置管理

### 配置文件

```yaml
# config/dev.yaml
server:
  port: 3005
database:
  host: localhost
  port: 5432
  name: bobby_blog
  user: postgres
  password: xxx
```

### 环境变量覆盖

以 `APP_` 为前缀，可覆盖 YAML 配置中的任意字段：

```bash
APP_DATABASE_HOST=prod-db.example.com
APP_DATABASE_PASSWORD=secret
```

### 访问配置

```rust
use crate::config::APP_CONFIG;

let db_host = &APP_CONFIG.database.host;
```

---

## 构建与开发

### 构建说明

- 使用 `cargo leptos` 构建，不要用普通 `cargo build`
- 构建产物：
  - 服务端二进制：`target/release/rust_in_motion`
  - 静态资源：`target/site/`

---

## 图片上传规范

- 博客封面：`data/uploads/blog/covers/{timestamp}.jpg`
- 文章内图片：`data/uploads/blog/contents/{YYYY-MM-DD}/{timestamp}.jpg`
- 上传前转为 Base64，服务端解码后写入文件系统
- 通过 `/uploads` 路由对外提供静态文件服务

---

## 注意事项

1. **不要手动修改 `src/entity/` 中的文件**，这些是 Sea-ORM 自动生成的，应通过迁移文件 + 重新生成来更新。

2. **服务端专用代码**必须用 `#[cfg(feature = "ssr")]` 标注，避免被编译到 WASM 中。

3. **数据库访问**只能在 `#[server]` 函数中进行，不能在组件中直接访问。

4. **移动端检测**通过 Context 共享的 `ReadSignal<Option<bool>>`，不要在组件中直接操作 DOM。

5. **资源加载**优先使用 `OnceResource`（数据不变化时），避免不必要的重复请求。

6. **Markdown 内容**存储在 `/article` 目录（已加入 .gitignore），上线时需手动同步。
