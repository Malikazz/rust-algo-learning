pub fn is_anagram(mut s: String, mut t: String) -> bool {
        if s.len() != t.len(){
            return false;
        }
        let mut t_len = t.len();
        for item in s.chars(){
            let index = t.find(item).unwrap_or(usize::MAX);
            if index == usize::MAX{
                return false;
            }
            t.remove(index);
            if t.len() < t_len{
                t_len = t.len();
            }else {
                return false
            }
        }
        true
    }
