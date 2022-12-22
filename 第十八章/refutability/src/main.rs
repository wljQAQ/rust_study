/*
 * @Author: wlj
 * @Date: 2022-12-22 16:43:12
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-22 16:50:51
 * @Description: Refutability（可反驳性）: 模式是否会匹配失效
 * @see:https://kaisery.github.io/trpl-zh-cn/ch18-02-refutability.html
 */
// 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）。
// 一个例子就是 let x = 5; 语句中的 x，因为 x 可以匹配任何值所以不可能会失败。

// 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。一个这样的例子便是 if let Some(x) = a_value 表达式中的 Some(x)；
// 如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。

// 函数参数、 let 语句和 for 循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作

// if let 和 while let 表达式被限制为只能接受可反驳的模式，因为根据定义他们意在处理可能的失败：条件表达式的功能就是根据成功或失败执行不同的操作。

// 通常我们无需担心可反驳和不可反驳模式的区别，不过确实需要熟悉可反驳性的概念，这样当在错误信息中看到时就知道如何应对。
// 遇到这些情况，根据代码行为的意图，需要修改模式或者使用模式的结构。

fn main() {
    // 让我们看看一个尝试在 Rust 要求不可反驳模式的地方使用可反驳模式以及相反情况的例子。
    //在示例 18-8 中，有一个 let 语句，不过模式被指定为可反驳模式 Some(x)。
    let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value;
    // 如果 some_option_value 的值是 None，其不会成功匹配模式 Some(x)，表明这个模式是可反驳的。
    // 然而，因为 let 对于 None 匹配不能产生任何合法的代码，所以 let 语句只能接受不可反驳模式。
    // rust 会在编译时抱怨我们尝试在要求不可反驳模式的地方使用可反驳模式：`let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    // 因为我们没有覆盖（也不可能覆盖！）到模式 Some(x) 的每一个可能的值，所以 Rust 会合理地抗议。
    // 为了修复在需要不可反驳模式的地方使用可反驳模式的情况，可以修改使用模式的代码：不同于使用 let，可以使用 if let。
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
    //我们给了代码一个得以继续的出路！虽然我们没办法在避免产生错误的情况下使用不可反驳模式，但这段使用可反驳模式的代码是完全有效的。
    //如果为 if let 提供了一个总是会匹配的模式，
    // if let x = 5 {
    //     println!("{}", x);
    // };
    //尝试把不可反驳模式用到 if let 上 Rust 会抱怨将不可反驳模式用于 if let 是没有意义的：
    //基于此，match匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。
    // Rust允许我们在只有一个匹配分支的match中使用不可反驳模式，不过这么做不是特别有用，并可以被更简单的 let 语句替代。

}

