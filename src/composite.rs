//! 组合模式，用来构建树形结构，并且统一看待处理叶子节点和非叶子节点
//! 关键在于有一个抽象类，给叶子类和非叶子类的所有方法提供默认实现
//! 非叶子节点类是一个递归结构，有一个属性是数组，元素类型是抽象类
//! 这样非叶子节点能包含下级非叶子节点或者叶子节点

use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone)]
enum MenuComponentErr {
    UnSupportedOperation,
    NoChild,
}
impl Display for MenuComponentErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MenuComponentErr::UnSupportedOperation => write!(f, "UnSupportedOperation"),
            MenuComponentErr::NoChild => write!(f, "NoChild"),
        }
    }
}
impl Error for MenuComponentErr {}

trait MenuComponent {
    fn add(&mut self, menu_component: Rc<dyn MenuComponent>) -> Result<(), MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    fn remove(&mut self, menu_component: Rc<dyn MenuComponent>) -> Result<(), MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    fn child(&self, i: usize) -> Result<Rc<dyn MenuComponent>, MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    fn name(&self) -> Result<String, MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    fn description(&self) -> Result<String, MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    fn price(&self) -> Result<f64, MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    fn is_vegetarian(&self) -> Result<bool, MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    fn print(&self) -> Result<(), MenuComponentErr> {
        Err(MenuComponentErr::UnSupportedOperation)
    }
    // fn eq(&self, others: Rc<dyn MenuComponent>) -> bool;
}

// 似乎 trait 方法里面有参数自身类型的 trait object，就不能为 trait object  实现 PartialEq
// impl PartialEq for MenuComponent {
//     fn eq(&self, other: &MenuComponent) -> bool {
//         self.eq(other)
//     }
// }

#[derive(PartialEq)]
struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f64,
}
impl MenuItem {
    fn new(name: String, description: String, vegetarian: bool, price: f64) -> Self {
        MenuItem {
            name,
            description,
            vegetarian,
            price,
        }
    }
}
impl MenuComponent for MenuItem {
    fn name(&self) -> Result<String, MenuComponentErr> {
        Ok(self.name.clone())
    }
    fn description(&self) -> Result<String, MenuComponentErr> {
        Ok(self.description.clone())
    }
    fn price(&self) -> Result<f64, MenuComponentErr> {
        Ok(self.price)
    }
    fn is_vegetarian(&self) -> Result<bool, MenuComponentErr> {
        Ok(self.vegetarian)
    }
    fn print(&self) -> Result<(), MenuComponentErr> {
        print!("{}", self.name);
        if self.vegetarian {
            print!("(v)");
        }
        println!(", {}    -- {}", self.price, self.description);
        Ok(())
    }
}

struct Menu {
    menu_components: Option<Vec<Rc<dyn MenuComponent>>>,
    name: String,
    description: String,
}

impl Menu {
    fn new(name: String, description: String) -> Self {
        Menu {
            menu_components: None,
            name,
            description,
        }
    }
}

impl MenuComponent for Menu {
    fn add(&mut self, menu_component: Rc<dyn MenuComponent>) -> Result<(), MenuComponentErr> {
        match self.menu_components {
            None => {
                self.menu_components = Some(vec![menu_component]);
            }
            Some(ref mut v) => v.push(menu_component),
        }
        Ok(())
    }
    fn remove(&mut self, menu_component: Rc<dyn MenuComponent>) -> Result<(), MenuComponentErr> {
        if let Some(ref mut v) = self.menu_components {
            // v.remove_item(menu_component);
            unimplemented!()
        }
        Ok(())
    }
    fn child(&self, i: usize) -> Result<Rc<dyn MenuComponent>, MenuComponentErr> {
        if let Some(ref v) = self.menu_components {
            return Ok(v[i].clone());
        }
        Err(MenuComponentErr::NoChild)
    }

    fn name(&self) -> Result<String, MenuComponentErr> {
        Ok(self.name.clone())
    }
    fn description(&self) -> Result<String, MenuComponentErr> {
        Ok(self.description.clone())
    }
    fn print(&self) -> Result<(), MenuComponentErr> {
        println!("\n{}, {}\n-----------------", self.name, self.description);
        if let Some(ref v) = self.menu_components {
            for i in v {
                i.print()?
            }
        }
        Ok(())
    }
}

/// 使用者
struct Waitress {
    all_menus: Rc<dyn MenuComponent>,
}

impl Waitress {
    fn new(all_menus: Rc<dyn MenuComponent>) -> Self {
        Waitress { all_menus }
    }
    fn print_menu(&self) {
        self.all_menus.print();
    }
}

#[test]
fn test_composite() {
    let pancake_house_menu = Menu::new("PANCAKE HOUSE MENU".to_string(), "Breakfast".to_string());
    let mut diner_menu = Menu::new("DINER MENU".to_string(), "Launch".to_string());
    diner_menu
        .add(Rc::new(MenuItem::new(
            "Pasta".to_string(),
            "Spaghetti with Marinara Sauce, and a slice of sourdough bread".to_string(),
            true,
            3.89,
        )))
        .unwrap();
    let mut all_menus = Menu::new("ALL MENUS".to_string(), "All menus combined".to_string());
    all_menus.add(Rc::new(pancake_house_menu)).unwrap();
    all_menus.add(Rc::new(diner_menu)).unwrap();
    all_menus.print().unwrap();
}
