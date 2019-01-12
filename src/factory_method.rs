//! 工厂方法模式，有 4 种类：物品的抽象类、物品的具体类、工厂的抽象类、工厂的具体类
//! 特征在于：工厂可以生成各种具体物品，再继续对产品进行一系列加工
//! 工厂生成哪种产品是易变的部分，应该在「工厂的具体类」里面决定本工厂生成什么产品，
//! 「工厂的抽象类」里面的物品生成函数只是「纯虚函数」，这个函数的返回值是「物品的抽象类」,
//! 而工厂后续对产品的加工流程属于不变的部分，应该由「工厂的抽象类」提供，以达到复用，
//!
//! 另外把创建对象的细节部分抽出来由一个新类来做，这种「简单工厂」只是一种编程习惯，并不是真正的设计模式

/// 抽象物品
pub trait Pizza {
    fn prepare(&self);
    fn bake(&self) {
        println!("Bake for 25 minutes at 350");
    }
    fn cut(&self) {
        println!("Cutting the pizza into diagonal slices");
    }
    fn box_(&self) {
        println!("Place pizza in official PizzaStore box");
    }
    fn name(&self) -> &str;
}

/// 具体物品
pub struct NYStyleCheesePizza {
    name: String,
    dough: String,
    sauce: String,
    toppings: Vec<String>,
}
impl NYStyleCheesePizza {
    fn new() -> Self {
        NYStyleCheesePizza {
            name: "NY Style Sauce and Cheese Pizza".to_string(),
            dough: "Thin Crust Dough".to_string(),
            sauce: "Marinara Sauce".to_string(),
            toppings: vec!["Grated Reggiano Cheese".to_string()],
        }
    }
}
impl Pizza for NYStyleCheesePizza {
    fn name(&self) -> &str {
        return &self.name;
    }
    // rust 的 trait 和 struct 是完全去耦合的，所以 trait 的方法实现无法得知 struct 里面的 field
    // 无法实现这个方法的代码复用
    fn prepare(&self) {
        println!("Preparing {}", self.name);
        println!("Tossing dough...");
        println!("Adding sauch...");
        println!("Adding toppings: ");
        for i in &self.toppings {
            println!("    {}", i);
        }
    }
}

pub struct ChicagoStyleCheesePizza {
    name: String,
    dough: String,
    sauce: String,
    toppings: Vec<String>,
}
impl ChicagoStyleCheesePizza {
    fn new() -> Self {
        ChicagoStyleCheesePizza {
            name: "Chicago Style Deep Dish Cheese Pizza".to_string(),
            dough: "Extra Thick Crust Dough".to_string(),
            sauce: "Plum Tomato Sauce".to_string(),
            toppings: vec!["Shredded Mozzarella Cheese".to_string()],
        }
    }
}
impl Pizza for ChicagoStyleCheesePizza {
    fn name(&self) -> &str {
        return &self.name;
    }
    fn prepare(&self) {
        println!("Preparing {}", self.name);
        println!("Tossing dough...");
        println!("Adding sauch...");
        println!("Adding toppings: ");
        for i in &self.toppings {
            println!("    {}", i);
        }
    }
    fn cut(&self) {
        println!("Cutting the pizza into square slices");
    }
}

/// 工厂抽象
pub trait PizzaStore {
    fn order_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(pizza_type).expect("Unknown pizza type");
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_();
        pizza
    }
    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>>;
}

// 具体工厂
pub struct NYPizzaStore;
impl NYPizzaStore {
    pub fn new() -> Self {
        NYPizzaStore {}
    }
}
impl PizzaStore for NYPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>> {
        match pizza_type {
            "cheese" => Some(Box::new(NYStyleCheesePizza::new())),
            // "veggie" => Some(Box::new(NYStyleCheesePizza::new())),
            // "clam" => Some(Box::new(NYStyleCheesePizza::new())),
            // "pepperoni" => Some(Box::new(NYStyleCheesePizza::new())),
            _ => None,
        }
    }
}

pub struct ChicagoPizzaStore;
impl ChicagoPizzaStore {
    pub fn new() -> Self {
        ChicagoPizzaStore {}
    }
}
impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>> {
        match pizza_type {
            "cheese" => Some(Box::new(ChicagoStyleCheesePizza::new())),
            // "veggie" => Some(Box::new(ChicagoStyleCheesePizza::new())),
            // "clam" => Some(Box::new(ChicagoStyleCheesePizza::new())),
            // "pepperoni" => Some(Box::new(ChicagoStyleCheesePizza::new())),
            _ => None,
        }
    }
}
