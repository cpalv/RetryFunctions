use std::ops::Mul;
use std::thread;
use std::time::Duration;

use rand::prelude::*;

#[derive(Debug)]
enum RetryErr<E> {
    NoAttempts,
    RetryFailed(E),
}

fn retry_fn<F, T, E>(attempts: u8, time_unit: Duration, f: F) -> Result<T, RetryErr<E>>
    where F: Fn() -> Result<T, E> {
          
    let mut power: u32 = 1;
    
    if attempts < 1 {
        return Err(RetryErr::NoAttempts);
    }

    let mut res = f();
    if res.is_ok() {
        return Ok(res.ok().unwrap());
    }

    thread::sleep(time_unit.mul(power));
    power = power << 1;

    for _ in 1..attempts {
        res = f();
        if res.is_ok() {
            return Ok(res.ok().unwrap());
        }

        thread::sleep(time_unit.mul(power));
        power = power << 1;
    }

    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(RetryErr::RetryFailed(e)),
    }
}

fn retry_fn_mut<F, T, E>(attempts: u8, time_unit: Duration, mut f: F) -> Result<T, RetryErr<E>> 
    where F: FnMut() -> Result<T, E> {
    let mut power: u32 = 1;

    if attempts < 1 {
        return Err(RetryErr::NoAttempts);
    }

    let mut res = f();
    if res.is_ok() {
            return Ok(res.ok().unwrap());
    }

    thread::sleep(time_unit.mul(power));
    power = power << 1;

    for _ in 1..attempts {
        res = f();
        if res.is_ok() {
            return Ok(res.ok().unwrap());
        }

        thread::sleep(time_unit.mul(power));
        power = power << 1;
    }

    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(RetryErr::RetryFailed(e)),
    }
}

fn rand_even_num() -> Result<u16, u16> {
    let num = rand::random::<u16>();
    if num % 2 == 0 {
        return Ok(num);
    }

    Err(num)
}

fn rand_odd_num() -> Result<u16, u16> {
    let num = rand::random::<u16>();
    if num % 2 != 0 {
        return Ok(num);
    }

    Err(num)
}

fn print_res<T: std::fmt::Debug, E: std::fmt::Debug>(res: Result<T, RetryErr<E>>) {
    match res {
        Ok(v) => println!("retry successful! got {v:?}"),
        Err(e) => println!("retry errored: {e:?}"),
    }
}


fn main() {
    println!("Hello, world!");

    let x = 3;
    let y = 5;
    let res_ok_mut: Result<u32,_> = retry_fn_mut(3, Duration::from_millis(1), &mut || -> Result<u32,RetryErr<()>>{
        thread::sleep(Duration::from_secs(2));
        let z = x + y;
        Ok(z)
    });

    print_res(res_ok_mut);

    let res_ok: Result<u32,_> = retry_fn(3, Duration::from_millis(1), move || -> Result<u32,RetryErr<()>>{
        //thread::sleep(Duration::from_secs(2));
        Ok(2+4)
    });

    print_res(res_ok);

    // Retry until even number
    let rand_res = retry_fn(3, Duration::from_millis(1), rand_even_num);
    match rand_res {
        Ok(n) => println!("num is even: {n:?}"),
        Err(n) => println!("num is odd: {n:?}"),
    }

    // Retry until odd number
    let rand_res = retry_fn(3, Duration::from_millis(1), rand_odd_num);
    match rand_res {
        Ok(n) => println!("num is odd: {n:?}"),
        Err(n) => println!("num is even: {n:?}"),
    }

    let mut a = 2;
    let mut mv_a = move || -> Result<u32, RetryErr<()>> {
        thread::sleep(Duration::from_secs(2));
        a += 2;
        Ok(a)
    };

    let res_ok: Result<u32,_> = retry_fn_mut(3, Duration::from_millis(1), &mut mv_a);

    print_res(res_ok);
    println!("a = {}", a);

    let mut borrow = || -> Result<u32, RetryErr<()>> {
        thread::sleep(Duration::from_secs(2));
        a += 2;
        Ok(a)
    };

    let res_ok: Result<u32,_> = retry_fn_mut(3, Duration::from_millis(1), &mut borrow);

    print_res(res_ok);
    println!("a = {}", a);

    let err_fn = || {
        thread::sleep(Duration::from_secs(2));
        Err("retry")
    };

    let res_err: Result<(), RetryErr<&str>> = retry_fn(3, Duration::from_millis(1), err_fn);

    print_res(res_err);

    let res_err: Result<(), RetryErr<&str>> = retry_fn(1, Duration::from_millis(1), err_fn);

    print_res(res_err);

    let res_err: Result<(), RetryErr<&str>> = retry_fn(0, Duration::from_millis(1), err_fn);

    print_res(res_err);

    let res_err: Result<(), RetryErr<&str>> = retry_fn_mut(0, Duration::from_millis(1), err_fn);
    print_res(res_err);


    let res_err: Result<(), RetryErr<&str>> = retry_fn_mut(3, Duration::from_millis(1), || {
        thread::sleep(Duration::from_millis(500));
        a += 2;
        println!("a = {}", a);
        Err("retry")
    });
    print_res(res_err);
}
