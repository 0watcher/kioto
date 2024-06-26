use enum_conversion::prelude::{DeriveTryFrom, EnumConversions};
use ratatui::style::Color as ratColor;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use strum::{Display, EnumIter, EnumString};
use tokio::net::TcpStream;

use crate::network::{RoomServer, User};

impl RoomServer {
    pub async fn serialize(&self) -> RoomData {
        let self_ = &self.inner.lock().await.data;
        RoomData {
            room_id: self_.room_id.clone(),
            socket_addr: self_.socket_addr,
            password: self_.password.clone(),
            locked_addrs: self_.locked_addrs.clone(),
            is_owner: self_.is_owner,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RoomStyle {
    pub fg: Color,
    pub bg: Color,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RoomData {
    pub room_id: String,
    pub socket_addr: SocketAddr,
    pub password: Option<String>,
    pub locked_addrs: Vec<SocketAddr>,
    pub style: RoomStyle,
    pub is_owner: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user_id: String,
    pub addr: SocketAddr,
    pub color: Color,
}

impl User {
    pub fn serialize(&self) -> UserData {
        UserData {
            addr: self.addr.peer_addr().unwrap(),
            user_id: self.user_id.clone(),
            color: self.color,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub msg_id: u32,
    pub sender_id: String,
    pub sender_color: Color,
    pub chatroom_id: String,
    pub content: String,
    pub timestamp: std::time::SystemTime,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
}

impl Into<ratColor> for Color {
    fn into(self) -> ratColor {
        match self {
            Color::Reset => ratColor::Reset,
            Color::Black => ratColor::Black,
            Color::Red => ratColor::Red,
            Color::Green => ratColor::Green,
            Color::Yellow => ratColor::Yellow,
            Color::Blue => ratColor::Blue,
            Color::Magenta => ratColor::Magenta,
            Color::Cyan => ratColor::Cyan,
            Color::Gray => ratColor::Gray,
            Color::DarkGray => ratColor::DarkGray,
            Color::LightRed => ratColor::LightRed,
            Color::LightGreen => ratColor::LightGreen,
            Color::LightYellow => ratColor::LightYellow,
            Color::LightBlue => ratColor::LightBlue,
            Color::LightMagenta => ratColor::LightMagenta,
            Color::LightCyan => ratColor::LightCyan,
            Color::White => ratColor::White,
        }
    }
}

#[derive(Serialize, Deserialize, EnumString, EnumIter, Display, PartialEq, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AppOpt {
    #[strum(serialize = "remember_passwords")]
    RememberPasswords,
    #[strum(serialize = "light_mode")]
    LightMode,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppOption {
    pub option: AppOpt,
    pub enabled: bool,
}
