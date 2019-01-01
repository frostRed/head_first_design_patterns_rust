#![feature(vec_remove_item)]

pub mod observer;
// pub mod observer_pull;
pub mod decorator;
pub mod factory_method;
pub mod strategy;

use crate::decorator::{Beverage, DarkRost, Espresso, HouseBlend, Mocha, Soy, Whip};
use crate::factory_method::{ChicagoPizzaStore, NYPizzaStore, Pizza, PizzaStore};
use crate::observer::{CurrentConditionsDisplay, Subject, WeatherDate};
use crate::strategy::{Duck, FlyBehavior, FlyRocketPowered, MallardDuck, ModelDuck};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mallard_duck() {
        let mallard = MallardDuck::new();
        mallard.perform_quack();
        mallard.perform_fly();
        println!("\n");

        let model = ModelDuck::new();
        model.perform_fly();
        let r = Box::new(FlyRocketPowered);
        model.set_fly_behavior(r);
        model.perform_fly();
    }

    #[test]
    fn test_observer() {
        let weather_data = std::rc::Rc::new(std::cell::RefCell::new(WeatherDate::new()));
        let _ = CurrentConditionsDisplay::new(weather_data.clone());

        weather_data.borrow_mut().set_musurements(80.0, 65.0, 30.4);
        weather_data.borrow_mut().notify_observers();
        weather_data.borrow_mut().set_musurements(82.0, 70.0, 29.2);
        weather_data.borrow_mut().notify_observers();
        weather_data.borrow_mut().set_musurements(78.0, 90.0, 29.2);
        weather_data.borrow_mut().notify_observers();
    }

    #[test]
    fn test_decorator() {
        let beravage = Espresso::new();
        println!("{} ${}", beravage.description(), beravage.cost());

        let beravage2 = DarkRost::new();
        let beravage2 = Mocha::new(Box::new(beravage2));
        let beravage2 = Mocha::new(Box::new(beravage2));
        let beravage2 = Whip::new(Box::new(beravage2));
        println!("{} ${}", beravage2.description(), beravage2.cost());

        let beravage3 = HouseBlend::new();
        let beravage3 = Soy::new(Box::new(beravage3));
        let beravage3 = Mocha::new(Box::new(beravage3));
        let beravage3 = Whip::new(Box::new(beravage3));
        println!("{} ${}", beravage3.description(), beravage3.cost());
    }

    #[test]
    fn test_factory_method() {
        let ny_store = NYPizzaStore::new();
        let chicago_store = ChicagoPizzaStore::new();

        let pizza = ny_store.order_pizza("cheese");
        println!("Ethan ordered a {}\n", pizza.name());

        let pizza = chicago_store.order_pizza("cheese");
        println!("Joel ordered a {}", pizza.name())
    }
}
