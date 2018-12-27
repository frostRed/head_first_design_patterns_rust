//! 观察者订阅某个主题，主题对象管理某些数据，一旦主题内的数据改变，就会通知观察者或者把行数据送到观察者手上

pub trait Subject<T>
where
    T: Observer,
{
    fn register_observer(&mut self, o: std::rc::Rc<std::cell::RefCell<T>>);
    fn remove_observer(
        &mut self,
        o: std::rc::Rc<std::cell::RefCell<T>>,
    ) -> Option<std::rc::Rc<std::cell::RefCell<T>>>;
    fn notify_observers(&mut self);
}

pub struct WeatherDate<T: Observer> {
    // 不能用 HashSet，因为它不能 for i in &mut hashset
    // 存的也是RecCell<T>不能是T
    observers: Vec<std::rc::Rc<std::cell::RefCell<T>>>,
    temperature: f64,
    humidity: f64,
    pressure: f64,
}

impl<O> WeatherDate<O>
where
    O: Observer,
{
    pub fn new() -> Self {
        WeatherDate {
            observers: Vec::new(),
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
        }
    }

    pub fn set_musurements(&mut self, temprature: f64, humidity: f64, pressure: f64) {
        self.temperature = temprature;
        self.humidity = humidity;
        self.pressure = pressure;
        // todo 不能调用 trait 里面的方法
        // self.notify_observers();
    }
}

impl<T> Subject<T> for WeatherDate<T>
where
    T: Observer + std::cmp::PartialEq,
{
    fn register_observer(&mut self, o: std::rc::Rc<std::cell::RefCell<T>>) {
        self.observers.push(o);
    }
    fn remove_observer(
        &mut self,
        o: std::rc::Rc<std::cell::RefCell<T>>,
    ) -> Option<std::rc::Rc<std::cell::RefCell<T>>> {
        self.observers.remove_item(&o)
    }
    fn notify_observers(&mut self) {
        for i in &self.observers {
            i.borrow_mut()
                .update(self.temperature, self.humidity, self.pressure);
        }
    }
}

pub trait Observer {
    fn update(&mut self, temprature: f64, humidity: f64, pressure: f64);
}

/// 观察者
pub struct CurrentConditionsDisplay {
    temperature: f64,
    humidity: f64,
    weather_data: std::rc::Weak<std::cell::RefCell<WeatherDate<Self>>>,
}
impl PartialEq for CurrentConditionsDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.temperature == other.temperature && self.humidity == other.humidity
    }
}

impl CurrentConditionsDisplay {
    pub fn new(
        weather_data: std::rc::Rc<std::cell::RefCell<WeatherDate<Self>>>,
    ) -> std::rc::Rc<std::cell::RefCell<Self>> {
        // 这里有循环引用，一对多，一里面存的是多的 Rc，多存的是一的 Weak
        let s = CurrentConditionsDisplay {
            temperature: 0.0,
            humidity: 0.0,
            weather_data: std::rc::Rc::downgrade(&weather_data),
        };
        let p = std::rc::Rc::new(std::cell::RefCell::new(s));
        weather_data.borrow_mut().register_observer(p.clone());
        return p;
    }
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temperature: f64, humidity: f64, _: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        println!("{}", self)
    }
}
impl std::fmt::Display for CurrentConditionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Current conditions: {} F degrees and {} % humidity",
            self.temperature, self.humidity
        )
    }
}
