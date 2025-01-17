## Domain Driven Design

### 建模元素

领域驱动设计（Domain Driven Design: DDD）中把建模元素分为如下类型，并给出了实现的最佳实践；

- Event：一个组件从外部接受消息，一般为纯数据类型，可以被序列化和反序列化。
- Service：服务类，对外提供接口和服务；作为组件的入口，一般接收 Event 作为输入，调用内部处理逻辑，然后返回 Event。
- Aggregate：聚合根，一组相关的对象，一般是有一致性约束的一组对象；作为一个整体进行操作；其中一个 Entity 作为 Root，有全局 ID；其它 Entity 不对外。
- Entity：实体类，一般是具有唯一标识的对象，有清晰的生命周期，使用 ID 比较是否相同。在 Aggregate 内部有局部 ID，否则有全局 ID；
- Value Object：值对象，一般是没有 ID 的对象，只有属性；所有属性相等则相等。一般作为 Entity 的属性，或者作为参数传递。一般 Value Object 是不可变的，整体替换（不绝对）；
- Repository：仓库，提供对 Aggregate 的跨 Service 存储，提供接口供 Service 获取、增加、移除 Aggregate；一般 Service 从接口参数或者 Event中获得 ID，然后通过 Repository 获取 Aggregate，调用其方法完成业务逻辑；
- Factory：工厂，提供创建 Aggregate 的方法，一般是复杂的创建逻辑；Factory 也是供 Service 调用的，创建的 Aggregate 通过 Repository 存储；

### 惯用实现

- Event：一般是纯数据类，作为 Service 的参数和返回值，入参一般是不可变的，内存由外部管理；
- Service：一般是无状态的，接收 Event，从 Repository 中查找并调用 Aggregate 的方法并调用完成实际的业务逻辑；Service 一般是无状态的，可以并发调用；
- Repository：一般是容器类，存储和查询 Aggregate，供 Service 使用；每种 Aggregate 一个 Repository；Repository一般是单例，并发访问需要加锁；Service 根据业务需要调用合适的 Repository； Repository 的接口有的是只读，例如统计类的查询；有的是读写，例如增加、删除等；
- Factory：一般是创建 Aggregate 的方法，复杂的创建逻辑；Factory 一般是无状态的，大多是静态类和方法，可以并发调用；
- Aggregate：一般是某一个 Entity 类作为聚合根承担该角色；Aggregate复杂内部 Entity 之间的一致性；由于 Aggregate 一般按照一致性边界划分，因此不可并发访问，需要加锁；Aggregate 有对应的 Factory 封装其创建过程，并保存在其对应的 Repository 中（如果不需要持久化，就是一个基于内存的容器封装类型）；
- Entity：一般是有状态的，有 ID，有生命周期；Entity 一般是有状态的，不可并发访问（同步控制由上层的 Aggregate 控制）；有生命周期，由对应的 Aggregate 创建并聚合；Entity需要实现正确的 `eq` 和 `hash` 方法（借助 ID）；
- Value Object：一般是无状态的，不可变的；可以被多个 Entity 引用； 也可以直接被 Entity 组合；Value Object 需要实现正确的 `eq` 和 `hash` 方法（全属性）；

### 代码实现

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use uuid::Uuid;

// ----------- Value Objects -----------
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Money {
    amount: u64,
    currency: String,
}

impl Money {
    fn new(amount: u64, currency: &str) -> Self {
        Money {
            amount,
            currency: currency.to_string(),
        }
    }

    fn add(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Currency mismatch".to_string());
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    fn subtract(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Currency mismatch".to_string());
        }
        if self.amount < other.amount {
            return Err("Insufficient funds".to_string());
        }
        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }
}

// ----------- Entities -----------
#[derive(Debug, Clone)]
struct Order {
    id: Uuid,
    amount: Money,
    description: String,
}

impl Order {
    fn new(id: Uuid, amount: Money, description: &str) -> Self {
        Order {
            id,
            amount,
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Customer {
    id: Uuid,
    name: String,
    orders: Vec<Order>,
    balance: Money,
}

impl Customer {
    fn new(id: Uuid, name: &str, initial_balance: Money) -> Self {
        Customer {
            id,
            name: name.to_string(),
            orders: Vec::new(),
            balance: initial_balance,
        }
    }

    // Domain methods
    fn place_order(&mut self, amount: Money, description: &str) -> Result<Order, String> {
        self.place_order_with_id(Uuid::new_v4(), amount, description)
    }

    fn place_order_with_id(&mut self, order_id: Uuid, amount: Money, description: &str) -> Result<Order, String> {
        if self.balance.amount < amount.amount {
            return Err("Insufficient balance to place order".to_string());
        }
        self.balance = self.balance.subtract(&amount)?;
        let order = Order::new(order_id, amount, description);
        self.orders.push(order.clone());
        Ok(order)
    }

    fn cancel_order(&mut self, order_id: Uuid) -> Result<(), String> {
        if let Some(pos) = self.orders.iter().position(|o| o.id == order_id) {
            let order = self.orders.remove(pos);
            self.balance = self.balance.add(&order.amount)?;
            Ok(())
        } else {
            Err("Order not found".to_string())
        }
    }

    fn deposit(&mut self, amount: Money) -> Result<(), String> {
        self.balance = self.balance.add(&amount)?;
        Ok(())
    }

    fn withdraw(&mut self, amount: Money) -> Result<(), String> {
        self.balance = self.balance.subtract(&amount)?;
        Ok(())
    }

    fn get_balance(&self) -> &Money {
        &self.balance
    }

    fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }
}

// ----------- Repositories -----------
trait Repository {
    fn find(&self, id: &Uuid) -> Option<Arc<Mutex<Customer>>>;
    fn save(&self, customer: Arc<Mutex<Customer>>) -> Result<(), String>;
}

struct InMemoryRepository {
    store: RwLock<HashMap<Uuid, Arc<Mutex<Customer>>>>,
}

impl InMemoryRepository {
    fn new() -> Self {
        InMemoryRepository {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl Repository for InMemoryRepository {
    fn find(&self, id: &Uuid) -> Option<Arc<Mutex<Customer>>> {
        let store = self.store.read().ok()?;
        store.get(id).cloned()
    }

    fn save(&self, customer: Arc<Mutex<Customer>>) -> Result<(), String> {
        let mut store = self.store.write().map_err(|_| "Failed to acquire write lock".to_string())?;
        let id = {
            let customer_lock = customer.lock().map_err(|_| "Failed to lock customer".to_string())?;
            customer_lock.id
        };
        store.insert(id, customer);
        Ok(())
    }
}

// ----------- Factory -----------
trait Factory {
    fn create_customer(&self, id: Uuid, name: &str, initial_balance: Money) -> Result<Customer, String>;
}

struct CustomerFactoryImpl;

impl Factory for CustomerFactoryImpl {
    fn create_customer(&self, id: Uuid, name: &str, initial_balance: Money) -> Result<Customer, String> {
        Ok(Customer::new(id, name, initial_balance))
    }
}

// ----------- Events -----------
#[derive(Debug, Clone)]
enum CustomerEvent {
    CustomerCreated {
        id: Uuid,
        name: String,
        initial_balance: Money,
    },
    // 其他与客户相关的事件可在此处添加
}

#[derive(Debug, Clone)]
enum OrderEvent {
    OrderPlaced {
        customer_id: Uuid,
        order_id: Uuid,
        amount: Money,
        description: String,
    },
    OrderCancelled {
        customer_id: Uuid,
        order_id: Uuid,
    },
}

#[derive(Debug, Clone)]
enum BalanceEvent {
    BalanceDeposited {
        customer_id: Uuid,
        amount: Money,
    },
    BalanceWithdrawn {
        customer_id: Uuid,
        amount: Money,
    },
}

// ----------- Services -----------
struct CustomerService {
    repository: Arc<dyn Repository + Send + Sync>,
    factory: Arc<dyn Factory + Send + Sync>,
}

impl CustomerService {
    fn new(repository: Arc<dyn Repository + Send + Sync>, factory: Arc<dyn Factory + Send + Sync>) -> Self {
        CustomerService { repository, factory }
    }

    // 处理与客户相关的事件
    fn handle_event(&self, event: CustomerEvent) -> Result<(), String> {
        match event {
            CustomerEvent::CustomerCreated { id, name, initial_balance } => {
                let customer = self.factory.create_customer(id, &name, initial_balance)?;
                let customer_arc = Arc::new(Mutex::new(customer));
                self.repository.save(customer_arc)?;
            },
        }
        Ok(())
    }

    // 可在此处添加其他与客户相关的方法（如删除客户）
}

struct OrderService {
    repository: Arc<dyn Repository + Send + Sync>,
}

impl OrderService {
    fn new(repository: Arc<dyn Repository + Send + Sync>) -> Self {
        OrderService { repository }
    }

    // 处理与订单相关的事件
    fn handle_event(&self, event: OrderEvent) -> Result<(), String> {
        match event {
            OrderEvent::OrderPlaced { customer_id, order_id, amount, description } => {
                let customer_arc = self.repository.find(&customer_id).ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc.lock().map_err(|_| "Failed to lock customer".to_string())?;
                customer.place_order_with_id(order_id, amount, &description)?;
            },
            OrderEvent::OrderCancelled { customer_id, order_id } => {
                let customer_arc = self.repository.find(&customer_id).ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc.lock().map_err(|_| "Failed to lock customer".to_string())?;
                customer.cancel_order(order_id)?;
            },
        }
        Ok(())
    }

    // 查询特定客户的所有订单
    fn get_orders(&self, customer_id: Uuid) -> Result<Vec<Order>, String> {
        let customer_arc = self.repository.find(&customer_id).ok_or("Customer not found".to_string())?;
        let customer = customer_arc.lock().map_err(|_| "Failed to lock customer".to_string())?;
        Ok(customer.get_orders().clone())
    }
}

struct BalanceService {
    repository: Arc<dyn Repository + Send + Sync>,
}

impl BalanceService {
    fn new(repository: Arc<dyn Repository + Send + Sync>) -> Self {
        BalanceService { repository }
    }

    // 处理与余额相关的事件
    fn handle_event(&self, event: BalanceEvent) -> Result<(), String> {
        match event {
            BalanceEvent::BalanceDeposited { customer_id, amount } => {
                let customer_arc = self.repository.find(&customer_id).ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc.lock().map_err(|_| "Failed to lock customer".to_string())?;
                customer.deposit(amount)?;
            },
            BalanceEvent::BalanceWithdrawn { customer_id, amount } => {
                let customer_arc = self.repository.find(&customer_id).ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc.lock().map_err(|_| "Failed to lock customer".to_string())?;
                customer.withdraw(amount)?;
            },
        }
        Ok(())
    }

    // 查询特定客户的余额
    fn get_balance(&self, customer_id: Uuid) -> Result<Money, String> {
        let customer_arc = self.repository.find(&customer_id).ok_or("Customer not found".to_string())?;
        let customer = customer_arc.lock().map_err(|_| "Failed to lock customer".to_string())?;
        Ok(customer.get_balance().clone())
    }
}

// ----------- Tests -----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer_creation() {
        let repo = Arc::new(InMemoryRepository::new());
        let factory = Arc::new(CustomerFactoryImpl);
        let customer_service = CustomerService::new(repo.clone(), factory.clone());

        let customer_id = Uuid::new_v4();
        let event = CustomerEvent::CustomerCreated {
            id: customer_id,
            name: "Alice".to_string(),
            initial_balance: Money::new(1000, "USD"),
        };

        customer_service.handle_event(event).unwrap();

        let customer_arc = repo.find(&customer_id).unwrap();
        let customer = customer_arc.lock().unwrap();
        assert_eq!(customer.name, "Alice");
        assert_eq!(customer.balance, Money::new(1000, "USD"));
        assert!(customer.orders.is_empty());
    }

    #[test]
    fn test_place_order() {
        let repo = Arc::new(InMemoryRepository::new());
        let factory = Arc::new(CustomerFactoryImpl);
        let customer_service = CustomerService::new(repo.clone(), factory.clone());
        let order_service = OrderService::new(repo.clone());

        let customer_id = Uuid::new_v4();
        let create_event = CustomerEvent::CustomerCreated {
            id: customer_id,
            name: "Bob".to_string(),
            initial_balance: Money::new(1000, "USD"),
        };
        customer_service.handle_event(create_event).unwrap();

        let order_id = Uuid::new_v4();
        let order_event = OrderEvent::OrderPlaced {
            customer_id,
            order_id,
            amount: Money::new(200, "USD"),
            description: "Purchase Laptop".to_string(),
        };
        order_service.handle_event(order_event).unwrap();

        let customer_arc = repo.find(&customer_id).unwrap();
        let customer = customer_arc.lock().unwrap();
        assert_eq!(customer.balance, Money::new(800, "USD"));
        assert_eq!(customer.orders.len(), 1);
        let order = &customer.orders[0];
        assert_eq!(order.id, order_id);
        assert_eq!(order.amount, Money::new(200, "USD"));
        assert_eq!(order.description, "Purchase Laptop");
    }

    #[test]
    fn test_cancel_order() {
        let repo = Arc::new(InMemoryRepository::new());
        let factory = Arc::new(CustomerFactoryImpl);
        let customer_service = CustomerService::new(repo.clone(), factory.clone());
        let order_service = OrderService::new(repo.clone());

        let customer_id = Uuid::new_v4();
        let create_event = CustomerEvent::CustomerCreated {
            id: customer_id,
            name: "Charlie".to_string(),
            initial_balance: Money::new(1000, "USD"),
        };
        customer_service.handle_event(create_event).unwrap();

        let order_id = Uuid::new_v4();
        let order_place_event = OrderEvent::OrderPlaced {
            customer_id,
            order_id,
            amount: Money::new(300, "USD"),
            description: "Buy Phone".to_string(),
        };
        order_service.handle_event(order_place_event).unwrap();

        // Cancel the order
        let order_cancel_event = OrderEvent::OrderCancelled {
            customer_id,
            order_id,
        };
        order_service.handle_event(order_cancel_event).unwrap();

        let customer_arc = repo.find(&customer_id).unwrap();
        let customer = customer_arc.lock().unwrap();
        assert_eq!(customer.balance, Money::new(1000, "USD"));
        assert!(customer.orders.is_empty());
    }

    #[test]
    fn test_deposit_withdraw() {
        let repo = Arc::new(InMemoryRepository::new());
        let factory = Arc::new(CustomerFactoryImpl);
        let customer_service = CustomerService::new(repo.clone(), factory.clone());
        let balance_service = BalanceService::new(repo.clone());

        let customer_id = Uuid::new_v4();
        let create_event = CustomerEvent::CustomerCreated {
            id: customer_id,
            name: "David".to_string(),
            initial_balance: Money::new(500, "USD"),
        };
        customer_service.handle_event(create_event).unwrap();

        // Deposit
        let deposit_event = BalanceEvent::BalanceDeposited {
            customer_id,
            amount: Money::new(200, "USD"),
        };
        balance_service.handle_event(deposit_event).unwrap();

        let balance = balance_service.get_balance(customer_id).unwrap();
        assert_eq!(balance, Money::new(700, "USD"));

        // Withdraw
        let withdraw_event = BalanceEvent::BalanceWithdrawn {
            customer_id,
            amount: Money::new(300, "USD"),
        };
        balance_service.handle_event(withdraw_event).unwrap();

        let balance = balance_service.get_balance(customer_id).unwrap();
        assert_eq!(balance, Money::new(400, "USD"));
    }

    #[test]
    fn test_insufficient_balance() {
        let repo = Arc::new(InMemoryRepository::new());
        let factory = Arc::new(CustomerFactoryImpl);
        let customer_service = CustomerService::new(repo.clone(), factory.clone());
        let order_service = OrderService::new(repo.clone());

        let customer_id = Uuid::new_v4();
        let create_event = CustomerEvent::CustomerCreated {
            id: customer_id,
            name: "Eve".to_string(),
            initial_balance: Money::new(100, "USD"),
        };
        customer_service.handle_event(create_event).unwrap();

        let order_id = Uuid::new_v4();
        let order_place_event = OrderEvent::OrderPlaced {
            customer_id,
            order_id,
            amount: Money::new(150, "USD"),
            description: "Purchase Tablet".to_string(),
        };
        let result = order_service.handle_event(order_place_event);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance to place order".to_string());

        let customer_arc = repo.find(&customer_id).unwrap();
        let customer = customer_arc.lock().unwrap();
        assert_eq!(customer.balance, Money::new(100, "USD"));
        assert!(customer.orders.is_empty());
    }

    #[test]
    fn test_order_not_found() {
        let repo = Arc::new(InMemoryRepository::new());
        let factory = Arc::new(CustomerFactoryImpl);
        let customer_service = CustomerService::new(repo.clone(), factory.clone());
        let order_service = OrderService::new(repo.clone());

        let customer_id = Uuid::new_v4();
        let create_event = CustomerEvent::CustomerCreated {
            id: customer_id,
            name: "Frank".to_string(),
            initial_balance: Money::new(1000, "USD"),
        };
        customer_service.handle_event(create_event).unwrap();

        let non_existent_order_id = Uuid::new_v4();
        let order_cancel_event = OrderEvent::OrderCancelled {
            customer_id,
            order_id: non_existent_order_id,
        };
        let result = order_service.handle_event(order_cancel_event);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Order not found".to_string());

        let customer_arc = repo.find(&customer_id).unwrap();
        let customer = customer_arc.lock().unwrap();
        assert_eq!(customer.balance, Money::new(1000, "USD"));
        assert!(customer.orders.is_empty());
    }

    #[test]
    fn test_deposit_to_nonexistent_customer() {
        let repo = Arc::new(InMemoryRepository::new());
        let balance_service = BalanceService::new(repo.clone());

        let customer_id = Uuid::new_v4();
        let deposit_event = BalanceEvent::BalanceDeposited {
            customer_id,
            amount: Money::new(100, "USD"),
        };
        let result = balance_service.handle_event(deposit_event);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Customer not found".to_string());
    }

    #[test]
    fn test_order_to_nonexistent_customer() {
        let repo = Arc::new(InMemoryRepository::new());
        let order_service = OrderService::new(repo.clone());

        let customer_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();
        let order_event = OrderEvent::OrderPlaced {
            customer_id,
            order_id,
            amount: Money::new(100, "USD"),
            description: "Purchase Book".to_string(),
        };

        let result = order_service.handle_event(order_event);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Customer not found".to_string());
    }
}

// ----------- Main Function (Optional) -----------
fn main() {
    // 示例用法
    let repo = Arc::new(InMemoryRepository::new());
    let factory = Arc::new(CustomerFactoryImpl);
    let customer_service = CustomerService::new(repo.clone(), factory.clone());
    let order_service = OrderService::new(repo.clone());
    let balance_service = BalanceService::new(repo.clone());

    // 创建客户
    let customer_id = Uuid::new_v4();
    let create_event = CustomerEvent::CustomerCreated {
        id: customer_id,
        name: "Frank".to_string(),
        initial_balance: Money::new(1000, "USD"),
    };
    customer_service.handle_event(create_event).unwrap();
    println!("Customer Frank created.");

    // 下订单
    let order_id = Uuid::new_v4();
    let place_order_event = OrderEvent::OrderPlaced {
        customer_id,
        order_id,
        amount: Money::new(250, "USD"),
        description: "Buy Headphones".to_string(),
    };
    order_service.handle_event(place_order_event).unwrap();
    println!("Order placed: Buy Headphones.");

    // 查询余额
    let balance = balance_service.get_balance(customer_id).unwrap();
    println!("Balance after order: {:?}", balance);

    // 取消订单
    let cancel_order_event = OrderEvent::OrderCancelled {
        customer_id,
        order_id,
    };
    order_service.handle_event(cancel_order_event).unwrap();
    println!("Order cancelled: Buy Headphones.");

    // 查询余额
    let balance_after_cancel = balance_service.get_balance(customer_id).unwrap();
    println!("Balance after cancellation: {:?}", balance_after_cancel);

    // 存款
    let deposit_event = BalanceEvent::BalanceDeposited {
        customer_id,
        amount: Money::new(500, "USD"),
    };
    balance_service.handle_event(deposit_event).unwrap();
    println!("Deposited 500 USD.");

    // 提款
    let withdraw_event = BalanceEvent::BalanceWithdrawn {
        customer_id,
        amount: Money::new(200, "USD"),
    };
    balance_service.handle_event(withdraw_event).unwrap();
    println!("Withdrew 200 USD.");

    // 最终余额
    let final_balance = balance_service.get_balance(customer_id).unwrap();
    println!("Final Balance: {:?}", final_balance);

    // 查询订单
    let orders = order_service.get_orders(customer_id).unwrap();
    println!("Current Orders: {:?}", orders);
}
```