//! 策略模式
//! 它的特征是有一个抽象基类，抽象基类里面的属性是接口，抽象基类方法行为委托给这些接口值的方法
//! 具体类继承抽象基类，然后传入不同的接口值就能拥有不同的行为，而且有方法能改变里面的接口值
use std::cell::RefCell;
use std::rc::Rc;

pub trait QuackBehavior {
    fn quack(&self);
}

pub trait FlyBehavior {
    fn fly(&self);
}

/// Duck 抽象 trait
pub trait Duck {
    fn display(&self);

    fn quack_behavior(&self) -> Rc<RefCell<Box<QuackBehavior>>>;
    fn perform_quack(&self) {
        self.quack_behavior().borrow().quack()
    }

    fn fly_behavior(&self) -> Rc<RefCell<Box<FlyBehavior>>>;
    fn perform_fly(&self) {
        self.fly_behavior().borrow().fly()
    }

    fn set_fly_behavior(&self, fly_behavior: Box<FlyBehavior>) {
        let s = self.fly_behavior();
        s.replace(fly_behavior);
    }

    fn swim(&self) {
        println!("All ducks float, even decoys!");
    }
}

pub struct FlyWithWings;
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("I'm flying");
    }
}

pub struct FlyNoWay;
impl FlyBehavior for FlyNoWay {
    fn fly(&self) {
        println!("I can't fly");
    }
}

pub struct FlyRocketPowered;
impl FlyBehavior for FlyRocketPowered {
    fn fly(&self) {
        println!("I'm flying with a rocket!");
    }
}

struct Quack;
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("Quack");
    }
}

struct MuteQuack;
impl QuackBehavior for MuteQuack {
    fn quack(&self) {
        println!("<< Silence >>");
    }
}

struct Squeak;
impl QuackBehavior for Squeak {
    fn quack(&self) {
        println!("Squeak");
    }
}

//////////////////////////////////
pub struct MallardDuck {
    quack_behavior: Rc<RefCell<Box<QuackBehavior>>>,
    fly_behavior: Rc<RefCell<Box<FlyBehavior>>>,
}

impl MallardDuck {
    pub fn new() -> Self {
        // 为了这个不需参数的构造函数，trait object 不能是局部变量，不能是 Box
        let fly = Rc::new(RefCell::new(Box::new(FlyWithWings {}) as Box<FlyBehavior>));
        let quack = Rc::new(RefCell::new(Box::new(Quack {}) as Box<QuackBehavior>));
        MallardDuck {
            fly_behavior: fly.clone(),
            quack_behavior: quack.clone(),
        }
    }
}

impl Duck for MallardDuck {
    fn display(&self) {
        println!("I'm a real Mallard duck");
    }
    fn quack_behavior(&self) -> Rc<RefCell<Box<QuackBehavior>>> {
        self.quack_behavior.clone()
    }
    fn fly_behavior(&self) -> Rc<RefCell<Box<FlyBehavior>>> {
        self.fly_behavior.clone()
    }
}

////////////////
pub struct ModelDuck {
    quack_behavior: Rc<RefCell<Box<QuackBehavior>>>,
    fly_behavior: Rc<RefCell<Box<FlyBehavior>>>,
}

impl ModelDuck {
    pub fn new() -> Self {
        let fly = Rc::new(RefCell::new(Box::new(FlyNoWay {}) as Box<FlyBehavior>));
        let quack = Rc::new(RefCell::new(Box::new(Quack {}) as Box<QuackBehavior>));
        ModelDuck {
            fly_behavior: fly,
            quack_behavior: quack,
        }
    }
}
impl Duck for ModelDuck {
    fn display(&self) {
        println!("I'm a Model duck");
    }
    fn quack_behavior(&self) -> Rc<RefCell<Box<QuackBehavior>>> {
        self.quack_behavior.clone()
    }
    fn fly_behavior(&self) -> Rc<RefCell<Box<FlyBehavior>>> {
        self.fly_behavior.clone()
    }
}
