## Test Pass
 rust_projects/ink/todo  > cargo test
   Compiling todo v0.1.0 (/rust_projects/ink/todo)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.13s
     Running unittests lib.rs (target/debug/deps/todo-0edcf37dd9b51203)

running 5 tests
test todo::tests::update_task_works ... ok
test todo::tests::delete_task_works ... ok
test todo::tests::get_all_tasks_works ... ok
test todo::tests::add_and_get_task_works ... ok
test todo::tests::complete_task_works ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests todo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


## Build Pass
rust_projects/ink/todo  > cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s