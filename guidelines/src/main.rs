#[derive(Debug)]
struct Item {
    name: String,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            name: String::new(), // 默认名称为空字符串
        }
    }
}

#[derive(Debug)]
struct StaticContainer {
    items: [Item; 3],
}

impl Default for StaticContainer {
    fn default() -> Self {
        StaticContainer {
            items: Default::default(), // 每个元素调用 Item::default()
        }
    }
}

fn main() {
    let container = StaticContainer::default();
    println!("{:?}", container);
}
