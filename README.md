
# Usage

* `todo take out the trash -g household -[a/b/c]`: new task with group household and prio a, b or c
* `todo`: list todos
* `todo -g household -a`: list todos with group household and prio a
* `todo -d 12`: done task number 12 (deletes the task)
* `todo -u 7 -g Group -c`: update task
* `todo -dg Group`: delete group
* `todo -m 12,4,2,6 -g household -a`: move multiple tasks
* `todo -d 12,5,16,8`: delete multiple tasks

# To do

* when filtering for a prio and no tasks fulfill this criterion, print a nice message (and not nothing)