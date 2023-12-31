# Rust语言的所有权和引用

## 所有权（Ownership）

- Rust中的每一个值都有一个被称为其所有者（owner）的变量。
- 一次只能有一个所有者。
- 当所有者（变量）离开作用域，这个值将被丢弃。

## 不可变引用（Immutable references）

- 可以同时拥有多个不可变引用，但是这些引用不能在同一时间与任何可变引用共存。
- 通过不可变引用，可以读取数据，但不能修改数据。

## 可变引用（Mutable references）

- 在特定作用域中，针对特定数据，只能有一个可变引用。这样可以防止数据竞争。
- 可变引用在存在时，不能有不可变引用指向同一份数据。这是为了防止在数据可能被改变的同时读取该数据。
- 通过可变引用，可以读取也可以修改数据。

这些规则一起工作，以帮助Rust提供其内存安全保证，同时不需要垃圾回收。它们使你在编译时就能发现很多类型的错误，这样你就不需要在运行时通过其他手段来检测和修复这些错误。
