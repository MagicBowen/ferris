use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
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

    fn place_order_with_id(
        &mut self,
        order_id: Uuid,
        amount: Money,
        description: &str,
    ) -> Result<Order, String> {
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
        let mut store = self
            .store
            .write()
            .map_err(|_| "Failed to acquire write lock".to_string())?;
        let id = {
            let customer_lock = customer
                .lock()
                .map_err(|_| "Failed to lock customer".to_string())?;
            customer_lock.id
        };
        store.insert(id, customer);
        Ok(())
    }
}

// ----------- Factory -----------
trait Factory {
    fn create_customer(
        &self,
        id: Uuid,
        name: &str,
        initial_balance: Money,
    ) -> Result<Customer, String>;
}

struct CustomerFactoryImpl;

impl Factory for CustomerFactoryImpl {
    fn create_customer(
        &self,
        id: Uuid,
        name: &str,
        initial_balance: Money,
    ) -> Result<Customer, String> {
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
    BalanceDeposited { customer_id: Uuid, amount: Money },
    BalanceWithdrawn { customer_id: Uuid, amount: Money },
}

// ----------- Services -----------
struct CustomerService {
    repository: Arc<dyn Repository + Send + Sync>,
    factory: Arc<dyn Factory + Send + Sync>,
}

impl CustomerService {
    fn new(
        repository: Arc<dyn Repository + Send + Sync>,
        factory: Arc<dyn Factory + Send + Sync>,
    ) -> Self {
        CustomerService {
            repository,
            factory,
        }
    }

    // 处理与客户相关的事件
    fn handle_event(&self, event: CustomerEvent) -> Result<(), String> {
        match event {
            CustomerEvent::CustomerCreated {
                id,
                name,
                initial_balance,
            } => {
                let customer = self.factory.create_customer(id, &name, initial_balance)?;
                let customer_arc = Arc::new(Mutex::new(customer));
                self.repository.save(customer_arc)?;
            }
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
            OrderEvent::OrderPlaced {
                customer_id,
                order_id,
                amount,
                description,
            } => {
                let customer_arc = self
                    .repository
                    .find(&customer_id)
                    .ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc
                    .lock()
                    .map_err(|_| "Failed to lock customer".to_string())?;
                customer.place_order_with_id(order_id, amount, &description)?;
            }
            OrderEvent::OrderCancelled {
                customer_id,
                order_id,
            } => {
                let customer_arc = self
                    .repository
                    .find(&customer_id)
                    .ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc
                    .lock()
                    .map_err(|_| "Failed to lock customer".to_string())?;
                customer.cancel_order(order_id)?;
            }
        }
        Ok(())
    }

    // 查询特定客户的所有订单
    fn get_orders(&self, customer_id: Uuid) -> Result<Vec<Order>, String> {
        let customer_arc = self
            .repository
            .find(&customer_id)
            .ok_or("Customer not found".to_string())?;
        let customer = customer_arc
            .lock()
            .map_err(|_| "Failed to lock customer".to_string())?;
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
            BalanceEvent::BalanceDeposited {
                customer_id,
                amount,
            } => {
                let customer_arc = self
                    .repository
                    .find(&customer_id)
                    .ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc
                    .lock()
                    .map_err(|_| "Failed to lock customer".to_string())?;
                customer.deposit(amount)?;
            }
            BalanceEvent::BalanceWithdrawn {
                customer_id,
                amount,
            } => {
                let customer_arc = self
                    .repository
                    .find(&customer_id)
                    .ok_or("Customer not found".to_string())?;
                let mut customer = customer_arc
                    .lock()
                    .map_err(|_| "Failed to lock customer".to_string())?;
                customer.withdraw(amount)?;
            }
        }
        Ok(())
    }

    // 查询特定客户的余额
    fn get_balance(&self, customer_id: Uuid) -> Result<Money, String> {
        let customer_arc = self
            .repository
            .find(&customer_id)
            .ok_or("Customer not found".to_string())?;
        let customer = customer_arc
            .lock()
            .map_err(|_| "Failed to lock customer".to_string())?;
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
        assert_eq!(
            result.unwrap_err(),
            "Insufficient balance to place order".to_string()
        );

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
