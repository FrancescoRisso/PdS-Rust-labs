// use es_02::{node::Node, Filesystem};

// #[test]
// fn demo() {
//     let mut fs = Filesystem::new();

//     // create a directory structure, 10 dirs with a child dir and file each one
//     for i in 0..10 {
//         fs.mkdir("/", format!("dir{}", i).as_str()).unwrap();
//         fs.mkdir(format!("/dir{}", i).as_str(), "child1").unwrap();
//         // fs.create_file(format!("/dir{}", i).as_str(), "file1")
//         //     .unwrap();
//     }

//     println!("find /child");
//     if let Ok(res) = fs.get("/dir2/child1") {
//         match res {
//             Node::Dir(_) => {
//                 println!("dir2 found");
//             }
//             // try to match all possible errros
//             _ => {}
//         }
//     } else {
//         println!("not found");
//     }

//     //     // let's try with matches
//     //     let matches = fs.find(&["name:child1", "type:file"]);
//     //     for m in matches {
//     //         match m.node {
//     //             Node::File(f) => {
//     //                 // inspect content
//     //             },
//     //             Node::Dir(d) => {
//     //                 // inspect children
//     //             },
//     //             _ => {}
//     //         }
//     //     }

//     //     // see note "riferimenti mutabili" in exercise text
//     //     // now let's try to modify the filesystem using the found matches
//     //     // is it possible to do it? which error do you get from the compiler?
//     //     let matches = fs.find(&["/dir2/child1", "/dir3/child1"]);
//     //     for m in matches {
//     //         let node = fs.get_mut(m.path).unwrap();
//     //         match node {
//     //             Node::File(f) => {
//     //                 // inspect content
//     //             }
//     //             _ => {}
//     //         }
//     //     }

//     //     // how can you fix the previous code?
//     //     // suggestion: this code using paths which are not referenced by MatchResults should compile. Why?
//     //     // Therefore how can you use the paths returned in the MatchResults to modify the filesystem?
//     //     let paths = ["/dir1/child1", "/dir2/child1", "/dir3/child1"];
//     //     for p in paths {
//     //         let n = fs.get_mut(p.as_str());
//     //     }

//     //     // now let's try to walk the filesystem
//     //     fs.walk(|path, node| {
//     //         match node {
//     //             Node::File(f) => {
//     //                 println!("file: {}", path);
//     //             }
//     //             Node::Dir(d) => {
//     //                 println!("dir: {}", path);
//     //             }
//     //         }
//     //     });
// }
