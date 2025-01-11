use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 定义 Entity 结构体
#[derive(Debug)]
struct Entity {
    id: usize,
    data: String,
}

// 定义全局静态 Repo 和 Aggregator，使用 thread_local! 进行单线程局部存储
thread_local! {
    static REPO: RefCell<Vec<Option<Rc<RefCell<Entity>>>>> = RefCell::new(Vec::new());
    static AGGREGATOR: RefCell<Weak<RefCell<Entity>>> = RefCell::new(Weak::new());
}

// 函数：向 Repo 中添加一个 Entity，确保其放在 id 对应的索引位置
fn add_entity(entity: Entity) -> usize {
    REPO.with(|repo| {
        let mut repo = repo.borrow_mut();
        let id = entity.id;
        if id >= repo.len() {
            repo.resize(id + 1, None); // 扩展 Vec 大小，填充 None
        }
        repo[id] = Some(Rc::new(RefCell::new(entity)));
        id
    })
}

// 函数：根据 id 从 Repo 中移除一个 Entity
fn remove_entity(id: usize) {
    REPO.with(|repo| {
        let mut repo = repo.borrow_mut();
        if id < repo.len() {
            repo[id] = None;
        }
    })
}

// 函数：根据 id 从 Repo 中获取一个 Entity 的 Rc 指针
fn get_entity(id: usize) -> Option<Rc<RefCell<Entity>>> {
    REPO.with(|repo| {
        repo.borrow().get(id).and_then(|opt| opt.clone())
    })
}

// 函数：设置 Aggregator 指向满足特定条件的 Entity
fn set_aggregator<F>(predicate: F)
where
    F: Fn(&Entity) -> bool,
{
    REPO.with(|repo| {
        let repo = repo.borrow();
        let weak = repo.iter()
            .flatten() // 展开 Option，过滤掉 None
            .find(|e_rc| {
                let e = e_rc.borrow();
                predicate(&e)
            })
            .map(|e_rc| Rc::downgrade(e_rc)) // 转换为 Weak 指针
            .unwrap_or_else(|| Weak::new());

        AGGREGATOR.with(|agg| {
            *agg.borrow_mut() = weak;
        })
    })
}

// 函数：获取 Aggregator 当前指向的 Entity，如果存在的话
fn get_aggregator_entity() -> Option<Rc<RefCell<Entity>>> {
    AGGREGATOR.with(|agg| {
        agg.borrow().upgrade()
    })
}

fn main() {
    // 添加一些实体
    add_entity(Entity { id: 0, data: "Entity0".to_string() });
    add_entity(Entity { id: 1, data: "Entity1".to_string() });
    add_entity(Entity { id: 2, data: "Entity2".to_string() });

    // 设置 Aggregator 指向 id 为 1 的实体
    set_aggregator(|e| e.id == 1);

    // 通过 Repo 修改实体数据
    if let Some(entity_rc) = get_entity(1) {
        let mut entity = entity_rc.borrow_mut();
        entity.data = "UpdatedEntity1".to_string();
    }

    // 通过 Aggregator 访问实体
    if let Some(entity_rc) = get_aggregator_entity() {
        let entity = entity_rc.borrow();
        println!("Aggregator sees: {:?}", entity);
    } else {
        println!("Aggregator does not point to any entity.");
    }

    // 移除实体 id 为 1
    remove_entity(1);

    // 尝试通过 Aggregator 访问实体，应该失效
    if let Some(entity_rc) = get_aggregator_entity() {
        let entity = entity_rc.borrow();
        println!("Aggregator still sees: {:?}", entity);
    } else {
        println!("Aggregator's reference is no longer valid.");
    }

    // 尝试通过 Repo 访问实体，应该不存在
    if let Some(entity_rc) = get_entity(1) {
        let entity = entity_rc.borrow();
        println!("Repo sees: {:?}", entity);
    } else {
        println!("Repo does not have entity with id 1.");
    }

    add_entity(Entity { id: 1, data: "Entity1".to_string() });

    set_aggregator(|e| e.id == 1);

    // 尝试通过 Aggregator 访问实体，应该失效
    if let Some(entity_rc) = get_aggregator_entity() {
        let mut entity = entity_rc.borrow_mut();
        entity.data = "weak to entityweak".to_string();
        println!("Aggregator still sees: {:?}", entity);
    } else {
        println!("Aggregator's reference is no longer valid.");
    }

    // 尝试通过 Repo 访问实体，应该不存在
    if let Some(entity_rc) = get_entity(1) {
        let entity = entity_rc.borrow();
        println!("Repo sees: {:?}", entity);
    } else {
        println!("Repo does not have entity with id 1.");
    }
}