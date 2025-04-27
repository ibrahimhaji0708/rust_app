// pub mod closures_rust;
use std::thread;
use std::time::Duration;

fn stimulated_exp_calc(intensity: u32) -> u32 {
    println!("calc slowly..");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T> 
where 
    T: Fn(u32) -> u32 {
        calculation: T,
        value: Option<u32>, 
    }

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = stimulated_exp_calc(intensity);
    let expensive_closure = |num: u32| -> u32 {
        println!("calc slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("today , do {} pushups", expensive_closure(intensity));
        println!("next , do {} situps", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("take a brk today stay hydrated.");
        } else {
            println!("today , run for {} minutes.", expensive_closure(intensity));
        }
    }
}

fn main() {

    let stimulated_intensity = 10;
    let stimulated_random_no = 7;

    generate_workout(stimulated_intensity, stimulated_random_no);
}
