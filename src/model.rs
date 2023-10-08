use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::{DateTime, Local};

pub type TID = u32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Prio {
    A,
    B,
    C,
}

pub struct NewCommand {
    pub task: String,
    pub group: String,
    pub prio: Prio,
}

pub struct ListCommand {
    pub group: Option<String>,
    pub prio: Option<Prio>,
}

pub struct DoneCommand {
    pub tids: Vec<TID>,
}

pub struct UpdateCommand {
    pub tid: TID,
    pub group: Option<String>,
    pub prio: Option<Prio>,
    pub task: Option<String>,
}

pub struct DeleteGroupCommand {
    pub group: String,
}

pub struct MoveCommand {
    pub group: Option<String>,
    pub prio: Option<Prio>,
    pub tids: Vec<TID>,
}

pub enum Command {
    New(NewCommand),
    List(ListCommand),
    Done(DoneCommand),
    Update(UpdateCommand),
    DeleteGroup(DeleteGroupCommand),
    Move(MoveCommand),
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub tid: TID,
    pub name: String,
    pub prio: Prio,
    pub date: DateTime<Local>,
    pub group: String,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub tids: Vec<TID>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    pub tasks: HashMap<TID, Task>,
    pub groups: HashMap<String, Group>,
}

pub enum Error {
    TIDMissing,
    TIDsMissing,
    InvalidTID(String),
    GroupMissing,
    InvalidArgument(String),
    CannotReadDataFile,
    InvalidDataFile,
    SerializationError,
    CannotWriteOpenDataFile,
    CannotWriteDataFile,
    NothingToUpdate,
    InvalidGroup(String),
    NothingToMove,
}
