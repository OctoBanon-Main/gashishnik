use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    GetMessages = 0x00,
    SendUnauthenticated = 0x01,
    SendAuthenticated = 0x02,
    Register = 0x03,
}

impl TryFrom<u8> for Command {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::GetMessages),
            0x01 => Ok(Self::SendUnauthenticated),
            0x02 => Ok(Self::SendAuthenticated),
            0x03 => Ok(Self::Register),
            _ => Err(()),
        }
    }
}
