use std::io::*;

pub trait RustGymRead: BufRead {
    fn parse_line<T>(&mut self) -> T
    where
        Self: Sized,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let line = self.lines().next().unwrap().unwrap();
        line.trim().parse::<T>().unwrap()
    }

    fn parse_vec<T>(&mut self) -> Vec<T>
    where
        Self: Sized,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let line = self.lines().next().unwrap().unwrap();
        line.split_whitespace()
            .map(|s| s.parse::<T>().unwrap())
            .collect()
    }

    fn parse_mat<T>(&mut self, n: usize) -> Vec<Vec<T>>
    where
        Self: Sized,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut res: Vec<Vec<T>> = vec![];
        let mut it = self.lines();
        for _ in 0..n {
            let line = it.next().unwrap().unwrap();
            let row = line
                .split_whitespace()
                .map(|s| s.parse::<T>().unwrap())
                .collect();
            res.push(row);
        }
        res
    }

    fn collect_string(&mut self) -> String
    where
        Self: Sized,
    {
        self.lines().next().unwrap().unwrap()
    }

    fn collect_vec(&mut self) -> Vec<String>
    where
        Self: Sized,
    {
        let line = self.lines().next().unwrap().unwrap();
        line.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn collect_mat(&mut self, n: usize) -> Vec<Vec<String>>
    where
        Self: Sized,
    {
        let mut res: Vec<Vec<String>> = vec![];
        let mut it = self.lines();
        for _ in 0..n {
            let line = it.next().unwrap().unwrap();
            let row = line.split_whitespace().map(|s| s.to_string()).collect();
            res.push(row);
        }
        res
    }
}

impl<R: BufRead> RustGymRead for R {}
