use super::super::super::encoding::MessageReader;
use std::io::Cursor;

pub struct XdgToplevel;

impl XdgToplevel {
    pub const EVENT_CONFIGURE: u16 = 0;
    pub const EVENT_CLOSE: u16 = 1;
}

#[derive(PartialEq)]
pub enum ToplevelState {
    Maximized = 1,
    Fullscreen = 2,
    Resizing = 3,
    Activated = 4,
    TiledLeft = 5,
    TiledRight = 6,
    TiledTop = 7,
    TiledBottom = 8,
    Suspended = 9,
}

impl TryFrom<u32> for ToplevelState {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(ToplevelState::Maximized),
            2 => Ok(ToplevelState::Fullscreen),
            3 => Ok(ToplevelState::Resizing),
            4 => Ok(ToplevelState::Activated),
            5 => Ok(ToplevelState::TiledLeft),
            6 => Ok(ToplevelState::TiledRight),
            7 => Ok(ToplevelState::TiledTop),
            8 => Ok(ToplevelState::TiledBottom),
            9 => Ok(ToplevelState::Suspended),
            _ => Err(()),
        }
    }
}

pub struct ToplevelConfigure {
    pub width: i32,
    pub height: i32,
    pub states: Vec<ToplevelState>,
}

impl ToplevelConfigure {
    pub fn parse(data: &[u8]) -> Self {
        let mut cursor = Cursor::new(data);
        let width = cursor.read_u32() as i32;
        let height = cursor.read_u32() as i32;
        let array_len = cursor.read_u32() as i32;
        let num_states = array_len >> 2;
        let states = (0..num_states)
            .filter_map(|_| ToplevelState::try_from(cursor.read_u32()).ok())
            .collect();
        Self {
            width,
            height,
            states,
        }
    }

    pub fn is_maximized(&self) -> bool {
        self.states.contains(&ToplevelState::Maximized)
    }

    pub fn is_fullscreen(&self) -> bool {
        self.states.contains(&ToplevelState::Fullscreen)
    }

    pub fn is_resizing(&self) -> bool {
        self.states.contains(&ToplevelState::Resizing)
    }

    pub fn is_activated(&self) -> bool {
        self.states.contains(&ToplevelState::Activated)
    }

    pub fn is_tiled_left(&self) -> bool {
        self.states.contains(&ToplevelState::TiledLeft)
    }

    pub fn is_tiled_right(&self) -> bool {
        self.states.contains(&ToplevelState::TiledRight)
    }

    pub fn is_tiled_top(&self) -> bool {
        self.states.contains(&ToplevelState::TiledTop)
    }

    pub fn is_tiled_bottom(&self) -> bool {
        self.states.contains(&ToplevelState::TiledBottom)
    }

    pub fn is_suspended(&self) -> bool {
        self.states.contains(&ToplevelState::Suspended)
    }
}
