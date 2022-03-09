async fn greet() {
    println!("hello");
}

#[tokio::main]
async fn main() {
    // この時点ではgreet()の中身は実行されない。
    let greeting = greet();

    // 先に実行される。
    println!("world!");

    // greetingにたいして.awaitを呼び出すことでgreet()の中身が実行される
    greeting.await;
}

// #[tokio::main]
// async fn main() {
//     println!("hello");
// }
//         |      |
//         |      |
//         |      |
//         |      |
//    -----        -----
//     \              /
//      \            /     #[tokio::main]マクロをmain関数につけることで 
//       \          /      async fn main() → fn main()へと変換される。
//        \        /
//         \      /
//          \    /
//           \  /
//            \/
//
// fn main() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         println!("hello");
//     })
// }