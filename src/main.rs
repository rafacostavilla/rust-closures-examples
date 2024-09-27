use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32{
//     println!("Calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T>
where 
    T: Fn(u32)-> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl <T>Cacher<T> 
where
    T: Fn(u32) -> u32,
{

    fn new(calculation_function: T) -> Cacher<T>{
        Cacher {
            calculation: calculation_function,
            value: None,
        }
    }

    fn get_value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,    
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }    
        } 
    }
}

fn main(){
    let simulated_intensity = 22;
    let simulated_random_number = 3;
    
    generate_workout(simulated_intensity, simulated_random_number);
}

fn generate_workout(mut intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25{
        println!(
            "Today, do {} pushups",
            expensive_closure.get_value(intensity)
        );
        intensity +=1;
        println!(
            "Next, do {} situps",
            expensive_closure.get_value(intensity)
        );
    } else {
        // This is not ideal because the expensive calculation will be called even in this case
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure.get_value(intensity)
            );
        }
    }
}