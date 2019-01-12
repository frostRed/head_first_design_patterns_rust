//! 单例模式，特征在于构造函数是私有的，结构内有一个私有静态变量来保存自身的唯一实例
//! 通过某个方法来获取这个实例
//! 多线程时要么在一开始就初始化一次（这会丧失单例模式相对于全局变量，延迟初始化的优势）
//! 要么获取变量时先同步（double-checked locking，实例不存在，加锁，检查实例是否存在，不存在则初始化一个）
//! rust 一般用 lazy_static! 实现单例，全局可变要用锁

use crate::lazy_static::lazy_static;
use std::sync::RwLock;

pub struct ChocolateBoiler {
    empty: bool,
    boiled: bool,
}
impl ChocolateBoiler {
    pub fn new() -> Self {
        ChocolateBoiler {
            empty: true,
            boiled: false,
        }
    }

    pub fn fill(&mut self) {
        if self.is_empty() {
            self.empty = false;
            self.boiled = false;
            // 添加巧克力和牛奶
        }
    }
    pub fn drain(&mut self) {
        if !self.is_empty() && self.is_boiled() {
            self.empty = true;
        }
    }

    pub fn boil(&mut self) {
        if !self.is_empty() && !self.is_boiled() {
            self.boiled = true;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }
    pub fn is_boiled(&self) -> bool {
        self.boiled
    }
}

lazy_static! {
    pub static ref C: RwLock<ChocolateBoiler> = { RwLock::new(ChocolateBoiler::new()) };
}
