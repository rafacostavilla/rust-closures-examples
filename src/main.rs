use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use core::hash::Hash;

struct Cacher<T, U>
where 
    T: Fn(U)-> U,
{
    calculation: T,
    map: HashMap<U,U>,
}

impl <T, U>Cacher<T, U> 
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{

    fn new(calculation_function: T) -> Cacher<T, U>{
        Cacher {
            calculation: calculation_function,
            map: HashMap::new(),
        }
    }

    fn get_value(&mut self, arg: U) -> U{
        match self.map.get(&arg) {
            Some(v) => *v,    
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
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
        println!(
            "Today, do {} abs",
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