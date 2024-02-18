////////////////////////////////////////////////////////////////////////////////

#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct PriorityQueue<P,T,K=()> {
    num : usize,
    buffer : Vec<(P,T,K)>,
}

impl<P:Ord,T,K:Eq> PriorityQueue<P,T,K> {
    fn bubble_down(&mut self, n: usize) -> usize {
        let mut n = n;
        let mut m = n*2;
        while m < self.num {
            if (m+1 < self.num) && (self.buffer[m-1].0 > self.buffer[m+1-1].0) {
                m += 1;
            }
            if self.buffer[n-1].0 <= self.buffer[m-1].0 {
                break;
            }
            self.buffer.swap(n-1, m-1);
            n = m;
            m = n*2;
        }
        return n;
    }

    fn bubble_up(&mut self, n: usize) -> usize {
        let mut n = n;
        let mut m = n/2;
        while (m != 0) && (self.buffer[n-1].0 < self.buffer[m-1].0) {
            self.buffer.swap(n-1, m-1);
            n = m;
            m = n/2;
        }
        return n;
    }

    pub fn new() -> Self {
        PriorityQueue { num:1, buffer:Vec::new() }
    }

    pub fn clear(&mut self) {
        self.num = 1;
    }

    pub fn len(&self) -> usize {
        self.num - 1
    }

    pub fn push(&mut self, priority:P, data:T, key:K) {
        self.buffer.push((priority,data,key));
        let num = self.num;
        self.bubble_up(num);
        self.num += 1;
    }

    pub fn pop(&mut self) -> Option<(P,T,K)> {
        let num = self.num;
        if num == 1 {
            None
        } else {
            self.num -= 1;
            self.buffer.swap(1-1, num - 1-1);
            self.bubble_down(1);
            self.buffer.pop()
        }
    }

    pub fn peek_priority(&self) -> Option<&P> {
        if self.num == 1 {
            None
        } else {
            Some(&self.buffer[1-1].0)
        }
    }

    pub fn peek(&self) -> Option<&(P,T,K)> {
        if self.num == 1 {
            None
        } else {
            Some(&self.buffer[1-1])
        }
    }

    fn remove_idx(&mut self, idx:usize) -> Option<(P,T,K)> {
        if idx == 0 { return None; }
        self.num -= 1;
        self.buffer.swap(idx-1, self.num - 1-1);
        self.bubble_down(idx-1);
        self.buffer.pop()
    }

    fn find_idx(&self, k:&K) -> usize {
        for i in 1 .. self.num {
            if &self.buffer[i-1].2 == k {
                return i;
            }
        }
        return 0;
    }

    pub fn contains_key(&self, k:&K) -> bool {
        self.find_idx(k) != 0
    }

    pub fn change_priority(&mut self, k:&K, mut priority:P) -> Option<P> {
        if let Some((mut p,t,k)) = self.remove(k) {
            std::mem::swap(&mut priority, &mut p);
            self.push(p,t,k);
            Some(priority)
        } else {
            None
        }
    }

    pub fn remove(&mut self, k:&K) -> Option<(P,T,K)> {
        let idx = self.find_idx(k);
        if idx == 0 { return None; }
        self.remove_idx(idx)
    }

    pub fn walk_iter(&self) -> impl Iterator<Item=&(P,T,K)> {
        PriorityQueueWalkIterator { pq:self, idx:0 }
    }
}

////////////////////////////////////////

pub struct PriorityQueueWalkIterator<'a,P:'a,T:'a,K:'a> {
    pq: &'a PriorityQueue<P,T,K>,
    idx: usize,
}

impl<'a,P:Ord,T,K:Eq> Iterator for PriorityQueueWalkIterator<'a,P,T,K> {
    type Item = &'a (P,T,K);
    fn next(&mut self) -> Option<&'a(P,T,K)> {
        if self.idx+1 >= self.pq.num {
            None
        } else {
            self.idx += 1;
            Some(&self.pq.buffer[self.idx-1])
        }
    }
}

////////////////////////////////////////

pub struct PriorityQueueDrainIterator<'a,P:'a,T:'a,K:'a> {
    pq: &'a mut PriorityQueue<P,T,K>,
}

impl<'a,P:Ord,T,K:Eq> Iterator for PriorityQueueDrainIterator<'a,P,T,K> {
    type Item = (P,T,K);
    fn next(&mut self) -> Option<(P,T,K)> {
        self.pq.pop()
    }
}

impl<'a,P:Ord,T,K:Eq> IntoIterator for &'a mut PriorityQueue<P,T,K> {
    type Item = (P,T,K);
    type IntoIter = PriorityQueueDrainIterator<'a,P,T,K>;
    fn into_iter(self) -> Self::IntoIter {
        PriorityQueueDrainIterator { pq: self }
    }
}

////////////////////////////////////////////////////////////////////////////////
