pub fn fibonacci(n: i32) -> u64
{
    if n < 0
    {
       panic!("{} negativo!",n);
    }
    match n < 0 
    {
      0 => panic!("zero nao e um argumento certo para fibonacci()!")
      1 | 2 => 1,
      3 => 2,

      
        => fibonacci(n -1) + fibonacci(n - 2)

    }
}

pub fn fibo(n: i32) -> u64
{
    if n < 0
    {
       panic!("{} negativo!",n);
    }  else if n == 0 {
		panic!("zero is not a right argument to fibonacci()!");
	} else if n == 1 {
		return 1;
	}

	let mut soma = 0;
	let mut ultimo = 0;
	let mut atual = 1;
	for _i in 1..n {
		soma = ultimo + atual;
		ultimo = atual;
		atual = soma;
	}
	soma
}



pub struct fibo 
{
    ultimo : u64,
    prox : u64,
}

impl iterator for fibo
{
    type item = u64;
    fn prox(&mut self) -> option<u64>
    {
        let novo_prox = self.atual + self.prox;

        self.atual = self.prox;
        self.prox = novo_prox;
        Some(self.atual)
    }
}

pub fn fibo_interativo() -> fibo
{
    fibo{atual:1, prox:1}

}

#[cfg(test)]
fn rg_024_x(n: i32, expected: u64)
{
    let mut achar = fibonacci(n);
    assert!(expected == achar, format!("fibonacci({}): expected ({}) != achar ({})",n, expected,achar));
    achar = fibo(n);
    assert!(expected == achar, format!("fibo({}): expected ({}) != achar ({})",n, expected,achar));
}

#[test]
fn rg_024_1() {
	rg_024_x(1, 1);
}

#[test]
fn rg_024_2() {
	rg_024_x(2, 1);
}

#[test]
fn rg_024_3_a() {
	rg_024_x(3, 2);
}

#[test]
#[should_panic]
fn rg_024_4_a() {
	fibo(-1);
	fibo(0);
}

#[test]
fn rg_024_5() {
	/*rg_024_x(55, 139583862445);*/
	rg_024_x(30, 832040);
}

#[cfg(feature="nightly")]
#[cfg(test)]

mod bench
{
    extern crate test;
    use self::test::Bencher;
    use super::*;

    #[cfg(test)]
    static BENCH_SIZE: usize = 20;
    #[cfg(test)]
    static BENCH_SIZE_INT: i32 = 20;

    #[bench]
    fn bench_fibonacci_reccursive(b: &mut Bencher) {
        b.iter(|| {
            fibonacci(BENCH_SIZE_INT);
        });
    }
    #[bench]
    fn bench_fibonacci(b: &mut Bencher) {
        b.iter(|| {
            fibo(BENCH_SIZE_INT);
        });
    }

    #[bench]
    fn bench_iterative_fibonacci(b: &mut Bencher) {
        b.iter(|| {
            // http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take
            fibo_interativo().take(BENCH_SIZE).last().unwrap()
            //iterative_fibonacci().take(BENCH_SIZE).collect::<Vec<u64>>()
        })
    }

    
}
