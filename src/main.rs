use std::env;
use rayon::prelude::*;
use std::sync::Mutex;

fn fact(n: u64) -> u64 {
    (2..=n).fold(1,|r,n| r*n)
}

#[derive(Debug)]
struct Selector {
    blocks: [u8;32],
    size:   usize
}

impl Selector {
    fn new(size: usize) -> Selector {
        let mut blocks: [u8;32] = [0;32];
        for n in 0..size {
            blocks[n] = (n+1) as u8;
        }
        Selector{ blocks, size }
    }

    fn select(&self, n: u64) -> (u64,(i32,i32)) {
        let mut blk: [u8;32] = self.blocks;
        let mut res: u64 = 0;
        let mut num = n;
        let mut i = self.size as u64;
        let mut pos_a = -1;
        let mut pos_b = -1;
        let mut p_cnt = 0;
        while i>0 {
            let idx = (num % i) as usize;
            num = num / i;
            let block = blk[idx] as u64;
            res = if block > 9 {
                if block == 11 {
                    pos_b = p_cnt;
                }
                p_cnt += 2;
                res * 100 + block
            } else {
                if block == 1 {
                    pos_a = p_cnt;
                }
                p_cnt += 1;
                res * 10 + block
            };
            for m in idx..(i as usize) {
                blk[m] = blk[m+1];
            }
            i -= 1;
        }
        (res,(p_cnt-pos_a-1,p_cnt-pos_b-2))
    }
}

fn min_brk(min: u64, digits: u64, atomic: (i32,i32)) -> Option<u64>
{
    let mut denom = 10;
    let mut r: Option<u64> = None;
    let mut pos = 1;
    while digits > denom {
        let a = digits / denom;
        let b = digits % denom;
        if a < min {
            break;
        }
        // println!("a:{} b:{} p:{} atom:{:?} min: {}",a,b,pos,atomic, min);
        if b > a && ((a%10)!=1 || pos==atomic.0 || pos==atomic.1 ) {
            match min_brk(a,b,atomic) {
                Some(v) => { let part = a + v;
                             if let Some(cmin) = r {
                                 if part < cmin {
                                     r = Some(part);
                                 }
                             } else {
                                 r = Some(part);
                             }
                },
                None => ()
            }
        }
        denom = denom * 10;
        pos += 1;
    }
    if r.is_none() {
        r = Some(digits);
    }
    // println!("min_brk( {}, {} ) -> {:?}", min, digits, r);
    r
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} num", args[0]);
        return;
    }
    let num = match args[1].parse::<u64>() {
        Ok(n)  => n,
        Err(e) => { println!("Could not parse number {:?}.", e); return; }
    };
    println!("{:?}", min_brk(0,3527101198641,(0,5)));
    let cmb = fact(num);
    let s = Selector::new(num as usize);

    let max_v: Mutex<u64> = Mutex::new(0);
    let best = (0..cmb).into_par_iter().
        map(|i| s.select(i)).fold(|| 0,|max,(v,ap)| {
            let sum = min_brk(0, v, ap).unwrap();
            if sum >= max {
                {
                    let mut mv = max_v.lock().unwrap();
                    if sum >= *mv {
                        println!("{} {} (atomic: {:?})", sum, v, ap);
                        *mv = sum;
                    }
                    *mv // Return mutex protected max value
                }
            } else {
                max
            }}).
        reduce(|| 0, |p: u64, n: u64|  p.max(n) );

    println!("best: {:?}", best);
}
