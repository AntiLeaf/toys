const p : i64 = 998244353;

fn get_multi_num() -> Vec<i64> {
	let mut s = String::new();
	std::io::stdin().read_line(&mut s).expect("qwq");
	s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn qpow(mut a : i64, mut b : i64) -> i64 {
	let mut ans : i64 = 1;

	while b > 0 {
		if b & 1 != 0 {
			ans = ans * a % p;
		}

		b >>= 1;
		a = a * a % p;
	}

	return ans;
}

fn NTT_init(n : usize) -> Vec<i64> {
	let wn : i64 = qpow(3, (p - 1) / n as i64);
	let mut omega : Vec<i64> = vec![0; n];
	omega[0] = 1;

	for i in 1 .. n {
		omega[i] = omega[i - 1] * wn % p;
	}

	return omega;
}

fn NTT(mut a : Vec<i64>, n : usize, tp : i64, omega : &Vec<i64>) -> Vec<i64> {
	let mut j : usize = 0;
	for i in 1 .. n - 1 {
		let mut k : usize = n;

		loop {
			k >>= 1;
			j = (j as i64 ^ k as i64) as usize;

			if !(j < k) {
				break;
			}
		}

		if i < j {
			let tmp = a[i];
			a[i] = a[j];
			a[j] = tmp;
		}
	}

	let mut k : usize = 2;
	let mut m : usize = n / 2;

	while k <= n {
		let mut i : usize = 0;
		while i < n {
			for j in 0 .. k / 2 {
				let w : i64 = if tp > 0 {omega[m * j]} else {if j == 0 {1} else {omega[n - m * j]}};

				let u : i64 = a[i + j];
				let v : i64 = w * a[i + j + k / 2] % p;

				a[i + j] = u + v;
				if a[i + j] >= p {
					a[i + j] -= p;
				}

				a[i + j + k / 2] = u - v;
				if a[i + j + k / 2] < 0 {
					a[i + j + k / 2] += p;
				}
			}

			i += k;
		}

		k *= 2;
		m /= 2;
	}

	if tp < 0 {
		let inv = qpow(n as i64, p - 2);
		for i in 0 .. n {
			a[i] = a[i] * inv % p;
		}
	}

	return a;
}

fn main() {
	let vec = get_multi_num();
	let n : usize = vec[0] as usize;
	let m : usize = vec[1] as usize;

	let mut N : usize = 1;
	while N <= n + m {
		N *= 2;
	}

	let buf = get_multi_num();
	let mut a = vec![0; N];
	for i in 0 .. n + 1 {
		a[i] = buf[i];
	}

	let buf = get_multi_num();
	let mut b = vec![0; N];
	for i in 0 .. m + 1 {
		b[i] = buf[i];
	}

	let omega : Vec<i64> = NTT_init(N);

	let a : Vec<i64> = NTT(a, N, 1, &omega);
	let b : Vec<i64> = NTT(b, N, 1, &omega);

	let mut c : Vec<i64> = vec![0; N];
	for i in 0 .. N {
		c[i] = a[i] * b[i] % p;
	}

	let c : Vec<i64> = NTT(c, N, -1, &omega);

	for i in 0 .. n + m + 1 {
		print!("{}{}", if i > 0 {" "} else {""}, c[i]);
	}
}
