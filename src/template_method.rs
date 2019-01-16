//! 模板方法模式，特征在于抽象类里面有一个不允许重写的算法框架方法，这个方法会使用自己的其它方法
//! 这些其它方法分为 3 种：不允许子类重写、子类必须重写、子类可选地重写
//! 其中可选重写的方法被称为「钩子」，这就给子类带来了弹性

use std::io::{self, Read};

pub trait CaffeineBeverageWithHook {
    fn prepare_recipe(&mut self) {
        self.boil_water();
        self.brew();
        self.pour_in_cup();
        if self.customer_wants_condiments() {
            self.add_condiments();
        }
    }
    fn brew(&mut self);
    fn add_condiments(&mut self);
    fn boil_water(&mut self) {
        println!("Boiling water");
    }
    fn pour_in_cup(&mut self) {
        println!("Pouring into cup");
    }
    fn customer_wants_condiments(&mut self) -> bool {
        true
    }
}

struct CoffeeWithHook;
impl CaffeineBeverageWithHook for CoffeeWithHook {
    fn brew(&mut self) {
        println!("Dripping Coffee through filter");
    }
    fn add_condiments(&mut self) {
        println!("Adding Sugar and Milk");
    }

    fn customer_wants_condiments(&mut self) -> bool {
        let answer: String = self.get_user_input();
        if answer.to_lowercase().starts_with("y") {
            true
        } else {
            false
        }
    }
}

impl CoffeeWithHook {
    fn get_user_input(&self) -> String {
        print!("Would you like milk and sugar with you coffee (y/n)");

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Err(_) => panic!("IO error trying to read your anser"),
            Ok(i) => {
                if i == 0 {
                    "no".to_string()
                } else {
                    buffer
                }
            }
        }
    }
}

#[test]
fn test_coffee() {
    let mut coffee = CoffeeWithHook {};
    println!("\nMaking coffee...");
    coffee.prepare_recipe();
}
