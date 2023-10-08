
# Introduction

This is a small no-bullshit command-line tool for managing to-dos. Tasks are organized in groups and have one of three priorities.

# Installation

* clone this project
* `cd todo`
* `cargo install --path .`
* make sure that `~/.cargo/bin` is in your `$PATH`

# Usage

* `todo`: list todos
* `todo study the Pythagoras theorem`: new task in group `Default` with prio `b`
* `todo take out the trash -g household -a`: new task in group `household` with prio `a` (other prios are `b` and `c`)
* `todo -g household -a`: list todos in group `household` and with prio `a`
* `todo -u 1 -g unimportant -c`: update task 1 to be in group `unimportant` and have prio `c`
* `todo -m 0,1 -g my_todos -b`: move multiple tasks to group `my_todos` and set prio to `b`
* `todo -dg my_todos`: delete group `my_todos`, deletes all tasks in the group
* `todo task1`
* `todo task2`
* `todo -d 0,1`: delete tasks (when you're done with them)
