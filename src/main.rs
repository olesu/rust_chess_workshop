use std::env::args;

use crate::test_runner::highest_passing_test;

mod assertions;
mod finished_game;
mod square;
mod task_0;
mod task_1;
mod task_2;
mod task_3;
mod task_4;
mod task_5;
mod task_6;
mod task_7;
mod task_8;
mod task_9;
mod test_runner;

fn main() {
    println!("Velkomen til Rust-workshop!");

    if let Some(task) = args().nth(1) {
        match task.as_str() {
            "0" => task_0::main(),
            "1" => task_1::main(),
            "2" => task_2::main(),
            "3" => task_3::main(),
            "4" => task_4::main(),
            "5" => task_5::main(),
            "6" => task_6::main(),
            "7" => task_7::main(),
            "8" => task_8::main(),
            "9" => task_9::main(),
            _ => {
                println!("Køyrer ferdig spel");
                finished_game::main();
            }
        }
    }

    if let Some(task) = highest_passing_test() {
        println!("Tester til og med oppgåve {} køyrde grønt!", task);
        match task {
            0 => task_1::main(),
            _ => {
                println!("Alle tester kjører grønt! Starter sjakkspill:");
                finished_game::main()
            }
        }
    }

    println!("Alle tester køyrde rødt! Teiknar brett for oppgåve 0");
    task_0::main()
}
