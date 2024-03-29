use std::marker::PhantomData;
use std::mem::take;

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
            let mut found : bool = true;
            for prime in self.primes.iter(){
                if prime * prime > self.current{
                    // early interruption for square rule
                    break;
                }
                if self.current % prime == 0 {
                    // early interruption for division
                    found = false;
                    break;
                }
            }
            if found {
                // this is a prime number
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

impl Prime{
    fn new() -> Prime{
        Prime{
            coroutine: Created(PrimesCoroutine::new())
        }
    }
}

impl Iterator for Prime{
    type Item = u64;
    fn next(& mut self) -> Option<Self::Item>{
        let coroutine = take(& mut self.coroutine);
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


    #[test]
    fn iter_test(){
        let mut result : Vec<u64>= Vec::new();
        let prime = Prime::new();
        for p in prime.into_iter(){
            result.push(p);
            if p > 20{
                break;
            }
        }
        let expected : Vec<u64> = vec!(2u64,3u64,5u64,7u64,11u64,13u64,17u64,19u64,23u64);
        assert_eq!(expected, result);
    }
}
