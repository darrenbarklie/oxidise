use std::thread;

fn main() {
    let mut data = 100;


    // expect error: closure may outlive the current function, but it borrows
    //   `data`, which is owned by the current function
    // expect error: cannot borrow `data` as mutable more than once at a time
    // expect error:  cannot borrow `data` as immutable because it is also
    //   borrowed as mutable
    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });

    println!("{}", data);

    println!("Done!");
}
