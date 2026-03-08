# `push_str()` 与 `format!()` 的区别

在 Rust 中，`push_str()` 和 `format!()` 都可以用于字符串拼接，但它们的使用场景、性能特性和底层行为完全不同。

---

## 一、`push_str()`

### ✅ 作用

向一个已存在的 `String` 末尾追加字符串。

### 📌 示例

```rust
let mut s = String::from("Hello");
s.push_str(" World");
println!("{}", s);
```

输出：

```
Hello World
```

### ⚙️ 特点

- 必须作用于 `String`
- 会 **修改原字符串**
- 不会创建新的 `String`
- 性能更高（除非容量不足才会重新分配内存）

### 🔎 方法签名

```rust
fn push_str(&mut self, string: &str)
```

它是一个方法，直接操作已有的 `String`。

---

## 二、`format!()`

### ✅ 作用

创建一个新的格式化字符串。

### 📌 示例

```rust
let name = "Rust";
let s = format!("Hello {}", name);
println!("{}", s);
```

输出：

```
Hello Rust
```

### ⚙️ 特点

- 返回一个新的 `String`
- 不会修改原变量
- 支持复杂格式化
- 可读性更好

### 🔎 本质

`format!()` 是一个宏，内部会创建新的 `String` 并写入格式化内容。

---

## 三、性能对比

| 对比项 | `push_str()` | `format!()` |
|--------|--------------|-------------|
| 是否修改原字符串 | ✅ 是 | ❌ 否 |
| 是否创建新字符串 | ❌ 不一定 | ✅ 一定 |
| 是否重新分配内存 | 可能（容量不足时） | 一定 |
| 适合频繁拼接 | ✅ 推荐 | ❌ 不推荐 |
| 支持格式化 | ❌ 不支持 | ✅ 支持 |

---

## 四、使用场景建议

### ✅ 适合用 `push_str()`

- 高频字符串拼接
- 构建日志字符串
- 构建文件路径
- 性能敏感场景

```rust
let mut path = String::new();
path.push_str("/uploads/");
path.push_str("blog/");
path.push_str("image.jpg");
```

---

### ✅ 适合用 `format!()`

- 需要格式化变量
- 拼接中包含数字
- 可读性优先

```rust
let filename = format!("{}.jpg", timestamp);
```

---

## 五、常见误区

很多人会写：

```rust
let s = format!("{}{}", a, b);
```

如果只是简单拼接，更高效的方式是：

```rust
let mut s = String::from(a);
s.push_str(b);
```

在高频场景下，`push_str()` 更节省内存。

---

## 六、总结

> 如果只是追加字符串，用 `push_str()`  
> 如果需要格式化，用 `format!()`

一句话总结：

- `push_str()` —— 修改已有字符串（更高效）
- `format!()` —— 创建新的格式化字符串（更灵活）
# `push_str()` 与 `format!()` 的区别

在 Rust 中，`push_str()` 和 `format!()` 都可以用于字符串拼接，但它们的使用场景、性能特性和底层行为完全不同。

---

## 一、`push_str()`

### ✅ 作用

向一个已存在的 `String` 末尾追加字符串。

### 📌 示例

```rust
let mut s = String::from("Hello");
s.push_str(" World");
println!("{}", s);
```

输出：

```
Hello World
```

### ⚙️ 特点

- 必须作用于 `String`
- 会 **修改原字符串**
- 不会创建新的 `String`
- 性能更高（除非容量不足才会重新分配内存）

### 🔎 方法签名

```rust
fn push_str(&mut self, string: &str)
```

它是一个方法，直接操作已有的 `String`。

---

## 二、`format!()`

### ✅ 作用

创建一个新的格式化字符串。

### 📌 示例

```rust
let name = "Rust";
let s = format!("Hello {}", name);
println!("{}", s);
```

输出：

```
Hello Rust
```

### ⚙️ 特点

- 返回一个新的 `String`
- 不会修改原变量
- 支持复杂格式化
- 可读性更好

### 🔎 本质

`format!()` 是一个宏，内部会创建新的 `String` 并写入格式化内容。

---

## 三、性能对比

| 对比项 | `push_str()` | `format!()` |
|--------|--------------|-------------|
| 是否修改原字符串 | ✅ 是 | ❌ 否 |
| 是否创建新字符串 | ❌ 不一定 | ✅ 一定 |
| 是否重新分配内存 | 可能（容量不足时） | 一定 |
| 适合频繁拼接 | ✅ 推荐 | ❌ 不推荐 |
| 支持格式化 | ❌ 不支持 | ✅ 支持 |

---

## 四、使用场景建议

### ✅ 适合用 `push_str()`

- 高频字符串拼接
- 构建日志字符串
- 构建文件路径
- 性能敏感场景

```rust
let mut path = String::new();
path.push_str("/uploads/");
path.push_str("blog/");
path.push_str("image.jpg");
```

---

### ✅ 适合用 `format!()`

- 需要格式化变量
- 拼接中包含数字
- 可读性优先

```rust
let filename = format!("{}.jpg", timestamp);
```

---

## 五、常见误区

很多人会写：

```rust
let s = format!("{}{}", a, b);
```

如果只是简单拼接，更高效的方式是：

```rust
let mut s = String::from(a);
s.push_str(b);
```

在高频场景下，`push_str()` 更节省内存。

---

## 六、总结

> 如果只是追加字符串，用 `push_str()`  
> 如果需要格式化，用 `format!()`

一句话总结：

- `push_str()` —— 修改已有字符串（更高效）
- `format!()` —— 创建新的格式化字符串（更灵活）