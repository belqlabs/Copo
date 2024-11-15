# Architecture
This file documents the architecture of Copo.

## Entities
### Process - The One Who Obey
`alias: prc`

**Definition**:
Store basic information about how the orchestrator must manage processes defined in the configuration file. Also, after spawning, the process holds an instance the `Child

**Enums**:
```rust
enum TriggerTypes {
	Cron,
	Manual,
	DependenceFinish,
	OnStart
}
```

**Struct**:
```rust
struct Process {
	name: String,
	internal_name: Some(String),  // this will hold the copo assigned name to the process
	executable_path: String,
	file_path: String,
	depends_on: Vec<String>,
	triggerType: TriggerTypes,
	tryggerDefinition: String,
	process: Some(std::process::Child)
}
```

### Context
`alias: ctx`

**Definition**:
Holds information about the Copo application in a field named `meta` and data to share between processes in the `udd` field. 

**Structs**:
```rust
struct MetaReport{
	report_time: u64,
	report_buff: Vec<u8>
}
struct Meta {
	last_report: u64,
	reports: Vec<MetaReport>
}

struct UddRecord{
	sent_time: u64,
	sent_by: String , // this holds the internal name of the process that sent this record
	udd_buff: Vec<u8>
}
struct Udd{
	last_update: u64,
	last_update_by: String, // same as sent_by in UddRecord
	headers_buff: Vec<String>, // Holds the headers of the user defined header
	udd: Vec<UddRecord>
}

struct Context{
	meta: Meta,
	udd: Udd
}
```

### Server
`alias: srv`

**Definition**:
In windows, this is a named pipe, in unix (linux, mac), this is a unix domain socket. 
It will be used as a two-way communication bus between the orchestrator and the processes managed by it.

**Struct**:
```rust
struct Client{
	// TO BE DEFINED
}

struct Server{
	name: String, // this will hold the copo assigned name to the server
	listener: interprocess:Listener
}
```

### Orchestrator
`alias: orc`

**Definition**:
Manage process execution. Using os specific IPC methods defined by its `Server` to send data between the context and the processes

**Struct**:
```rust
struct Orchestrator{
	internal_name: String,
	context: Context,
	server: Server,
	processes: Vec<Process>
}
```

# File structure
```
$HOME
|_ .copo
    |_ <application>
        |_ reports.json
        |_ pids/
            |_ <process>.pid
        |_ logs
            |_ <process>.log
        |_ <orchestrator>
            |_ sock
        |_ <process>
            |_ stdio
```
