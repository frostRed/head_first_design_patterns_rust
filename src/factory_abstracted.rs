//! 抽象工厂模式，是最一般的工厂模式，它与工厂方法最大的不同是，工厂制作一个产品需要一系列「产品的组件」，不同的工厂可以组装不同组件来生成不同的产品
//! 所以添加新的工厂是容易的，但产品中添加新组件是困难的
//!

// 面团
pub trait Dough {}
struct ThinCrustDough;
struct ThickCrustDough;
impl Dough for ThinCrustDough {}
impl Dough for ThickCrustDough {}

// 酱汁
pub trait Sauce {}
struct MarinaraSauce;
struct PlumTomatoSauce;
impl Sauce for MarinaraSauce {}
impl Sauce for PlumTomatoSauce {}

// 芝士
pub trait Cheese {}
struct ReggianoCheese;
impl Cheese for ReggianoCheese {}

// 蔬菜
pub trait Veggy {}
struct Garlic;
impl Veggy for Garlic {}
struct Onion;
impl Veggy for Onion {}
struct Mushroom;
impl Veggy for Mushroom {}
struct RedPepper;
impl Veggy for RedPepper {}

// 胡椒
pub trait Pepper {}
struct SlicedPepper;
impl Pepper for SlicedPepper {}

// 蛤蜊
pub trait Clam {}
struct FreshClams;
struct FrozenClams;
impl Clam for FreshClams {}
impl Clam for FrozenClams {}

// 批萨原料工厂
pub trait PizzaIngredientFactory: Clone {
    fn create_dough(&self) -> Box<dyn Dough>;
    fn create_sauce(&self) -> Box<dyn Sauce>;
    fn create_cheese(&self) -> Box<dyn Cheese>;
    fn create_veggies(&self) -> Vec<Box<dyn Veggy>>;
    fn create_pepperoni(&self) -> Box<dyn Pepper>;
    fn create_clam(&self) -> Box<dyn Clam>;
}

#[derive(Clone)]
struct NYPizzaIngredientFactory;
impl PizzaIngredientFactory for NYPizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThinCrustDough {})
    }
    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(MarinaraSauce {})
    }
    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(ReggianoCheese {})
    }
    fn create_veggies(&self) -> Vec<Box<dyn Veggy>> {
        vec![
            Box::new(Garlic {}),
            Box::new(Onion {}),
            Box::new(Mushroom {}),
            Box::new(RedPepper {}),
        ]
    }
    fn create_pepperoni(&self) -> Box<dyn Pepper> {
        Box::new(SlicedPepper {})
    }
    fn create_clam(&self) -> Box<dyn Clam> {
        Box::new(FreshClams {})
    }
}
#[derive(Clone)]
struct ChicagoPizzaIngredientFactory;
impl PizzaIngredientFactory for ChicagoPizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThickCrustDough {})
    }
    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(PlumTomatoSauce {})
    }
    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(ReggianoCheese {})
    }
    fn create_veggies(&self) -> Vec<Box<dyn Veggy>> {
        vec![
            Box::new(Garlic {}),
            Box::new(Onion {}),
            Box::new(Mushroom {}),
            Box::new(RedPepper {}),
        ]
    }
    fn create_pepperoni(&self) -> Box<dyn Pepper> {
        Box::new(SlicedPepper {})
    }
    fn create_clam(&self) -> Box<dyn Clam> {
        Box::new(FrozenClams {})
    }
}

/// 物品抽象
pub trait Pizza {
    fn prepare(&mut self);
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
    fn set_name(&mut self, name: &str);
}

/// 具体物品
pub struct CheesePizza<F: PizzaIngredientFactory> {
    name: String,
    dough: Option<Box<dyn Dough>>,
    sauce: Option<Box<dyn Sauce>>,
    veggies: Option<Vec<Box<dyn Veggy>>>,
    cheese: Option<Box<dyn Cheese>>,
    pepperoni: Option<Box<dyn Pepper>>,
    clam: Option<Box<dyn Clam>>,
    ingredient_factory: F,
}
impl<F: PizzaIngredientFactory> CheesePizza<F> {
    fn new(f: F) -> Self {
        CheesePizza {
            name: "Cheese Pizza".to_string(),
            dough: None,
            sauce: None,
            veggies: None,
            cheese: None,
            pepperoni: None,
            clam: None,
            ingredient_factory: f,
        }
    }
}
impl<F: PizzaIngredientFactory> Pizza for CheesePizza<F> {
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn prepare(&mut self) {
        println!("peparing {}", self.name());
        self.dough = Some(self.ingredient_factory.create_dough());
        self.sauce = Some(self.ingredient_factory.create_sauce());
        self.cheese = Some(self.ingredient_factory.create_cheese());
    }
}

// pub struct ChicagoStyleCheesePizza {
//     name: String,
//     dough: String,
//     sauce: String,
//     toppings: Vec<String>,
// }
// impl ChicagoStyleCheesePizza {
//     fn new() -> Self {
//         ChicagoStyleCheesePizza {
//             name: "Chicago Style Deep Dish Cheese Pizza".to_string(),
//             dough: "Extra Thick Crust Dough".to_string(),
//             sauce: "Plum Tomato Sauce".to_string(),
//             toppings: vec!["Shredded Mozzarella Cheese".to_string()],
//         }
//     }
// }
// impl Pizza for ChicagoStyleCheesePizza {
//     fn name(&self) -> &str {
//         return &self.name;
//     }
//     fn prepare(&self) {
//         println!("Preparing {}", self.name);
//         println!("Tossing dough...");
//         println!("Adding sauch...");
//         println!("Adding toppings: ");
//         for i in &self.toppings {
//             println!("    {}", i);
//         }
//     }
//     fn cut(&self) {
//         println!("Cutting the pizza into square slices");
//     }
// }

/// 工厂抽象
pub trait PizzaStore {
    fn order_pizza(&self, pizza_type: String) -> Box<dyn Pizza> {
        let mut pizza = self.create_pizza(pizza_type).expect("Unknown pizza type");
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_();
        pizza
    }
    fn create_pizza(&self, pizza_type: String) -> Option<Box<dyn Pizza>>;
}

// 具体工厂
pub struct NYPizzaStore<F> where F: PizzaIngredientFactory + 'a {
    ingredient_factory: F,
}

impl<F> NYPizzaStore<F> where F: PizzaIngredientFactory + 'a{
    pub fn new(f: F) -> Self {
        NYPizzaStore {
            ingredient_factory: f,
        }
    }
}
impl<F: PizzaIngredientFactory> PizzaStore for NYPizzaStore<F> {
    fn create_pizza(&self, pizza_type: String) -> Option<Box<dyn Pizza>> {
        match pizza_type.as_ref() {
            "cheese" => Some(Box::new(CheesePizza::new(self.ingredient_factory.clone()))),
            // "veggie" => Some(Box::new(NYStyleCheesePizza::new())),
            // "clam" => Some(Box::new(NYStyleCheesePizza::new())),
            // "pepperoni" => Some(Box::new(NYStyleCheesePizza::new())),
            _ => None,
        }
    }
}

//pub struct ChicagoPizzaStore<F: PizzaIngredientFactory> {
//    ingredient_factory: F,
//}
//
//impl<F> ChicagoPizzaStore<F>
//where
//    F: PizzaIngredientFactory,
//{
//    pub fn new(f: F) -> Self {
//        ChicagoPizzaStore {
//            ingredient_factory: f,
//        }
//    }
//}
//impl<F: PizzaIngredientFactory> PizzaStore for ChicagoPizzaStore<F> {
//    fn create_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>> {
//        match pizza_type {
//            // "cheese" => Some(Box::new(ChicagoStyleCheesePizza::new())),
//            // "veggie" => Some(Box::new(ChicagoStyleCheesePizza::new())),
//            // "clam" => Some(Box::new(ChicagoStyleCheesePizza::new())),
//            // "pepperoni" => Some(Box::new(ChicagoStyleCheesePizza::new())),
//            _ => None,
//        }
//    }
//}
