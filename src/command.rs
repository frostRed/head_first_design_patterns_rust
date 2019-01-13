//! 命令模式，关键在于有一个「命令类」将「动作主体」和「具体动作」打包封装在一个对象里面
//! 客户拿到这个命令实例后，只管触发动作即可，它不关心到底是谁在执行，和执行什么动作

use std::fmt::{self, Display};

// 抽象命令
trait Command: Display {
    fn execute(&mut self);
    fn undo(&mut self);
    // 所有实现了 Command 的 trait 都要有 box_clone 函数，克隆自己并用 Box 打包
    fn box_clone(&self) -> Box<dyn Command>;
}

// 这里为了对 Box<dyn Command> 实现 Clone，用了比较麻烦的技巧
impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        // dyn Command 的 clone 委托个具体的 struct
        self.box_clone()
    }
}
// 具体动作执行主体
#[derive(Clone)]
struct Light {
    name: String,
}
impl Light {
    fn new(name: &str) -> Self {
        Light {
            name: name.to_string(),
        }
    }
    fn on(&self) {
        println!("{} Light is on", self.name);
    }
    fn off(&self) {
        println!("{} Light is off", self.name);
    }
}

#[derive(Clone)]
struct Stereo {
    name: String,
    cd: Option<Cd>,
    volume: usize,
}
#[derive(Clone)]
struct Cd;
impl Stereo {
    fn new(name: &str) -> Self {
        Stereo {
            name: name.to_string(),
            cd: None,
            volume: 0,
        }
    }
    fn on(&self) {
        println!("{} Stereo is on", self.name);
    }
    fn off(&self) {
        println!("{} Stereo is off", self.name);
    }
    fn set_cd(&mut self, cd: Cd) {
        println!("{} Stereo is set for Cd input", self.name);
        self.cd = Some(cd);
    }
    fn set_volumn(&mut self, volume: usize) {
        println!("{} Stereo volume set to {}", self.name, volume);
        self.volume = volume;
    }
}

// 具体命令
#[derive(Clone)]
struct LightOnCommand {
    light: Light,
}
impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.on()
    }
    fn undo(&mut self) {
        self.light.off()
    }
    fn box_clone(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
impl Display for LightOnCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LightOnCommand")
    }
}
#[derive(Clone)]
struct LightOffCommand {
    light: Light,
}
impl Command for LightOffCommand {
    fn execute(&mut self) {
        self.light.off()
    }
    fn undo(&mut self) {
        self.light.on()
    }
    fn box_clone(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
impl Display for LightOffCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LightOffCommand")
    }
}

#[derive(Clone)]
struct NoCommand;
impl Command for NoCommand {
    fn execute(&mut self) {}
    fn undo(&mut self) {}
    fn box_clone(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
impl Display for NoCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NoCommand")
    }
}

#[derive(Clone)]
struct StereoOnWithCdCommand {
    stereo: Stereo,
}
impl Command for StereoOnWithCdCommand {
    fn execute(&mut self) {
        self.stereo.on();
        self.stereo.set_cd(Cd {});
        self.stereo.set_volumn(11);
    }
    fn undo(&mut self) {
        self.stereo.off();
    }
    fn box_clone(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
impl Display for StereoOnWithCdCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StereoOnWithCdCommand")
    }
}

#[derive(Clone)]
struct StereoOffCommand {
    stereo: Stereo,
}
impl Command for StereoOffCommand {
    fn execute(&mut self) {
        self.stereo.off();
    }
    fn undo(&mut self) {
        self.stereo.on();
        self.stereo.set_cd(Cd {});
        self.stereo.set_volumn(11);
    }
    fn box_clone(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
impl Display for StereoOffCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StereoOffCommand")
    }
}

#[derive(Clone)]
struct MacroCommand {
    commands: Vec<Box<dyn Command>>,
}
impl Command for MacroCommand {
    fn execute(&mut self) {
        for i in &mut self.commands {
            i.execute();
        }
    }
    fn undo(&mut self) {
        for i in &mut self.commands {
            i.undo();
        }
    }
    fn box_clone(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }
}
impl Display for MacroCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MacroCommand")
    }
}

// 命令类的消费者
struct RemoteControl {
    on_commands: Vec<Box<dyn Command>>,
    off_commands: Vec<Box<dyn Command>>,
    undo_command: Box<dyn Command>,
}

impl RemoteControl {
    pub fn new() -> Self {
        let no_command = NoCommand {};
        let mut on_commands = Vec::with_capacity(7);
        let mut off_commands = Vec::with_capacity(7);

        for i in 0..7 {
            on_commands.push(Box::new(no_command.clone()) as Box<dyn Command>);
            off_commands.push(Box::new(no_command.clone()) as Box<dyn Command>);
        }
        RemoteControl {
            on_commands: on_commands,
            off_commands: off_commands,
            undo_command: Box::new(no_command.clone()),
        }
    }
    pub fn set_command(
        &mut self,
        slot: usize,
        on_command: Box<dyn Command>,
        off_command: Box<dyn Command>,
    ) {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }

    pub fn on_button_was_pushed(&mut self, slot: usize) {
        self.on_commands[slot].execute();
        self.undo_command = self.on_commands[slot].clone();
    }
    pub fn off_button_was_pushed(&mut self, slot: usize) {
        self.off_commands[slot].execute();
        self.undo_command = self.off_commands[slot].clone();
    }
    pub fn undo_button_was_pushed(&mut self) {
        self.undo_command.undo();
    }
}

impl Display for RemoteControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n------ Remote Control ------\n")?;
        let len = self.on_commands.len();
        for i in 0..len {
            write!(
                f,
                "[slot {}] {}    {}\n",
                i, self.on_commands[i], self.off_commands[i]
            )?;
        }
        Ok(())
    }
}

#[test]
fn test_command() {
    let living_room_light = Light::new("Living Room");
    let kitchen_room_light = Light::new("Kitchen Room");
    let stereo = Stereo::new("Living Room");

    let living_room_light_on = LightOnCommand {
        light: living_room_light.clone(),
    };
    let living_room_light_off = LightOffCommand {
        light: living_room_light,
    };

    let kitchen_room_light_on = LightOnCommand {
        light: kitchen_room_light.clone(),
    };
    let kitchen_room_light_off = LightOffCommand {
        light: kitchen_room_light,
    };

    let stereo_on_with_cd = StereoOnWithCdCommand {
        stereo: stereo.clone(),
    };
    let stereo_off = StereoOffCommand { stereo: stereo };

    let mut remote_control = RemoteControl::new();
    remote_control.set_command(
        0,
        Box::new(living_room_light_on),
        Box::new(living_room_light_off),
    );
    remote_control.set_command(
        1,
        Box::new(kitchen_room_light_on),
        Box::new(kitchen_room_light_off),
    );
    remote_control.set_command(3, Box::new(stereo_on_with_cd), Box::new(stereo_off));
}

#[test]
fn test_command_undo() {
    let living_room_light = Light::new("Living Room");
    let living_room_light_on = LightOnCommand {
        light: living_room_light.clone(),
    };
    let living_room_light_off = LightOffCommand {
        light: living_room_light,
    };

    let mut remote_control = RemoteControl::new();
    remote_control.set_command(
        0,
        Box::new(living_room_light_on),
        Box::new(living_room_light_off),
    );

    remote_control.on_button_was_pushed(0);
    remote_control.off_button_was_pushed(0);
    println!("{}", remote_control);
    remote_control.undo_button_was_pushed();
    remote_control.off_button_was_pushed(0);
    remote_control.on_button_was_pushed(0);
    println!("{}", remote_control);
    remote_control.undo_button_was_pushed();
}

#[test]
fn test_macro_command() {
    let living_room_light = Light::new("Living Room");
    let kitchen_room_light = Light::new("Kitchen Room");
    let stereo = Stereo::new("Living Room");

    let living_room_light_on = LightOnCommand {
        light: living_room_light.clone(),
    };
    let living_room_light_off = LightOffCommand {
        light: living_room_light,
    };

    let kitchen_room_light_on = LightOnCommand {
        light: kitchen_room_light.clone(),
    };
    let kitchen_room_light_off = LightOffCommand {
        light: kitchen_room_light,
    };

    let stereo_on_with_cd = StereoOnWithCdCommand {
        stereo: stereo.clone(),
    };
    let stereo_off = StereoOffCommand { stereo: stereo };

    let party_on_command = MacroCommand {
        commands: vec![
            Box::new(living_room_light_on),
            Box::new(kitchen_room_light_on),
            Box::new(stereo_on_with_cd),
        ],
    };
    let party_off_command = MacroCommand {
        commands: vec![
            Box::new(living_room_light_off),
            Box::new(kitchen_room_light_off),
            Box::new(stereo_off),
        ],
    };

    let mut remote_control = RemoteControl::new();
    remote_control.set_command(0, Box::new(party_on_command), Box::new(party_off_command));

    println!("{}", remote_control);
    println!("--- Pushing Macro On ---");
    remote_control.on_button_was_pushed(0);
    println!("--- Pushing Macro Off ---");
    remote_control.off_button_was_pushed(0);
}
