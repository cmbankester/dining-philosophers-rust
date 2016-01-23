use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

/// Philosopher has a name, a fork to their left, and a fork to their right. `left` and `right` are
/// the table-fork indexes (0-4) of the respective forks.
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    /// Creates a Philosopher with the specified `name`, `left` (fork index) and `right` (fork
    // index) values
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    /// Locks left fork mutex, sleeps 150ms, locks right fork mutex, sleeps 1s
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        println!("{} has picked up the left fork", self.name);
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

/// Table has a collection of "forks", which are simply mutexes (as a fork can only be used by one
/// person at a time)
struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    // Create a new Table with 5 forks, and provide it with atomic reference counting so that we
    // can access the table from different threads
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});
    // Create 5 philosophers, specifying the table-fork indexes of the fork to their left and the
    // fork to their right. Note the "Michel Foucault" philosopher is left-handed, and will thus
    // grab the fork to his left before grabbing the one to his right.
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4), // Michel Foucault is left-handed
    ];

    // Map each philosopher to a new thread and tell them to eat
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // Increment the table reference count by cloning it
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    // Loop through thread handles and block the main thread until they are all complete
    for h in handles {
        h.join().unwrap();
    }
}
