use std::marker::PhantomData;
use std::mem::swap;

struct Uninitialized;
struct Suspended;
struct Completed;

struct PrimesCoroutine<State = Uninitialized>{
    primes : Vec<u64>,
    current : u64,
    state : std::marker::PhantomData<State>,
}

impl PrimesCoroutine::<Uninitialized>{
    fn init(self) -> Result<(u64, PrimesCoroutine<Suspended>), PrimesCoroutine<Completed>>{
        Ok((
            2,
        PrimesCoroutine{
            primes : self.primes,
            current : 2,
            state : PhantomData,
        }
        ))
    }
}

impl PrimesCoroutine<Suspended>{
    fn resume(mut self) -> Result<(u64, PrimesCoroutine<Suspended>), PrimesCoroutine<Completed>>{
        self.primes.push(self.current);
        while self.current < u64::MAX{
            self.current += 1;
            println!("testing {}",self.current);
            let mut found : bool = true;
            for prime in self.primes.iter(){
                if prime * prime > self.current{
                    println!("early interruption for square rule {}",prime);
                    break;
                }
                if self.current % prime == 0 {
                    println!("early interruption for division with {}",prime);
                    found = false;
                    break;
                }
            }
            if found {
                println!("found {}",self.current);
                return Ok(
                    (self.current
                    ,self)
                )
            }
        }

        Err(
            PrimesCoroutine{
                primes : self.primes,
                current : 0,
                state : PhantomData
            }
        )
    }
}

impl<T> PrimesCoroutine<T>{
    fn get_primes(& self) -> & Vec<u64>{
        &self.primes
    }
}

impl PrimesCoroutine{
    fn new() -> PrimesCoroutine<Uninitialized>{
        PrimesCoroutine{
            primes : Vec::new(),
            current : 2,
            state : PhantomData,
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

enum CoroutineStatus{
    Undefined,
    Created(PrimesCoroutine<Uninitialized>),
    Ready(PrimesCoroutine<Suspended>),
    Closed(PrimesCoroutine<Completed>)
}
use CoroutineStatus::*;

impl Default for CoroutineStatus {
    fn default() -> Self { Undefined }
}

struct Prime{
    coroutine : CoroutineStatus
}

impl Iterator for Prime{
    type Item = u64;
    fn next(& mut self) -> Option<Self::Item>{
        let mut coroutine = & CoroutineStatus::default();
        swap(& mut coroutine, self.coroutine);
        match coroutine.next(){
            (status, Some(value)) => {
                self.coroutine = status;
                Some(value)
            },
            (status, None) => {
                self.coroutine = status;
                None
            }
        }
    }
}

impl CoroutineStatus{
    fn next(self) -> (CoroutineStatus, Option<u64>){
        match self{
            Created(coroutine) => {
                match coroutine.init(){
                    Ok((result, coroutine)) =>{
                        ( Ready(coroutine),
                          Some(result))
                    },
                    Err(coroutine) => {
                        ( Closed(coroutine),
                        None)
                    }
                }
            }
,
            Ready(coroutine) => {
                match coroutine.resume(){
                    Ok((result, coroutine)) =>{
                        ( Ready(coroutine),
                          Some(result))
                    },
                    Err(coroutine) => {
                        ( Closed(coroutine),
                        None)
                    }
                }
            },
            _ => (self, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let primes = PrimesCoroutine::new();
        if let Ok((_result, primes)) = primes.init(){
            let result = primes.resume();
            match result{
                Ok((value,_)) => {assert_eq!(value,3)}
                Err(_) => {panic!("closed stream")}
            }
        }
    }
}
