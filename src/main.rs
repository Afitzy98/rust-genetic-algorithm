use num_bigint::{BigInt, RandBigInt};
use num_traits::One;
use rand::Rng;
use std::env;

fn generate_random_bits(n: u64, l: u64) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut bits: Vec<String> = vec![];

    let mut max: BigInt = One::one();
    let two = BigInt::from(2);
    for _i in 0..l {
        max *= two.clone();
    }
    for _i in 0..n {
        bits.push(format!("{num:0len$b}", num=rng.gen_bigint_range(&BigInt::from(0), &max), len=l as usize));
    }
    drop(rng);
    bits
}

fn mutate(b: String) -> String {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..b.len()) as u64;
    let mut mutated = String::with_capacity(b.len());
    let mut i: u64 = 0;
    
    for c in b.chars() {
        if i == idx {
            if c == '0' {
                mutated.push('1');
            } else {
                mutated.push('0');
            }
        } else {
            mutated.push(c);
        }
        i+=1;
    }
    drop(rng);

    mutated
}

fn crossover(a: String, b: String) -> String {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..a.len());
    let mut child = String::with_capacity(b.len());
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    for i in 0..a.len() {
        if i < idx {
            child.push(a_bytes[i] as char);
        } else {
            child.push(b_bytes[i] as char);
        }
    }
    child
}

fn fitness(b: String) -> f64 {
    let mut score: f64 = 0.0;
    for c in b.chars() {
        if c == '1' {
            score += 1.0;
        }
    }
    score
}

fn evolve(pop: Vec<String>, p_mut: f32) -> Vec<String> {
    let max_num_fittest = (pop.len()/2) as u64;
    let mut rng = rand::thread_rng();
    let mut curr_pop = pop.clone();

    loop {

        let mut next_pop: Vec<String> = vec![];

        let mut fittest = curr_pop.clone().into_iter().map(|b| (b.clone(), fitness(b.clone()))).collect::<Vec<(String, f64)>>();

        fittest.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        fittest.reverse();

        let avg_fitness = fittest.clone().into_iter().map(|t| t.1).sum::<f64>() / fittest.len() as f64;
        let fittest_individual = fittest[0].0.clone();
        let fittest_val = fittest[0].1;
    
        println!("Current fittest: {} ({}), Average Fitness: {}", fittest_individual, fittest_val, avg_fitness);
        
        if avg_fitness/fittest_val > 0.99 {
            break;
        }

        for _i in 0..pop.len() {
            let p = rng.gen_range(0.0..1.0);
            let idx_a = rng.gen_range(0..max_num_fittest) as usize;
            let idx_b = rng.gen_range(0..max_num_fittest) as usize;
    
            let mut child = crossover(fittest[idx_a].0.clone(), fittest[idx_b].0.clone());
    
            if p < p_mut {
                child = mutate(child);
            }
            next_pop.push(child);
        }
        curr_pop = next_pop.clone();
    }
    drop(rng);
    return curr_pop;
}


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} [SIZE_OF_POPULATION] [SIZE_OF_BIT_STRINGS] [MUTATION_PROBABILITY(0-1)]", &args[0]);
    } else {
        let num = &args[1];
        let size = &args[2];
        let prob = &args[3];

        let n: u64 = match num.parse() {
            Ok(n) => { n },
            Err(_) => {
                eprintln!("error: first argument not an valid size for population");
                return;
            },
        };

        let l: u64 = match size.parse() {
            Ok(l) => { l },
            Err(_) => {
                eprintln!("error: second argument not an valid length for bit strings");
                return;
            },
        };

        let p: f64 = match prob.parse() {
            Ok(p) => { p },
            Err(_) => {
                eprintln!("error: third argument not an valid probability for mutation");
                return;
            },
        };

        if p < 0.0_f64 || p > 1.0_f64 {
            eprintln!("error: third argument not an valid probability for mutation");
            return;
        }

        let pop = generate_random_bits(n, l);
        evolve(pop, 0.01);
    }
}
