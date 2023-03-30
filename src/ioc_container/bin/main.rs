use std::{fs, rc::Rc};

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
    fn run(repo: &mut Rc<dyn Repository>)
    where
        Self: Sized,
    {
        if let Some(repo) = Rc::get_mut(repo) {
            repo.save(32);
        }
    }
}

struct Container {
    number_repository: Rc<dyn Repository>,
}

fn main() {
    let mut container = Container {
        number_repository: Rc::new(FileRepo),
    };
    UseCase::run(&mut container.number_repository);
    UseCase::run(&mut container.number_repository);
}
