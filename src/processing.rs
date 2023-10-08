use crate::display::PrintGroup;
use crate::model::*;
use crate::storage::*;

fn find_new_tid(model: &Model) -> TID {
    for tid in 0..TID::MAX {
        if !model.tasks.contains_key(&tid) {
            return tid;
        }
    }
    panic!("all task ids are occupied");
}

fn process_cmd_new(cmd: NewCommand) -> Result<(), Error> {
    let mut model = read_model()?;
    let tid = find_new_tid(&model);
    model.tasks.insert(
        tid,
        Task {
            tid,
            name: cmd.task,
            date: chrono::Local::now(),
            prio: cmd.prio,
        },
    );
    if !model.groups.contains_key(&cmd.group) {
        let group = Group {
            name: cmd.group.clone(),
            tids: Vec::new(),
        };
        model.groups.insert(cmd.group.clone(), group);
    }
    {
        let group = model.groups.get_mut(&cmd.group).unwrap();
        group.tids.push(tid);

        group.tids.sort_by_key(|tid| {
            let task = model.tasks.get(tid).unwrap();
            (task.prio, task.date)
        });
    }
    write_model(&model)?;

    println!("\nNew task with id {tid}");
    let group = model.groups.get(&cmd.group).unwrap();
    println!("{}", PrintGroup::new(group, &model, None));

    Ok(())
}

fn process_cmd_list(cmd: ListCommand) -> Result<(), Error> {
    let model = read_model()?;
    if let Some(group_name) = cmd.group {
        if !model.groups.contains_key(&group_name) {
            return Err(Error::InvalidGroup(group_name));
        }
        let group = model.groups.get(&group_name).unwrap();
        println!("{}", PrintGroup::new(group, &model, cmd.prio));
    } else {
        let groups: Vec<&Group> = model.groups.values().collect();
        if groups.is_empty() {
            println!("\nThere are no todos right now. Good job :)\n");
        }
        for group in groups {
            println!("{}", PrintGroup::new(group, &model, cmd.prio));
        }
    }
    Ok(())
}

// TODO: O(n^2) – cringe!
// returns the name of the group and the index in that group if found
fn find_group_of_task(tid: TID, model: &Model) -> Option<(&String, usize)> {
    for group in model.groups.values() {
        for (index, id) in group.tids.iter().enumerate() {
            if *id == tid {
                return Some((&group.name, index));
            }
        }
    }

    None
}

fn process_cmd_done(cmd: DoneCommand) -> Result<(), Error> {
    let mut model = read_model()?;

    // make sure that the tids are valid and return error otherwise
    for tid in cmd.tids.iter() {
        let _ = model
            .tasks
            .get(tid)
            .ok_or(Error::InvalidTID(tid.to_string()))?;
    }

    // O(n^3) - cringe TODO: safe group id in tasks
    for tid in cmd.tids.iter() {
        // get the name of the task's group
        let (group_name, index) = find_group_of_task(*tid, &model).unwrap();
        let group_name = group_name.clone();
        {
            // get a reference to the right group from the model
            let group: &mut Group = model.groups.get_mut(&group_name).unwrap();
            // delete tid from the right group
            group.tids.remove(index);
            // if the group is empty, delete the group
            if group.tids.is_empty() {
                model.groups.remove(&group_name);
            }
            model.tasks.remove(&tid);
        }
    }
    write_model(&model)?;

    println!("Delete successful.");

    Ok(())
}

fn move_task(tid: TID, model: &mut Model, new_group_name: &str) {
    let (old_group_name, index) = find_group_of_task(tid, model).unwrap();
    let old_group_name = old_group_name.clone();
    if new_group_name == old_group_name {
        return;
    }
    // remove the tid from the old group
    let old_group: &mut Group = model.groups.get_mut(&old_group_name).unwrap();
    old_group.tids.remove(index);
    if old_group.tids.is_empty() {
        model.groups.remove(&old_group_name);
    }

    // create new group if `new_group_name` doesn't exist yet
    if !model.groups.contains_key(new_group_name) {
        let group = Group {
            name: new_group_name.to_string(),
            tids: Vec::new(),
        };
        model.groups.insert(new_group_name.to_string(), group);
    }

    // insert the tid to the new group
    let new_group: &mut Group = model.groups.get_mut(new_group_name).unwrap();
    new_group.tids.push(tid);
}

fn process_cmd_update(cmd: UpdateCommand) -> Result<(), Error> {
    let tid = cmd.tid;
    let mut model = read_model()?;

    // make sure that the task exists
    let _ = model
        .tasks
        .get(&tid)
        .ok_or(Error::InvalidTID(tid.to_string()))?;

    let task = model.tasks.get_mut(&tid).unwrap();
    if let Some(prio) = cmd.prio {
        task.prio = prio;
    }
    if let Some(name) = cmd.task {
        task.name = name;
    }
    if let Some(ref group_name) = cmd.group {
        move_task(tid, &mut model, group_name);
    }

    // figure out the group of the task in order to sort all the tasks in the group
    let group_name: String;
    {
        let group = match cmd.group {
            Some(new_group_name) => model.groups.get_mut(&new_group_name).unwrap(),
            None => {
                let (old_group_name, _) = find_group_of_task(tid, &model).unwrap();
                let old_group_name = old_group_name.clone();
                model.groups.get_mut(&old_group_name).unwrap()
            }
        };
        group.tids.sort_by_key(|tid| {
            let task = model.tasks.get(tid).unwrap();
            (task.prio, task.date)
        });
        group_name = group.name.clone();
    }

    write_model(&model)?;

    println!("\nUpdate successful.");
    let group = model.groups.get(&group_name).unwrap();
    println!("{}", PrintGroup::new(group, &model, None));

    Ok(())
}

fn process_cmd_move(cmd: MoveCommand) -> Result<(), Error> {
    let mut model = read_model()?;

    // check that all tids exist
    for tid in cmd.tids.iter() {
        let _ = model
            .tasks
            .get(&tid)
            .ok_or(Error::InvalidTID(tid.to_string()))?;
    }

    if let Some(prio) = cmd.prio {
        for tid in cmd.tids.iter() {
            let task = model.tasks.get_mut(tid).unwrap();
            task.prio = prio;
        }
    }

    if let Some(ref new_group_name) = cmd.group {
        for tid in cmd.tids.iter() {
            move_task(*tid, &mut model, new_group_name);
        }
    }

    // sort all the groups
    for group in model.groups.values_mut() {
        group.tids.sort_by_key(|tid| {
            let task = model.tasks.get(tid).unwrap();
            (task.prio, task.date)
        })
    }

    write_model(&model)?;

    println!("\nMove successful.\n");

    Ok(())
}

fn process_cmd_deletegroup(cmd: DeleteGroupCommand) -> Result<(), Error> {
    let mut model = read_model()?;

    if !model.groups.contains_key(&cmd.group) {
        return Err(Error::InvalidGroup(cmd.group));
    }

    let group = model.groups.get(&cmd.group).unwrap();
    for tid in group.tids.iter() {
        model.tasks.remove(tid);
    }
    model.groups.remove(&cmd.group);

    write_model(&model)?;

    println!("\nDeleted group successfully.\n");

    Ok(())
}

pub fn process_command(command: Command) -> Result<(), Error> {
    match command {
        Command::New(cmd) => process_cmd_new(cmd),
        Command::List(cmd) => process_cmd_list(cmd),
        Command::Done(cmd) => process_cmd_done(cmd),
        Command::Update(cmd) => process_cmd_update(cmd),
        Command::DeleteGroup(cmd) => process_cmd_deletegroup(cmd),
        Command::Move(cmd) => process_cmd_move(cmd),
    }
}
