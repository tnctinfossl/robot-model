use super::traits::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::*;
//パスカルの三角形を求める
pub struct Pascal {
    memo: RefCell<HashMap<(i32, i32), i32>>,
}

impl Pascal {
    pub fn new() -> Pascal {
        Pascal {
            memo: RefCell::new(HashMap::new()),
        }
    }

    fn find(&self, n: i32, m: i32) -> Option<i32> {
        let memo = self.memo.borrow();
        if let Some(&x) = memo.get(&(n, m)) {
            Some(x)
        } else {
            None
        }
    }

    fn join(&self, n: i32, m: i32, value: i32) {
        let mut memo = self.memo.borrow_mut();
        memo.insert((n, m), value);
    }

    pub fn get(&self, n: i32, m: i32) -> i32 {
        //例外パターン
        if m == 0 || m == n {
            return 1;
        } else if m < 0 || m > n {
            return 0;
        }
        //既知の場合
        if let Some(x) = self.find(n, m) {
            return x;
        }
        //新規の場合
        let y = self.get(n - 1, m - 1) + self.get(n - 1, m);
        self.join(n, m, y);
        y
    }
    //組み合わせ　(a b)^n
    pub fn combinate(&self, a: f32, b: f32, n: i32) -> Vec<f32> {
        let mut result = Vec::new();
        for m in 0..=n {
            result.push(self.get(n, m) as f32 * a.powi(n - m) * b.powi(m));
        }
        result
    }

    //微分用の配列
    pub fn delta(&self, n: i32) -> Vec<f32> {
        let mut result = Vec::new();
        for m in 0..=n {
            if m & 0x1 == 0x1 {
                result.push(-self.get(n, m) as f32);
            } else {
                result.push(self.get(n, m) as f32);
            }
        }
        result
    }
}
