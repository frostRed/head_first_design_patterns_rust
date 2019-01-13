//! 外观模式，将一系列复杂的子系统隐藏在背后，提供一个简化的接口，同时如果需要仍然可以使用所有子系统的接口
//! 将客户和子系统解耦
//! 根据最少知识原则，在对象的方法内部，只能使用这些方法：
//! + 该对象本身
//! + 当作方法参数传进来的其它对象
//! + 该方法内部创建或实例化的对象
//! + 该对象的组件
//! **不能使用调用其它方法返回的对象的方法**

struct Amplifier;
impl Amplifier {
    fn on(&self) {
        println!("Top-O-Line Amplifier on");
    }
    fn set_dvd(&self, dvd: &DvdPlayer) {
        println!("Top-O-Line Amplifier setting DVD player to Top-O-Line DVD Player");
    }
    fn set_surround_sound(&self) {
        println!("Top-O-Line Amplifier surround sound on");
    }
    fn set_volume(&self, volume: usize) {
        println!("Top-O-Line Amplifier setting volume to {}", volume);
    }
    fn off(&self) {
        println!("Top-O-Line Amplifier off");
    }
}
struct Tuner;
struct DvdPlayer {
    movie: Option<String>,
}
impl DvdPlayer {
    fn on(&self) {
        println!("Top-O-Line DVD on");
    }
    fn stop(&self) {
        println!("Top-O-Line DVD stopped \"{}\"", self.movie.clone().unwrap());
    }
    fn play(&mut self, movie: &str) {
        self.movie = Some(movie.to_string());
        println!("Top-O-Line DVD play \"{}\"", movie);
    }
    fn off(&self) {
        println!("Top-O-Line DVD off");
    }
    fn eject(&self) {
        println!("Top-O-Line DVD eject");
    }
}
struct CdPlayer;
struct Projector;
impl Projector {
    fn on(&self) {
        println!("Top-O-Line Project on");
    }
    fn wide_screen_mode(&self) {
        println!("Top-O-Line Project is widescreen mode");
    }
    fn off(&self) {
        println!("Top-O-Line Project off");
    }
}
struct TheaterLights;
impl TheaterLights {
    fn dim(&self, num: usize) {
        println!("Theater Ceiling Lights dimming to {}%", num);
    }
    fn on(&self) {
        println!("Theater Ceiling Lights on");
    }
}
struct Screen;
impl Screen {
    fn down(&self) {
        println!("Theater Screen going down");
    }
    fn up(&self) {
        println!("Theater Screen going up");
    }
}
struct PopcornPopper;
impl PopcornPopper {
    fn on(&self) {
        println!("Popcorn Popper on");
    }
    fn off(&self) {
        println!("Popcorn Popper off");
    }
    fn pop(&self) {
        println!("Popcorn Popper popping popcorn!");
    }
}

struct HomeTheaterFacade {
    amp: Amplifier,
    tuner: Tuner,
    dvd: DvdPlayer,
    cd: CdPlayer,
    projector: Projector,
    lights: TheaterLights,
    screen: Screen,
    popper: PopcornPopper,
}
impl HomeTheaterFacade {
    fn new(
        amp: Amplifier,
        tuner: Tuner,
        dvd: DvdPlayer,
        cd: CdPlayer,
        projector: Projector,
        lights: TheaterLights,
        screen: Screen,
        popper: PopcornPopper,
    ) -> Self {
        HomeTheaterFacade {
            amp,
            tuner,
            dvd,
            cd,
            projector,
            lights,
            screen,
            popper,
        }
    }

    fn watch_movie(&mut self, movie: &str) {
        println!("Get ready to watch a movie...");
        self.popper.on();
        self.popper.pop();
        self.lights.dim(10);
        self.screen.down();
        self.projector.on();
        self.projector.wide_screen_mode();
        self.amp.on();
        self.amp.set_dvd(&self.dvd);
        self.amp.set_surround_sound();
        self.amp.set_volume(5);
        self.dvd.on();
        self.dvd.play(movie);
    }
    fn end_movie(&self) {
        println!("Shutting movie theater down...");
        self.popper.off();
        self.lights.on();
        self.screen.up();
        self.projector.off();
        self.amp.off();
        self.dvd.stop();
        self.dvd.eject();
        self.dvd.off();
    }
}

#[test]
fn test_home_theater() {
    let mut home_theater = HomeTheaterFacade::new(
        Amplifier {},
        Tuner {},
        DvdPlayer { movie: None },
        CdPlayer {},
        Projector {},
        TheaterLights {},
        Screen {},
        PopcornPopper {},
    );
    home_theater.watch_movie("Raiders of the Lost Ark");
    home_theater.end_movie();
}
