use std::fs;

struct MemoryRepo {
    num: i32,
}
impl Repository for MemoryRepo {
    fn save(&mut self, num: i32) {
        println!("Saving in memory!");
        self.num = num;
    }
}

struct FileRepo;
impl Repository for FileRepo {
    fn save(&mut self, num: i32) {
        println!("Saving in file!");
        fs::write("ioc_file_repo", num.to_string()).unwrap();
    }
}

trait Repository {
    fn save(&mut self, num: i32);
}

struct UseCase;
impl UseCase {
    fn run(mut repo: Box<dyn Repository>)
    where
        Self: Sized,
    {
        repo.save(32);
    }
}

fn main() {
    let mut memory_repo = MemoryRepo { num: 0 };
    UseCase::run(Box::new(memory_repo));
    let file_repo = FileRepo;
    UseCase::run(Box::new(file_repo));
}
