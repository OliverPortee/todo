use crate::model::*;

use chrono::{DateTime, Local};
use colored::Colorize;
use std::fmt::Display;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TIDMissing => write!(f, "Need to specify the ID of the task."),
            Error::InvalidTID(tid) => write!(f, "Task ID `{}` is invalid.", tid),
            Error::GroupMissing => write!(f, "Need to specify the group name after -g."),
            Error::InvalidArgument(arg) => write!(f, "Invalid argument: `{}`.", arg),
            Error::CannotReadDataFile => write!(f, "Cannot read data file."),
            Error::InvalidDataFile => write!(f, "Data file has the wrong format."),
            Error::SerializationError => write!(f, "Cannot serialize the data."),
            Error::CannotWriteOpenDataFile => write!(f, "Cannot open data file for writing."),
            Error::CannotWriteDataFile => write!(f, "Cannot write data file."),
            Error::NothingToUpdate => write!(f, "Nothing to update."),
            Error::InvalidGroup(group) => write!(f, "Invalid group name: `{}`.", group),
            Error::TIDsMissing => write!(f, "The list of task IDs is missing."),
            Error::NothingToMove => write!(f, "Nothing to move."),
        }
    }
}

fn format_date(date: &DateTime<Local>) -> String {
    let date = date.date_naive();
    let today = Local::now().date_naive();
    let diff = today - date;
    match diff.num_days() {
        0 => "today".to_string(),
        1 => "yesterday".to_string(),
        2 => "2d ago".to_string(),
        3 => "3d ago".to_string(),
        _ => format!("{}", date.format("%d.%m.%Y")),
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.prio {
            Prio::A => write!(f, "{}", "★".red())?,
            Prio::B => write!(f, "{}", "•")?,
            Prio::C => write!(f, "{}", "-".dimmed())?,
        }
        write!(f, " {}", format!("[{}]", self.tid).dimmed())?;
        write!(f, " {}", self.name)?;
        write!(f, "  {}", format_date(&self.date).dimmed().italic())?;
        Ok(())
    }
}

pub struct PrintGroup<'a> {
    name: &'a str,
    tasks: Vec<&'a Task>,
}

impl<'a> PrintGroup<'a> {
    pub fn new(group: &'a Group, model: &'a Model, prio: Option<Prio>) -> Self {
        let mut tasks = group
            .tids
            .iter()
            .map(|tid| model.tasks.get(tid).unwrap())
            .collect::<Vec<&Task>>();
        if let Some(prio) = prio {
            tasks.retain(|t| t.prio == prio);
        }
        PrintGroup {
            name: &group.name,
            tasks,
        }
    }
}

impl<'a> Display for PrintGroup<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.tasks.is_empty() {
            return write!(f, "");
        }
        writeln!(f, "\n{}\n", self.name.bold())?;
        for task in self.tasks.iter() {
            writeln!(f, "  {task}")?;
        }
        Ok(())
    }
}

pub struct PrintModel<'a> {
    groups: Vec<PrintGroup<'a>>,
    prio: Option<Prio>,
}

impl<'a> PrintModel<'a> {
    pub fn new(model: &'a Model, prio: Option<Prio>) -> Self {
        let groups: Vec<PrintGroup<'a>> = model
            .groups
            .values()
            .map(|g| PrintGroup::new(g, model, prio))
            .collect();
        // TODO: sort groups by something?
        PrintModel { groups, prio }
    }
}

impl<'a> Display for PrintModel<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.groups.is_empty() && self.prio.is_none() {
            writeln!(f, "\nThere are no todos right now. Good job :)")?;
            return Ok(())
        }
        // this case can only occur if the prio filters out all the tasks;
        // otherwise we delete groups without tasks immediately
        if self.groups.iter().all(|g| g.tasks.is_empty()) {
            assert!(self.prio.is_some());
            writeln!(f, "\nThere are no todos with the given priority.")?;
            return Ok(())
        }

        for group in self.groups.iter() {
            write!(f, "{}", group)?;
        }

        Ok(())
    }
}
