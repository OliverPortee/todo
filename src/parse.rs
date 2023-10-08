use crate::model::*;

enum CommandType {
    New,
    List,
    Done,
    Update,
    DeleteGroup,
    Move,
}

fn parse_cmd_new(args: &[String]) -> Result<Command, Error> {
    let mut group: String = "Default".to_string();
    let mut prio: Prio = Prio::B;
    let mut task: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-g" => {
                if i + 1 >= args.len() {
                    return Err(Error::GroupMissing);
                }
                group = args[i + 1].clone();
                i += 2;
            }
            "-a" => {
                prio = Prio::A;
                i += 1;
            }
            "-b" => {
                prio = Prio::B;
                i += 1;
            }
            "-c" => {
                prio = Prio::C;
                i += 1;
            }
            word => {
                task.push(word.to_string());
                i += 1;
            }
        }
    }
    Ok(Command::New(NewCommand {
        task: task.join(" "),
        group,
        prio,
    }))
}

fn parse_cmd_list(args: &[String]) -> Result<Command, Error> {
    let mut group: Option<String> = None;
    let mut prio: Option<Prio> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-g" => {
                if i + 1 >= args.len() {
                    return Err(Error::GroupMissing);
                }
                group = Some(args[i + 1].clone());
                i += 2;
            }
            "-a" => {
                prio = Some(Prio::A);
                i += 1;
            }
            "-b" => {
                prio = Some(Prio::B);
                i += 1;
            }
            "-c" => {
                prio = Some(Prio::C);
                i += 1;
            }
            word => {
                return Err(Error::InvalidArgument(word.to_string()));
            }
        }
    }
    Ok(Command::List(ListCommand { group, prio }))
}

fn parse_cmd_done(args: &[String]) -> Result<Command, Error> {
    assert!(args[1] == "-d");
    if args.len() > 3 {
        return Err(Error::InvalidArgument(args[3..].join(" ")));
    }
    if args.len() == 2 {
        return Err(Error::TIDMissing);
    }
    let tid_str = &args[2];
    let mut tids: Vec<TID> = Vec::new();

    for tid in tid_str.split(",") {
        let tid: TID = tid
            .parse()
            .map_err(|_| Error::InvalidTID(tid.to_string()))?;
        tids.push(tid);
    }
    if tids.is_empty() {
        return Err(Error::TIDsMissing);
    }

    Ok(Command::Done(DoneCommand { tids }))
}

fn parse_cmd_update(args: &[String]) -> Result<Command, Error> {
    let mut tid: Option<TID> = None;
    let mut group: Option<String> = None;
    let mut prio: Option<Prio> = None;
    let mut task: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-u" => {
                if i + 1 >= args.len() {
                    return Err(Error::TIDMissing);
                }
                let tid_str = &args[i + 1];
                tid = Some(
                    tid_str
                        .parse()
                        .map_err(|_| Error::InvalidTID(tid_str.to_string()))?,
                );
                i += 2;
            }
            "-g" => {
                if i + 1 >= args.len() {
                    return Err(Error::GroupMissing);
                }
                group = Some(args[i + 1].clone());
                i += 2;
            }
            "-a" => {
                prio = Some(Prio::A);
                i += 1;
            }
            "-b" => {
                prio = Some(Prio::B);
                i += 1;
            }
            "-c" => {
                prio = Some(Prio::C);
                i += 1;
            }
            word => {
                task.push(word.to_string());
                i += 1;
            }
        }
    }
    assert!(tid.is_some());
    if group.is_none() && prio.is_none() && task.is_empty() {
        return Err(Error::NothingToUpdate);
    }
    Ok(Command::Update(UpdateCommand {
        tid: tid.unwrap(),
        group,
        prio,
        task: if task.is_empty() {
            None
        } else {
            Some(task.join(" "))
        },
    }))
}

fn parse_cmd_deletegroup(args: &[String]) -> Result<Command, Error> {
    let mut group: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-dg" => {
                if i + 1 >= args.len() {
                    return Err(Error::GroupMissing);
                }
                group = Some(args[i + 1].to_string());
                i += 2;
            }
            word => Err(Error::InvalidArgument(word.to_string()))?,
        }
    }

    assert!(group.is_some());
    Ok(Command::DeleteGroup(DeleteGroupCommand {
        group: group.unwrap(),
    }))
}

fn parse_cmd_move(args: &[String]) -> Result<Command, Error> {
    let mut group: Option<String> = None;
    let mut prio: Option<Prio> = None;
    let mut tasks: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-m" => {
                if i + 1 >= args.len() {
                    return Err(Error::TIDsMissing);
                }
                tasks = Some(args[i + 1].to_string());
                i += 2;
            }
            "-g" => {
                if i + 1 >= args.len() {
                    return Err(Error::GroupMissing);
                }
                group = Some(args[i + 1].to_string());
                i += 2;
            }
            "-a" => {
                prio = Some(Prio::A);
                i += 1;
            }
            "-b" => {
                prio = Some(Prio::B);
                i += 1;
            }
            "-c" => {
                prio = Some(Prio::C);
                i += 1;
            }
            word => {
                return Err(Error::InvalidArgument(word.to_string()));
            }
        }
    }

    if group.is_none() && prio.is_none() {
        return Err(Error::NothingToMove);
    }

    assert!(tasks.is_some());
    let tasks = tasks.unwrap();
    let mut tids: Vec<TID> = Vec::new();
    for s in tasks.split(",") {
        let tid = s
            .parse::<TID>()
            .map_err(|_| Error::InvalidTID(s.to_string()))?;
        tids.push(tid);
    }
    if tids.is_empty() {
        return Err(Error::NothingToMove);
    }

    Ok(Command::Move(MoveCommand { group, prio, tids }))
}

pub fn parse(args: &[String]) -> Result<Command, Error> {
    let mut command_type = CommandType::List;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" => {
                command_type = CommandType::Done;
                break;
            }
            "-u" => {
                command_type = CommandType::Update;
                break;
            }
            "-dg" => {
                command_type = CommandType::DeleteGroup;
                break;
            }
            "-m" => {
                command_type = CommandType::Move;
                break;
            }
            "-g" => {
                i += 2;
            }
            "-a" | "-b" | "-c" => {
                i += 1;
            }
            _ => {
                command_type = CommandType::New;
                break;
            }
        }
    }

    match command_type {
        CommandType::New => parse_cmd_new(args),
        CommandType::List => parse_cmd_list(args),
        CommandType::Done => parse_cmd_done(args),
        CommandType::Update => parse_cmd_update(args),
        CommandType::DeleteGroup => parse_cmd_deletegroup(args),
        CommandType::Move => parse_cmd_move(args),
    }
}
