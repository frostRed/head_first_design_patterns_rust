//! 适配器模式，用来将旧的接口转化为新的期望的接口
//! 被适配者和调用适配器的客户是解耦的，它们互相不知道对方
//! 适配器模式和装饰者模式的区别：装饰者是在原有的基础上添加新的行为或职责；适配器是包装原有的接口使之满足新接口的期望，它们意图不同

pub trait Duck {
    fn quack(&self);
    fn fly(&self);
}

struct MallardDuck;
impl Duck for MallardDuck {
    fn quack(&self) {
        println!("Quack");
    }
    fn fly(&self) {
        println!("I'm flying");
    }
}

pub trait Turkey {
    fn gobble(&self);
    fn fly(&self);
}
struct WildTurkey;
impl Turkey for WildTurkey {
    fn gobble(&self) {
        println!("Gobble gobble");
    }
    fn fly(&self) {
        println!("I'm flying a short distance");
    }
}

struct TurkeyAdapter {
    turkey: Box<dyn Turkey>,
}
impl Duck for TurkeyAdapter {
    fn quack(&self) {
        self.turkey.gobble();
    }
    fn fly(&self) {
        for _ in 0..5 {
            self.turkey.fly();
        }
    }
}

#[test]
fn teset_turkey_adapter() {
    let duck = MallardDuck {};
    let turkey = WildTurkey {};
    let turkey_adapter = TurkeyAdapter {
        turkey: Box::new(WildTurkey {}),
    };

    println!("The Turkey says...");
    turkey.gobble();
    turkey.fly();

    fn test_duct(duck: Box<dyn Duck>) {
        duck.quack();
        duck.fly();
    }

    println!("\nThe Duck says...");
    test_duct(Box::new(duck));

    println!("\nThe TurkeyAdapter says...");
    test_duct(Box::new(turkey_adapter));
}
