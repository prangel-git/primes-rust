use std::collections::HashMap;

fn sieve(max_number : u64) -> Vec<u64>{

  let mut to_cross: HashMap<u64, u64> = HashMap::new();
  let mut primes = Vec::new();
  primes.push(2);
  
  for num in (3..max_number).step_by(2){
    if let Some(step) = to_cross.get(&num){
      
      let real_step = *step;
      let mut next = num + real_step;
      
      while to_cross.contains_key(&next){
        next += real_step;
      }
      
      if next <= max_number {
        to_cross.insert(next, real_step);
      }
      
      to_cross.remove(&num);
    } else {
      
      primes.push(num);
      
      let key = num * num;
      let value = 2 * num;
      
      to_cross.insert(key, value);
    }
  }
  return primes;
}

fn main() {
  let primes = sieve(100);
  println!("{:?}", primes);
  println!("{}", primes.len());
}
