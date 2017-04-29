

pub struct Permutation {
    elements: Vec<usize>,
    swappings: Vec<usize>,
    valid: bool
}

impl Permutation {
    pub fn new(elements: &Vec<usize>, num: usize) -> Permutation {
        Permutation {
            elements: elements.clone(),
            swappings: (0..num).collect::<Vec<_>>(),
            valid: true
        }
    }
}

impl Iterator for Permutation {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if !self.valid {
            return None
        }

        let res = self.elements
            .iter()
            .take(self.swappings.len())
            .map(|n| n.clone())
            .collect::<Vec<_>>();

        let mut i: isize = self.swappings.len() as isize - 1;

        while i >= 0 && self.swappings[i as usize] == self.elements.len() - 1 {
            self.elements.swap(i as usize, self.swappings[i as usize]);
            self.swappings[i as usize] = i as usize;
            i -= 1;
        }


        if i < 0 {
            self.valid = false
        }else {
            let prev = self.swappings[i as usize];
            self.elements.swap(i as usize, prev);
            let next = prev + 1;
            self.swappings[i as usize] = next;
            self.elements.swap(i as usize, next);
        }

        Some(res)
    }

}