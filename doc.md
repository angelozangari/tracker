architecture:
- user <-> mic + speakers <-> text-to-speach + speech-to-text <-(1)-> ... <-(2)-> kb

- ... includes
  - crud commands on tasks

- task has:
  - name
  - status
  - deadline
  - dependency

- agent axioms:
  - tasks need time
  - time available is limited
  - schedule tasks -> suggest when to start and what
  - update progress on tasks
  - update tasks based on new information
  - gather data for analytics (metrics: efficiency, ...)

- todo
  - [ ] define interface speech-to-text and back (1)
    - [ ] crud
      - [ ] create new task (name, deadline, status=false, dependencies=none)
      - [ ] read existing task (name, deadline, status, dependencies)
      - [ ] update existing task (name, deadline, status, dependencies)
      - [ ] delete existing task (name, deadline)
    - [ ] agent suggestions
      - input: current time, time available
      - output: list task suggestion
  - [ ] define interface agent-to-kb (2)
    - [ ] crud
      - [ ] same as (1)