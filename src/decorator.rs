//! 装饰者模式，有 2 种类：组件和装饰者。本质是装饰者类形成了自指，继承自某个抽象基类，里面又存在这个抽象基类类型的属性
//! 特征：所有组件和装饰者都继承自一个抽象基类或接口，然后装饰者类有一个属性，属性的类型就是抽象基类
//! 这样这个属性的值就能是其它组件实例，这个装饰者就形成了对组件的一层包装。
//! 并且被包装组件的行为总是可以委托给那个抽象基类属性来调用,装饰者附加的行为可以在委托之前或之后进行
//! 装饰者可以随意包装其它装饰者，装饰者一层套一层

/// 组件抽象
pub trait Beverage {
    fn description(&self) -> String;
    fn cost(&self) -> f64;
}

/// 装饰者抽象
pub trait CondimentDecorator: Beverage {}

/// 具体组件
pub struct Espresso {
    description: String,
}
impl Espresso {
    pub fn new() -> Self {
        Espresso {
            description: "Espresso".to_string(),
        }
    }
}
impl Beverage for Espresso {
    fn description(&self) -> String {
        self.description.clone()
    }
    fn cost(&self) -> f64 {
        1.99
    }
}

pub struct HouseBlend {
    description: String,
}
impl HouseBlend {
    pub fn new() -> Self {
        HouseBlend {
            description: "House Blend Coffee".to_string(),
        }
    }
}
impl Beverage for HouseBlend {
    fn description(&self) -> String {
        self.description.clone()
    }
    fn cost(&self) -> f64 {
        0.89
    }
}

pub struct DarkRost {
    description: String,
}
impl DarkRost {
    pub fn new() -> Self {
        DarkRost {
            description: "DarkRost Coffee".to_string(),
        }
    }
}
impl Beverage for DarkRost {
    fn description(&self) -> String {
        self.description.clone()
    }
    fn cost(&self) -> f64 {
        0.99
    }
}

pub struct Decat {
    description: String,
}
impl Decat {
    pub fn new() -> Self {
        Decat {
            description: "Decat Coffee".to_string(),
        }
    }
}
impl Beverage for Decat {
    fn description(&self) -> String {
        self.description.clone()
    }
    fn cost(&self) -> f64 {
        1.05
    }
}

/// 装饰者
pub struct Mocha {
    beverage: Box<dyn Beverage>,
}
impl Mocha {
    pub fn new(b: Box<dyn Beverage>) -> Self {
        Mocha { beverage: b }
    }
}
impl Beverage for Mocha {
    fn description(&self) -> String {
        self.beverage.description() + ", Mocha"
    }
    fn cost(&self) -> f64 {
        0.2 + self.beverage.cost()
    }
}
impl CondimentDecorator for Mocha {}

pub struct Soy {
    beverage: Box<dyn Beverage>,
}
impl Soy {
    pub fn new(b: Box<dyn Beverage>) -> Self {
        Soy { beverage: b }
    }
}
impl Beverage for Soy {
    fn description(&self) -> String {
        self.beverage.description() + ", Soy"
    }
    fn cost(&self) -> f64 {
        0.15 + self.beverage.cost()
    }
}
impl CondimentDecorator for Soy {}

pub struct Whip {
    beverage: Box<dyn Beverage>,
}
impl Whip {
    pub fn new(b: Box<dyn Beverage>) -> Self {
        Whip { beverage: b }
    }
}
impl Beverage for Whip {
    fn description(&self) -> String {
        self.beverage.description() + ", Whip"
    }
    fn cost(&self) -> f64 {
        0.1 + self.beverage.cost()
    }
}
impl CondimentDecorator for Whip {}
