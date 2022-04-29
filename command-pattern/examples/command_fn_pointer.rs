//////////////
// 函数指针版本
//////////////
type FnPtr = fn() -> String;
struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct Schema {
    commands: Vec<Command>,
}

impl Schema {
    // 初始化命令列表
    fn new() -> Self {
        Self { commands: vec![] }
    }
    // 添加命令
    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback });
    }
    // 正向迭代
    fn execute(&self) -> Vec<String> {
        self.commands
            .iter()              
            .map(|cmd| (cmd.execute)())
            .collect()
    }
    // 反向迭代
    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev() 
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

fn main() {
    let mut schema = Schema::new();
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(add_field, remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
