//! 迭代器模式，主要为了提供一致的访问，不用关系集合的实现细节，将在元素之间游走的职责规定给迭代器
//! Rust 的迭代器就是用的这个模式，有四种类。抽象集合类（IntoIterator），有一个方法（into_iter）能返回抽象迭代器类（Iterator）
//! 抽象迭代器类有 next 等方法。具体集合（容器）则有方法返回具体迭代器（一般有 2 个属性，容器 + 索引）
//! 为了保持单一职责再将具体集合和对应的具体迭代器拆成 2 个类，一个类应该只有一个引起改变的原因

#[derive(Clone, Debug)]
struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f64,
}
impl MenuItem {
    fn new(name: &str, description: &str, vegetarian: bool, price: f64) -> Self {
        MenuItem {
            name: name.to_string(),
            description: description.to_string(),
            vegetarian,
            price,
        }
    }
}

/// 集合类
struct PancakeHouseMenu {
    menu_items: Vec<MenuItem>,
}
impl PancakeHouseMenu {
    fn new() -> Self {
        let mut menu_items = Vec::new();
        menu_items.push(MenuItem::new(
            "K&B's Pancake Breakfast",
            "Pancakes with scrambled eggs, and toast",
            true,
            2.99,
        ));
        menu_items.push(MenuItem::new(
            "Regular Pancake Breakfast",
            "Pancakes with fried eggs, sausage",
            false,
            2.99,
        ));
        menu_items.push(MenuItem::new(
            "Blueberry Pancake Breakfast",
            "Pancakes with fresh blueberries or strawberries",
            true,
            3.49,
        ));
        menu_items.push(MenuItem::new(
            "Waffles",
            "Waffles, with your choice of blueberries or strawberries",
            true,
            3.59,
        ));

        PancakeHouseMenu { menu_items }
    }
    fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f64) {
        self.menu_items
            .push(MenuItem::new(name, description, vegetarian, price));
    }
    fn menu_items(&self) -> &[MenuItem] {
        &self.menu_items
    }
}

/// Rust 不支持结构内静态属性
const MAX_ITEMS: usize = 6;
struct DinerMenu {
    number_of_items: usize,
    menu_items: [MenuItem; MAX_ITEMS],
}
impl DinerMenu {
    fn new() -> Self {
        let menu_items = [
            MenuItem::new(
                "Vegetarian BLT",
                "(Fakin') Bacon with lettuce & tomato on whole wheat",
                true,
                2.99,
            ),
            MenuItem::new(
                "BLT",
                "Bacon with lettuce & tomato on whole wheat",
                false,
                2.99,
            ),
            MenuItem::new(
                "Soup of the day",
                "Soup of the day, with a side of potato salad",
                false,
                3.29,
            ),
            MenuItem::new(
                "Hotdog",
                "A hot dog, with saurkraut, relish, onions, topped with cheese",
                false,
                3.05,
            ),
            MenuItem::new(
                "Steamed Veggies and Brown Rice",
                "Steamed vegetables over brown rice",
                true,
                3.99,
            ),
            MenuItem::new(
                "Pasta",
                "Spaghetti with Marinara Sauce, and a slice of sourdough bread",
                true,
                3.89,
            ),
        ];
        DinerMenu {
            number_of_items: MAX_ITEMS,
            menu_items,
        }
    }
    fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f64) {
        let menu_item = MenuItem::new(name, description, vegetarian, price);
        if self.number_of_items >= MAX_ITEMS {
            eprintln!("Sorry, menu is full! Can't add item to menu");
        } else {
            self.menu_items[self.number_of_items] = menu_item;
            self.number_of_items += 1;
        }
    }
}

/// 迭代器类
struct MenuIterator<'a> {
    position: usize,
    items: &'a [MenuItem],
}

impl<'a> Iterator for MenuIterator<'a> {
    type Item = &'a MenuItem;
    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.items.len() {
            self.position += 1;
            Some(&self.items[self.position - 1])
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a DinerMenu {
    type Item = &'a MenuItem;
    type IntoIter = MenuIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MenuIterator {
            position: 0,
            items: &self.menu_items,
        }
    }
}

impl<'a> IntoIterator for &'a PancakeHouseMenu {
    type Item = &'a MenuItem;
    type IntoIter = MenuIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MenuIterator {
            position: 0,
            items: &self.menu_items,
        }
    }
}

struct Waitress {
    pancake_house_menu: PancakeHouseMenu,
    diner_menu: DinerMenu,
}
impl Waitress {
    fn new(pancake_house_menu: PancakeHouseMenu, diner_menu: DinerMenu) -> Self {
        Waitress {
            pancake_house_menu,
            diner_menu,
        }
    }
    fn print_menu(&self) {
        println!("MENU\n----BREAKFAST");
        print(&self.pancake_house_menu);
        println!("MENU\n----LUNCH");
        print(&self.diner_menu);
    }
}

fn print<'a>(ite: impl IntoIterator<Item = &'a MenuItem, IntoIter = MenuIterator<'a>>) {
    for i in ite {
        println!("{}, {} -- {}", i.name, i.price, i.description);
    }
}

#[test]
fn test_iterator() {
    let pancake_house_menu = PancakeHouseMenu::new();
    let diner_menu = DinerMenu::new();
    let waitress = Waitress::new(pancake_house_menu, diner_menu);
    waitress.print_menu();
}
