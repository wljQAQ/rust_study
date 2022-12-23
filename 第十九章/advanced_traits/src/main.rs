/*
 * @Author: wlj
 * @Date: 2022-12-23 11:38:47
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-23 16:41:52
 * @Description: 高级 trait
 * @see:https://kaisery.github.io/trpl-zh-cn/ch19-03-advanced-traits.html
 */

//高级 trait
//第十章 “trait：定义共享的行为” 部分，我们第一次涉及到了 trait，不过就像生命周期一样，我们并没有覆盖一些较为高级的细节。现在我们更加了解 Rust 了，可以深入理解其本质了。

//关联类型在 trait 定义中指定占位符类型
// 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型。
// trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型。如此可以定义一个使用多种类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么。

// 一个带有关联类型的 trait 的例子是标准库提供的 Iterator trait。它有一个叫做 Item 的关联类型来替代遍历的值的类型。
pub trait Iterator {
    type Item; //Item是一个占位符，同时next方法定义表明它返回Option<Self::Item> 类型的值。
               //这个 trait 的实现者会指定 Item 的具体类型，然而不管实现者指定何种类型, next 方法都会返回一个包含了此具体类型值的 Option。
               //然而不管实现者指定何种类型, next 方法都会返回一个包含了此具体类型值的 Option。
               //关联类型看起来像一个类似泛型的概念，因为它允许定义一个函数而不指定其可以处理的类型。那么为什么要使用关联类型呢？
    fn next(&mut self) -> Option<Self::Item>;
}
//让我们通过一个在第十三章中出现的 Counter 结构体上实现 Iterator trait 的例子来检视其中的区别。
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; //指定了 Item 的类型为 u32

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
//那么为什么 Iterator trait 不像下面那样定义呢
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
//使用泛型时，则不得不在每一个实现中标注类型。这是因为我们也可以实现为 Iterator<String> for Counter，或任何其他类型，这样就可以有多个 Counter 的 Iterator 的实现。
//换句话说，当 trait 有泛型参数时，可以多次实现这个 trait，每次需改变泛型参数的具体类型。接着当使用 Counter 的 next 方法时，必须提供类型注解来表明希望使用 Iterator 的哪一个实现。

//通过关联类型，则无需标注类型，因为不能多次实现这个 trait。我们只能选择一次 Item 会是什么类型，因为只能有一个 impl Iterator for Counter。当调用 Counter 的 next 时不必每次指定我们需要 u32 值的迭代器。

fn main() {
    let mut c = Counter { count: 0 };
    c.next(); //变量c是特征Iterator的实例，也是next方法的调用者。结合之前的黑体内容可以得出：对于next方法而言，Self是调用者c的具体类型：Counter，而Self::Item是Counter中定义的Item类型：

    //默认泛型类型参数和运算符重载
    //当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。如果默认类型就足够的话，这消除了为具体类型实现 trait 的需要。
    // 这种情况的一个非常好的例子是用于运算符重载。运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作。
    //Rust 并不允许创建自定义运算符或重载任意运算符，不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载。
    use std::ops::Add;
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    //add 方法将两个 Point 实例的 x 值和 y 值分别相加来创建一个新的 Point。
    //Add trait 有一个叫做 Output 的关联类型，它用来决定 add 方法的返回值类型。

    //这里默认泛型类型位于 Add trait 中。这里是其定义：
    // trait Add<Rhs=Self> {
    //     type Output;

    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    // 这些代码看来应该很熟悉，这是一个带有一个方法和一个关联类型的 trait。比较陌生的部分是尖括号中的 Rhs=Self：这个语法叫做 默认类型参数（default type parameters）。
    // 它用于定义 add 方法中的 rhs 参数 如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型。

    // 这里有两个存放不同单元值的结构体，Millimeters 和 Meters。（这种将现有类型简单封装进另一个结构体的方式被称为 newtype 模式（newtype pattern，之后的 “为了类型安全和抽象而使用 newtype 模式” 部分会详细介绍。）
    //我们希望能够将毫米值与米值相加，并让 Add 的实现正确处理转换。可以为 Millimeters 实现 Add 并以 Meters 作为 Rhs
    struct Millimeters(u32); //毫米
    struct Meters(u32); //米

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    //为了使 Millimeters 和 Meters 能够相加，我们指定 impl Add<Meters> 来设定 Rhs 类型参数的值而不是使用默认的 Self。
    // 默认参数类型主要用于如下两个方面：
    // 扩展类型而不破坏现有代码。
    // 在大部分用户都不需要的特定情况进行自定义。
    // 标准库的 Add trait 就是一个第二个目的例子：大部分时候你会将两个相似的类型相加，不过它提供了自定义额外行为的能力。
    // 在 Add trait 定义中使用默认类型参数意味着大部分时候无需指定额外的参数。换句话说，一小部分实现的样板代码是不必要的，这样使用 trait 就更容易了。
    // 第一个目的是相似的，但过程是反过来的：如果需要为现有 trait 增加类型参数，为其提供一个默认类型将允许我们在不破坏现有实现代码的基础上扩展 trait 的功能。

    // 完全限定语法与消歧义：调用相同名称的方法
    // Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait。甚至直接在类型上实现开始已经有的同名方法也是可能的！
    // 不过，当调用这些同名方法时，需要告诉 Rust 我们希望使用哪一个。考虑一下示例 19-16 中的代码，这里定义了 trait Pilot 和 Wizard 都拥有方法 fly。
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let person = Human;
    person.fly(); //当调用 Human 实例的 fly 时，编译器默认调用直接实现在类型上的方法*waving arms furiously*
                  // 为了能够调用 Pilot trait 或 Wizard trait 的 fly 方法，我们需要使用更明显的语法以便能指定我们指的是哪个 fly 方法。
    Pilot::fly(&person);
    Wizard::fly(&person);
    //因为 fly 方法获取一个 self 参数，如果有两个 类型 都实现了同一 trait，Rust 可以根据 self 的类型计算出应该使用哪一个 trait 实现。
    //然而，关联函数是 trait 的一部分，但没有 self 参数。当同一作用域的两个类型实现了同一 trait，Rust 就不能计算出我们期望的是哪一个类型，除非使用 完全限定语法（fully qualified syntax）。
    //示例 19-19 中的 Animal trait 来说，它有关联函数 baby_name，结构体 Dog 实现了 Animal，同时有关联函数 baby_name 直接定义于 Dog 之上
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    //这段代码用于一个动物收容所，他们将所有的小狗起名为 Spot，这实现为定义于 Dog 之上的关联函数 baby_name。
    //Dog 类型还实现了 Animal trait，它描述了所有动物的共有的特征。小狗被称为 puppy
    //这表现为 Dog 的 Animal trait 实现中与 Animal trait 相关联的函数 baby_name。
    //在 main 调用了 Dog::baby_name 函数，它直接调用了定义于 Dog 之上的关联函数。A baby dog is called a Spot
    //这并不是我们需要的。我们希望调用的是 Dog 上 Animal trait 实现那部分的 baby_name 函数，这样能够打印出 A baby dog is called a puppy。
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    //通常，完全限定语法定义为：
    //<Type as Trait>::function(receiver_if_method, next_arg, ...);

    //父 trait 用于在另一个 trait 中使用某 trait 的功能
    // 有时我们可能会需要某个 trait 使用另一个 trait 的功能。在这种情况下，需要能够依赖相关的 trait 也被实现。这个所需的 trait 是我们实现的 trait 的 父（超） trait（supertrait）。
    // 例如我们希望创建一个带有 outline_print 方法的 trait OutlinePrint，它会打印出带有星号框的值。
    use std::fmt;
    //在 outline_print 的实现中，因为希望能够使用 Display trait 的功能，则需要说明 OutlinePrint 只能用于同时也实现了 Display 并提供了 OutlinePrint 需要的功能的类型。
    //可以通过在 trait 定义中指定 OutlinePrint: Display 来做到这一点。这类似于为 trait 增加 trait bound
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    //因为指定了 OutlinePrint 需要 Display trait，则可以在 outline_print 中使用 to_string，其会为任何实现 Display 的类型自动实现。
    // 如果不在 trait 名后增加 : Display 并尝试在 outline_print 中使用 to_string，则会得到一个错误说在当前作用域中没有找到用于 &Self 类型的方法 to_string。

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    //newtype 模式用以在外部类型上实现外部 trait
    // 在第十章的 “为类型实现 trait” 部分，我们提到了孤儿规则（orphan rule），它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。
    // 一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体（第五章 “用没有命名字段的元组结构体来创建不同的类型” 部分介绍了元组结构体）中创建一个新类型。
    // 这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装。接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait。
    // Newtype 是一个源自 （U.C.0079，逃） Haskell 编程语言的概念。使用这个模式没有运行时性能惩罚，这个封装类型在编译时就被省略了。
    // 例如，如果想要在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，因为 Display trait 和 Vec<T> 都定义于我们的 crate 之外。

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    // Display 的实现使用 self.0 来访问其内部的 Vec<T>，因为 Wrapper 是元组结构体而 Vec<T> 是结构体总位于索引 0 的项。接着就可以使用 Wrapper 中 Display 的功能了。
    // 此方法的缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；必须直接在 Wrapper 上实现 Vec<T> 的所有方法
    // 这样就可以代理到self.0 上 —— 这就允许我们完全像 Vec<T> 那样对待 Wrapper
    // 如果希望新类型拥有其内部类型的每一个方法，为封装类型实现 Deref trait（第十五章 “通过 Deref trait 将智能指针当作常规引用处理” 部分讨论过）并返回其内部类型是一种解决方案
    // 如果不希望封装类型拥有所有内部类型的方法 —— 比如为了限制封装类型的行为 —— 则必须只自行实现所需的方法。
}
