* [Crust of Rust: Lifetime Annotations](https://youtu.be/rAl-9HwD858)

0:00:00 Introduction
0:03:36 Start a rust project
0:05:20 Struct and method definitions for StrSplit and first test
0:09:32 How you decide between a library and a binary
0:10:58 Start implementing StrSplit
0:16:15 When to use match vs if let some
0:17:10 Doesn't compile! missing lifetime specifier
0:20:33 Can I be wrong by specifying lifetimes?
0:21:25 Anonymous lifetime '_
0:23:10 Order lifetimes based on how long they are
0:25:18 Anonymous lifetime '_ (with multiple lifetimes)
0:26:52 Compile error: lifetime of reference outlives lifetime of borrowed content
0:34:45 Static lifetime
0:41:27 Bug when a delimiter tails a string
0:48:07 What is the ref keyword and why not &
0:51:36 What's the * on the left of remainder
0:52:46 What is take() doing
0:54:48 Mutable references are one level deep
0:55:39 Solving a hang with as_mut()
0:57:49 Multiple lifetimes, implementing until_char
1:03:19 Difference between a str and a String
1:08:15 Multiple lifetimes (continued)
1:15:24 Generic delimiter (Delimiter trait)
1:23:14 char length utf8
1:25:30 Standard library split
1:27:39 Q&A 