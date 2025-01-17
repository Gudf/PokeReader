use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(usize)]
pub enum PartySlot {
    Slot1 = 0,
    Slot2 = 1,
    Slot3 = 2,
    Slot4 = 3,
    Slot5 = 4,
    Slot6 = 5,
}

impl PartySlot {
    pub fn increment(&self) -> Self {
        match self {
            Self::Slot1 => Self::Slot2,
            Self::Slot2 => Self::Slot3,
            Self::Slot3 => Self::Slot4,
            Self::Slot4 => Self::Slot5,
            Self::Slot5 => Self::Slot6,
            Self::Slot6 => Self::Slot1,
        }
    }

    pub fn decrement(&self) -> Self {
        match self {
            Self::Slot1 => Self::Slot6,
            Self::Slot2 => Self::Slot1,
            Self::Slot3 => Self::Slot2,
            Self::Slot4 => Self::Slot3,
            Self::Slot5 => Self::Slot4,
            Self::Slot6 => Self::Slot5,
        }
    }
}

impl fmt::Display for PartySlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            Self::Slot1 => "1",
            Self::Slot2 => "2",
            Self::Slot3 => "3",
            Self::Slot4 => "4",
            Self::Slot5 => "5",
            Self::Slot6 => "6",
        };

        write!(f, "{}", display)
    }
}

impl Default for PartySlot {
    fn default() -> Self {
        Self::Slot1
    }
}
