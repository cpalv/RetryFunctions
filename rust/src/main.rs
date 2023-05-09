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
where
    F: Fn() -> Result<T, E>,
{
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

        println!("power = {}", power);
        thread::sleep(time_unit.mul(power));
        // circular shift
        // shift the leftmost bit back to the rightmost bit
        // of an unsigned 32-bit integer should the lefthand
        // side of the bitwise-or operator result in 0.
        power = power << 1 | power >> 31;
    }

    res.map_err(|e| RetryErr::RetryFailed(e))
}

fn retry_fn_mut<F, T, E>(attempts: u8, time_unit: Duration, mut f: F) -> Result<T, RetryErr<E>>
where
    F: FnMut() -> Result<T, E>,
{
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
        power = power << 1 | power >> 31;
    }

    res.map_err(|e| RetryErr::RetryFailed(e))
}

fn rand_even_num() -> Result<u16, u16> {
    /*let num = rand::random::<u16>();
    if num % 2 == 0 {
        return Ok(num);
    }

    Err(num)*/
    even_bit()
}

fn even_bit() -> Result<u16, u16> {
    let num = rand::random::<u16>();
    if (num & 0) == 0 {
        return Ok(num);
    }

    Err(num)
}

fn rand_odd_num() -> Result<u16, u16> {
    /*let num = rand::random::<u16>();
    if num % 2 != 0 {
        return Ok(num);
    }

    Err(num)*/
    odd_bit()
}

fn odd_bit() -> Result<u16, u16> {
    let num = rand::random::<u16>();
    if (num & 1) != 0 {
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
    println!("u8 max: {}", u8::MAX);
    println!("u16 max: {}", u16::MAX);

    let x = 3;
    let y = 5;
    println!("*x = {:p}, *y = {:p}", &x, &y);

    let capxy = || -> Result<u32, RetryErr<()>> {
        thread::sleep(Duration::from_secs(2));
        println!("*x = {:p}, *y = {:p}", &x, &y);
        let z = &x + &y;
        Ok(z)
    };

    let res_ok_mut: Result<u32, _> = retry_fn_mut(3, Duration::from_millis(1), capxy);
    print_res(res_ok_mut);

    let res_ok: Result<u32, _> = retry_fn(10, Duration::from_millis(1), capxy);
    print_res(res_ok);

    let res_ok: Result<u32, _> = retry_fn(
        3,
        Duration::from_millis(1),
        move || -> Result<u32, RetryErr<()>> {
            //thread::sleep(Duration::from_secs(2));
            Ok(2 + 4)
        },
    );

    print_res(res_ok);

    // Retry until even number
    let rand_res = retry_fn(3, Duration::from_millis(1), rand_even_num);
    match rand_res {
        Ok(n) => println!("num is even: {n:?}"),
        Err(n) => println!("num is odd: {n:?} wanted even"),
    }

    // Retry until odd number
    let rand_res = retry_fn(3, Duration::from_millis(1), rand_odd_num);
    match rand_res {
        Ok(n) => println!("num is odd: {n:?}"),
        Err(n) => println!("num is even: {n:?} wanted odd"),
    }

    let mut a = 2;
    let mut mv_a = move || -> Result<u32, RetryErr<()>> {
        thread::sleep(Duration::from_secs(2));
        a += 2;
        Ok(a)
    };

    let res_ok: Result<u32, _> = retry_fn_mut(3, Duration::from_millis(1), &mut mv_a);

    print_res(res_ok);
    println!("a = {}", a);

    let mut borrow = || -> Result<(), RetryErr<()>> {
        thread::sleep(Duration::from_secs(2));
        a += 2;
        Ok(())
    };

    let res_ok: Result<(), _> = retry_fn_mut(3, Duration::from_millis(1), &mut borrow);

    print_res(res_ok);
    println!("a = {}", a);

    let err_fn = || {
        thread::sleep(Duration::from_secs(2));
        Err("retry")
    };

    let mut res_err: Result<(), RetryErr<&str>> = retry_fn(3, Duration::from_millis(1), err_fn);
    print_res(res_err);

    res_err = retry_fn(1, Duration::from_millis(1), err_fn);
    print_res(res_err);

    res_err = retry_fn(12, Duration::from_millis(1), err_fn);
    print_res(res_err);

    res_err = retry_fn(35, Duration::from_nanos(1), err_fn);
    print_res(res_err);

    res_err = retry_fn(0, Duration::from_millis(1), err_fn);
    print_res(res_err);

    res_err = retry_fn_mut(0, Duration::from_millis(1), err_fn);
    print_res(res_err);

    res_err = retry_fn_mut(3, Duration::from_millis(1), || {
        thread::sleep(Duration::from_millis(500));
        a += 2;
        println!("a = {}", a);
        Err("retry")
    });
    print_res(res_err);
    println!("a = {}", a);
}
