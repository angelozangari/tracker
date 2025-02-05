### Architecture

- user <-> mic + speakers <-> text-to-speach + speech-to-text <-(1)-> ... <-(2)-> kb
- ... includes

  - crud commands on tasks

### Task has

  - name
  - status
  - deadline
  - dependency


### Agent axioms

  - tasks need time
  - time available is limited
  - schedule tasks -> suggest when to start and what
  - update progress on tasks
  - update tasks based on new information
  - gather data for analytics (metrics: efficiency, ...)

### To-do
  - [ ] define interface speech-to-text and back (1)
    - [X] crud
      - [X] create new task (name, deadline, status=false, dependencies=none)
      - [X] read existing task (name, deadline, status, dependencies)
      - [X] update existing task (name, deadline, status, dependencies)
      - [X] delete existing task (name, deadline)
    - [ ] agent suggestions
      - input: current time, time available
      - output: list task suggestion
  - [X] define interface agent-to-kb (2)
    - [X] crud
      - [X] same as (1)


### Crud module
- is relative to the `<-(1)-> ... <-(2)->` part in **Architecture**

- first of all we suppose that tasks are uniquely identifiable by name and date, thus two tasks with the same name cannot have the same deadline (e.g. 'pizza with friends' can happen both on monday at 8PM and on tuesday at 8PM)

- modules need to return **kwargs for the ones that they haven't been able to parse and need repeated