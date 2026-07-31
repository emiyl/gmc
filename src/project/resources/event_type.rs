use std::str::FromStr;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum EventType {
    Create = 0,
    Destroy = 1,
    Alarm = 2,
    Step = 3,
    Collision = 4,
    Keyboard = 5,
    Mouse = 6,
    Other = 7,
    Draw = 8,
    KeyPress = 9,
    KeyRelease = 10,
    Cleanup = 12,
    PreCreate = 14,
}

impl EventType {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventType::Create => "Create",
            EventType::Destroy => "Destroy",
            EventType::Alarm => "Alarm",
            EventType::Step => "Step",
            EventType::Collision => "Collision",
            EventType::Keyboard => "Keyboard",
            EventType::Mouse => "Mouse",
            EventType::Other => "Other",
            EventType::Draw => "Draw",
            EventType::KeyPress => "KeyPress",
            EventType::KeyRelease => "KeyRelease",
            EventType::Cleanup => "Cleanup",
            EventType::PreCreate => "PreCreate",
        }
    }
}

impl FromStr for EventType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Create" => Ok(Self::Create),
            "Destroy" => Ok(Self::Destroy),
            "Alarm" => Ok(Self::Alarm),
            "Step" => Ok(Self::Step),
            "Collision" => Ok(Self::Collision),
            "Keyboard" => Ok(Self::Keyboard),
            "Mouse" => Ok(Self::Mouse),
            "Other" => Ok(Self::Other),
            "Draw" => Ok(Self::Draw),
            "KeyPress" => Ok(Self::KeyPress),
            "KeyRelease" => Ok(Self::KeyRelease),
            "Cleanup" => Ok(Self::Cleanup),
            "PreCreate" => Ok(Self::PreCreate),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone)]
pub enum EventSubType {
    None = 0,

    Alarm(u32),

    Step(StepEvent),
    Draw(DrawEvent),
    Mouse(MouseEvent),
    Other(OtherEvent),

    Keyboard(u32),
    KeyPress(u32),
    KeyRelease(u32),

    Collision(String),
}

impl From<&EventSubType> for i32 {
    fn from(event_subtype: &EventSubType) -> Self {
        match event_subtype {
            EventSubType::None => 0,
            EventSubType::Alarm(n) => 1 + *n as i32,
            EventSubType::Step(e) => 10 + *e as i32,
            EventSubType::Draw(e) => 20 + *e as i32,
            EventSubType::Mouse(e) => 30 + *e as i32,
            EventSubType::Other(e) => 40 + *e as i32,
            EventSubType::Keyboard(n) => 50 + *n as i32,
            EventSubType::KeyPress(n) => 60 + *n as i32,
            EventSubType::KeyRelease(n) => 70 + *n as i32,
            EventSubType::Collision(_) => 80, // Collision events are more complex and may require additional handling
        }
    }
}

impl EventSubType {
    pub fn value(&self) -> i32 {
        self.into()
    }

    pub fn from_i32(event_type: EventType, value: i32) -> EventSubType {
        match event_type {
            EventType::Alarm => EventSubType::Alarm((value - 1) as u32),
            EventType::Step => match value - 10 {
                0 => EventSubType::Step(StepEvent::Normal),
                1 => EventSubType::Step(StepEvent::Begin),
                2 => EventSubType::Step(StepEvent::End),
                _ => EventSubType::Step(StepEvent::Normal), // Default case
            },
            EventType::Draw => match value - 20 {
                0 => EventSubType::Draw(DrawEvent::Normal),
                64 => EventSubType::Draw(DrawEvent::Gui),
                72 => EventSubType::Draw(DrawEvent::Begin),
                73 => EventSubType::Draw(DrawEvent::End),
                74 => EventSubType::Draw(DrawEvent::GuiBegin),
                75 => EventSubType::Draw(DrawEvent::GuiEnd),
                76 => EventSubType::Draw(DrawEvent::Pre),
                77 => EventSubType::Draw(DrawEvent::Post),
                _ => EventSubType::Draw(DrawEvent::Normal), // Default case
            },
            EventType::Mouse => match value - 30 {
                0 => EventSubType::Mouse(MouseEvent::LeftButton),
                1 => EventSubType::Mouse(MouseEvent::RightButton),
                2 => EventSubType::Mouse(MouseEvent::MiddleButton),
                60 => EventSubType::Mouse(MouseEvent::WheelUp),
                61 => EventSubType::Mouse(MouseEvent::WheelDown),
                _ => EventSubType::Mouse(MouseEvent::LeftButton), // Default case
            },
            _ => EventSubType::None, // Default case for other event types
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum StepEvent {
    Normal = 0,
    Begin = 1,
    End = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum DrawEvent {
    Normal = 0,
    Gui = 64,
    Begin = 72,
    End = 73,
    GuiBegin = 74,
    GuiEnd = 75,
    Pre = 76,
    Post = 77,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum MouseEvent {
    LeftButton = 0,
    RightButton = 1,
    MiddleButton = 2,
    // ...
    WheelUp = 60,
    WheelDown = 61,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum OtherEvent {
    OutsideRoom = 0,
    GameStart = 2,
    RoomStart = 4,
    // ...
    AsyncSystem = 75,
}
