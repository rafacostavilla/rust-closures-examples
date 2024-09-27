use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32{
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where 
    T: Fn(u32)-> u32,
{
    calculation: T,
    value: Option<u32>,
}

fn main(){
    let simulated_intensity = 22;
    let simulated_random_number = 3;

    generate_workout(simulated_intensity, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25{
        println!(
            "Today, do {} pushups",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_closure(intensity)
        );
    } else {
        // This is not ideal because the expensive calculation will be called even in this case
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure(intensity)
            );
        }
    }
}