pub fn example() {
    let answer = find_largest_num();
    println!("{:#?}", answer);

    let list = [10, 100, 34, 67, 101];
    let answer = largest_abstraction(&list);
    println!("{:#?}", answer);
}

fn find_largest_num() -> i32 {
    let arr = [10, 20, 50, 30];
    let mut largest_num = arr[0];
    for i in arr {
        if i > largest_num {
            largest_num = i;
        }
    }
    largest_num
}

fn largest_abstraction(list: &[u32]) -> u32 {
    let mut largest_num = list[0];
    for &i in list {
        if i > largest_num {
            largest_num = i;
        }
    }
    largest_num
}

// fn largest_generic<T>(list: &[T]) -> T {
//     let mut largest_num = list[0];
//     for &i in list {
//         if i > largest_num {
//             largest_num = i;
//         }
//     }
//     largest_num
// }

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    #[allow(dead_code)]
    pub fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
struct MultiPoint<T, U> {
    x: T,
    y: U,
}
