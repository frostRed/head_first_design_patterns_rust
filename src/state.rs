//! 状态模式，注意和有限状态机是不一样的
//! 有 2 种类：上下文类（content）有一个属性保存当前状态，其它属性分别是所有的状态类（state）实例
//! state 有一个属性指向上下文类，形成双向树形结构（content 是根节点，state 是叶子节点）
//! content 的所有动作都委托给当前状态，所有 state 都知道触发每个动作时要干什么（改变 content 内容或转移状态）
//! 如果将**状态**和**动作**画成二维表格矩阵，状态模式实质是以状态的视角，将状态和它所有的动作封装在一起

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// 上下文类
#[derive(Clone)]
struct GumballMachine {
    sold_out_state: Rc<dyn State>,
    no_quarter_state: Rc<dyn State>,
    has_quarter_state: Rc<dyn State>,
    sold_state: Rc<dyn State>,

    state: Rc<dyn State>,
    count: usize,
}
impl GumballMachine {
    fn new(number_gumballs: usize) -> Rc<RefCell<Self>> {
        let sold_out_state = Rc::new(SoldOutState::new(None));
        let no_quarter_state = Rc::new(NoQuarterState::new(None));
        let has_quarter_state = Rc::new(HasQuarterState::new(None));
        let sold_state = Rc::new(SoldState::new(None));
        let state = if number_gumballs > 0 {
            no_quarter_state.clone()
        } else {
            sold_out_state.clone() as Rc<dyn State>
        };
        let mut g = GumballMachine {
            sold_out_state,
            no_quarter_state,
            has_quarter_state,
            sold_state,
            state,
            count: number_gumballs,
        };

        let g = Rc::new(RefCell::new(g));
        g.borrow_mut().sold_out_state = Rc::new(SoldOutState::new(Some(Rc::downgrade(&g))));
        g.borrow_mut().no_quarter_state = Rc::new(NoQuarterState::new(Some(Rc::downgrade(&g))));
        g.borrow_mut().has_quarter_state = Rc::new(HasQuarterState::new(Some(Rc::downgrade(&g))));
        g.borrow_mut().sold_state = Rc::new(SoldState::new(Some(Rc::downgrade(&g))));
        let state = if number_gumballs > 0 {
            g.borrow().no_quarter_state.clone()
        } else {
            g.borrow().sold_out_state.clone() as Rc<dyn State>
        };
        g.borrow_mut().state = state;
        g
    }
    fn insert_quarter(&self) {
        self.state.insert_quarter();
    }
    fn eject_quarter(&self) {
        self.state.eject_quarter();
    }
    fn turn_crank(&self) {
        self.state.turn_crank();
        self.state.dispense();
    }

    fn set_state(&mut self, state: Rc<dyn State>) {
        self.state = state;
    }

    fn release_ball(&mut self) {
        println!("A gumball comes rolling out the slot...");
        if self.count != 0 {
            self.count -= 1;
        }
    }
    fn count(&self) -> usize {
        self.count
    }
    fn sold_out_state(&self) -> Rc<dyn State> {
        self.sold_out_state.clone()
    }
    fn no_quarter_state(&self) -> Rc<dyn State> {
        self.no_quarter_state.clone()
    }
    fn has_quarter_state(&self) -> Rc<dyn State> {
        self.has_quarter_state.clone()
    }
    fn sold_state(&self) -> Rc<dyn State> {
        self.sold_state.clone()
    }
}
impl std::fmt::Display for GumballMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.state, self.count)
    }
}

/// 状态
trait State: std::fmt::Display {
    fn insert_quarter(&self);
    fn eject_quarter(&self);
    fn turn_crank(&self);
    fn dispense(&self);
}

struct NoQuarterState {
    gumball_machine: Option<Weak<RefCell<GumballMachine>>>,
}
impl NoQuarterState {
    fn new(gumball_machine: Option<Weak<RefCell<GumballMachine>>>) -> Self {
        NoQuarterState { gumball_machine }
    }
}
impl std::fmt::Display for NoQuarterState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NoQuarterState")
    }
}
impl State for NoQuarterState {
    fn insert_quarter(&self) {
        println!("You inserted a quarter");
        let gumball_machine = self
            .gumball_machine
            .as_ref()
            .expect("a")
            .upgrade()
            .expect("b");
        let state = gumball_machine.borrow().has_quarter_state();
        gumball_machine.borrow_mut().set_state(state);
    }
    fn eject_quarter(&self) {
        println!("You have't inserted a quarter");
    }
    fn turn_crank(&self) {
        println!("You turned, but there's no quarter");
    }
    fn dispense(&self) {
        println!("You need to pay first");
    }
}

struct HasQuarterState {
    gumball_machine: Option<Weak<RefCell<GumballMachine>>>,
}
impl HasQuarterState {
    fn new(gumball_machine: Option<Weak<RefCell<GumballMachine>>>) -> Self {
        HasQuarterState { gumball_machine }
    }
}
impl std::fmt::Display for HasQuarterState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HasQuarterState")
    }
}
impl State for HasQuarterState {
    fn insert_quarter(&self) {
        println!("You cant't insert another quarter");
    }
    fn eject_quarter(&self) {
        println!("Quarter returned");
        let gumball_machine = self.gumball_machine.as_ref().unwrap().upgrade().unwrap();
        let state = gumball_machine.borrow().no_quarter_state();
        gumball_machine.borrow_mut().set_state(state);
    }
    fn turn_crank(&self) {
        println!("You turned...");
        let gumball_machine = self.gumball_machine.as_ref().unwrap().upgrade().unwrap();
        let state = gumball_machine.borrow().sold_state();
        gumball_machine.borrow_mut().set_state(state);
    }
    fn dispense(&self) {
        println!("No gumball dispensed");
    }
}

struct SoldState {
    gumball_machine: Option<Weak<RefCell<GumballMachine>>>,
}
impl SoldState {
    fn new(gumball_machine: Option<Weak<RefCell<GumballMachine>>>) -> Self {
        SoldState { gumball_machine }
    }
}
impl std::fmt::Display for SoldState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SoldState")
    }
}
impl State for SoldState {
    fn insert_quarter(&self) {
        println!("Please wait, we're already giving you gumball");
    }
    fn eject_quarter(&self) {
        println!("Sorry, you already turned the crank");
    }
    fn turn_crank(&self) {
        println!("Turning twice doesn't get you another gumball");
    }
    fn dispense(&self) {
        let gumball_machine = self.gumball_machine.as_ref().unwrap().upgrade().unwrap();
        gumball_machine.borrow_mut().release_ball();
        if gumball_machine.borrow().count() > 0 {
            let state = gumball_machine.borrow().no_quarter_state();
            gumball_machine.borrow_mut().set_state(state);
        } else {
            println!("Oops, out of gumballs!");
            let state = gumball_machine.borrow().sold_out_state();
            gumball_machine.borrow_mut().set_state(state);
        }
    }
}

struct SoldOutState {
    gumball_machine: Option<Weak<RefCell<GumballMachine>>>,
}
impl SoldOutState {
    fn new(gumball_machine: Option<Weak<RefCell<GumballMachine>>>) -> Self {
        SoldOutState { gumball_machine }
    }
}
impl std::fmt::Display for SoldOutState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SoldOutState")
    }
}
impl State for SoldOutState {
    fn insert_quarter(&self) {
        println!("You can't insert quarter, the machine is sold out");
    }
    fn eject_quarter(&self) {
        println!("You can't eject, you haven't inserted a quarter yet");
    }
    fn turn_crank(&self) {
        println!("You turned, but there are no gumballs");
    }
    fn dispense(&self) {
        println!("No gumball dispensed");
    }
}

#[test]
fn test_state() {
    let gumball_machine = GumballMachine::new(5);
    println!("{}", gumball_machine.borrow());

    gumball_machine.borrow().insert_quarter();
    println!("{}", gumball_machine.borrow());
    // gumball_machine.turn_crank();
    // println!("{}", gumball_machine);

    // gumball_machine.insert_quarter();
    // gumball_machine.eject_quarter();
    // gumball_machine.turn_crank();
    // println!("{}", gumball_machine);

    // gumball_machine.insert_quarter();
    // gumball_machine.turn_crank();
    // gumball_machine.insert_quarter();
    // gumball_machine.turn_crank();
    // gumball_machine.eject_quarter();
    // println!("{}", gumball_machine);

    // gumball_machine.insert_quarter();
    // gumball_machine.insert_quarter();
    // gumball_machine.turn_crank();
    // gumball_machine.insert_quarter();
    // gumball_machine.turn_crank();
    // gumball_machine.insert_quarter();
    // gumball_machine.turn_crank();
    // println!("{}", gumball_machine);
}
