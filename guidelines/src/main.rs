struct Student {
    name: String,
    height: u32,
    age: u32,
}

fn sort<T, F>(items: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = items.len();
    for y in 0..len-1 {
        for x in 1..len-y {
            if compare(&items[x], &items[x-1]) {
                items.swap(x, x-1);
            }
        }
    }
}

fn main() {
    let mut students = vec![
        Student { name: "Alice".to_string(), height: 165, age: 18 },
        Student { name: "Bob".to_string(), height: 180, age: 16 },
        Student { name: "Charlie".to_string(), height: 170, age: 20 },
    ];
    
    sort(&mut students, |a, b| a.height > b.height);
    
    sort(&mut students, |a, b| a.age > b.age);
    
    println!("\n按年龄排序后:");
    for s in &students {
        println!("{}: 身高 {}, 年龄 {}", s.name, s.height, s.age);
    }
    
    // 按名字排序（升序）
    sort(&mut students, |a, b| a.name < b.name);
    println!("\n按名字排序后:");
    for s in &students {
        println!("{}: 身高 {}, 年龄 {}", s.name, s.height, s.age);
    }
    
    // 可以使用任意复杂的比较逻辑
    sort(&mut students, |a, b| {
        if a.age == b.age {
            a.height > b.height  // 年龄相同时按身高降序
        } else {
            a.age < b.age  // 主要按年龄升序
        }
    });
    println!("\n按年龄升序（年龄相同时按身高降序）排序后:");
    for s in &students {
        println!("{}: 身高 {}, 年龄 {}", s.name, s.height, s.age);
    }
}