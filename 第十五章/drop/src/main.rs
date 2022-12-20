/*
 * @Author: wlj
 * @Date: 2022-12-20 09:45:21
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-20 10:10:56
 * @Description:使用 Drop Trait 运行清理代码
 * @see:https://kaisery.github.io/trpl-zh-cn/ch15-03-drop.html
 */
//使用 Drop Trait 运行清理代码
//对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码。
//可以为任何类型提供 Drop trait 的实现,同时所指定的代码被用于释放类似于文件或网络连接的资源。
//我们在智能指针上下文中讨论 Drop 是因为其功能几乎总是用于实现智能指针。
//例如，当 Box<T> 被丢弃时会释放 box 指向的堆空间。

//在其他一些语言中，我们不得不记住在每次使用完智能指针实例后调用清理内存或资源的代码。如果忘记的话，运行代码的系统可能会因为负荷过重而崩溃。
//在 Rust 中，可以指定每当值离开作用域时被执行的代码，编译器会自动插入这些代码。于是我们就不需要在程序中到处编写在实例结束时清理这些变量的代码 —— 而且还不会泄漏资源。
//指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。为了能够看出 Rust 何时调用 drop，让我们暂时使用 println! 语句实现 drop。
//展示了唯一定制功能就是当其实例离开作用域时，打印出 Dropping CustomSmartPointer! 的结构体 CustomSmartPointer。这会演示 Rust 何时运行 drop 函数：

struct CustomSmartPointer {
    data: String,
}

//Drop trait 包含在 prelude 中，所以无需导入它。
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        //我们在 CustomSmartPointer 上实现了 Drop trait，并提供了一个调用 println! 的 drop 方法实现。
        //drop 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方。这里选择打印一些文本以展示 Rust 何时调用 drop。
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        //Drop trait 实现中指定的代码可以用于许多方面，来使得清理变得方便和安全：
        //比如可以用其创建我们自己的内存分配器！
        //通过 Drop trait 和 Rust 所有权系统，你无需担心之后的代码清理，Rust 会自动考虑这些问题。
    }
}
//通过 std::mem::drop 提早丢弃值
//不幸的是，我们并不能直截了当的禁用 drop 这个功能。通常也不需要禁用 drop ；整个 Drop trait 存在的意义在于其是自动处理的。
//然而，有时你可能需要提早清理某个值。一个例子是当使用智能指针管理锁时；
//你可能希望强制运行 drop 方法来释放锁以便作用域中的其他代码可以获取锁。
//Rust 并不允许我们主动调用 Drop trait 的 drop 方法；当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // c.drop();//报错 这是不被允许的调用drop explicit destructor calls not allowed
    //报错信息使用了术语 析构函数 （destructor），这是一个清理实例的函数的通用编程概念。
    //析构函数 对应创建实例的 构造函数。Rust 中的 drop 函数就是这么一个析构函数。
    //Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。
    //因为不能禁用当值离开作用域时自动插入的 drop，并且不能显式调用 drop，如果我们需要强制提早清理值，可以使用 std::mem::drop 函数

    //std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望提早强制丢弃的值作为参数。
    //std::mem::drop 位于 prelude所以我们不需要引入。
    println!("CustomSmartPointer created.");
    drop(c);
    //我们也无需担心意外的清理掉仍在使用的值，这会造成编译器错误：所有权系统确保引用总是有效的，也会确保 drop 只会在值不再被使用时被调用一次。
    println!("CustomSmartPointer dropped before the end of main.");
} //当实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码。变量以被创建时相反的顺序被丢弃，所以 d 在 c 之前被丢弃。
