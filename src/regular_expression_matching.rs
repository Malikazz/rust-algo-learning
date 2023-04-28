// https://leetcode.com/problems/regular-expression-matching/

// output true if match false if not
// Constraintse

//     1 <= s.length <= 20
//     1 <= p.length <= 20
//     s contains only lowercase English letters.
//     p contains only lowercase English letters, '.', and '*'.
//     It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupType {
    Star,
    StarDot,
    Dot,
    Normal,
}
#[derive(Debug, Eq, PartialEq)]
pub struct MatchGroup {
    group_type: GroupType,
    characters: Vec<u8>,
}

// Should your type be considered to be in the other type
trait FoundAt {
    fn found_at(&self, other: &[u8]) -> Vec<bool>;
}

impl FoundAt for MatchGroup {
    fn found_at(&self, other: &[u8]) -> Vec<bool> {
        // [true, false, false, true, false]
        // [true, true ,true ,true ,true]
        // [false, true, true, false true]
        // [true, true, true, true, true]
        //

        let mut matched: Vec<bool> = Vec::new();
        for _ in 0..other.len(){
            matched.push(false);
        }
        match self.group_type {
            GroupType::Dot | GroupType::StarDot => {
                for index in 0..matched.len(){
                    matched[index] = true;
                }
                matched
            }
            GroupType::Star => {
                for (index, &item) in other.iter().enumerate() {
                    if item == self.characters[0] {
                        matched[index] = true;
                    }
                }
                matched
            }
            GroupType::Normal => {
                let mut count_iter: i32 = 0;
                for items in other.windows(self.characters.len()) {
                    if items == self.characters {
                        for index in count_iter..count_iter + (self.characters.len()) as i32 {
                            matched[index as usize] = true;
                        }
                    }
                    count_iter = count_iter + 1 as i32;
                }
                matched
            }
        }
    }
}

impl MatchGroup {
    pub fn new(group_type: GroupType, characters: Vec<u8>) -> Self {
        Self {
            group_type,
            characters,
        }
    }
    pub fn push(mut self, value: u8) {
        self.characters.push(value);
    }
}

pub fn solution(s: String, p: String) -> bool {
    if p.contains(".") || p.contains("*") {
        // split up regex into its parts
        // for each group of regex find its matches over the entire string
        // then compare indexes and see if any contiguous section matched
        // with all of them
        // (adb) (.*) (def)
        //   1    2    1   2    3
        // /adb/adsf/adb/asdf/def/
        // found continguous section at the end
        // valid tokens are ([.]) ([a-z]) ([.][*]) ([a-z][*])
        // star requires a token before it as it acts on the previous token
        let match_groups = create_match_groups(p.into_bytes());
        for item in match_groups.iter(){
            print!("{:?}\n", item);
        }
        let mut match_lists: Vec<Vec<bool>> = Vec::new();
        for item in match_groups.iter() {
            match_lists.push(item.found_at(s.as_bytes()));
        }
        // check rows

        for index in 0..match_lists[0].len(){
            let mut col_booly = false;
            for items in match_lists.iter() {
                if items[index] {
                    col_booly = true;
                }
            }
            if col_booly == false {
                return false;
            }
        }
        true
    } else {
        // if theres no look backs just check the entire string
        s == p
    }
}
// Find regex groups in provided string and return vec of groups
pub fn create_match_groups(p: Vec<u8>) -> Vec<MatchGroup> {
    const STAR: u8 = b'*';
    const DOT: u8 = b'.';
    let mut match_groups: Vec<MatchGroup> = Vec::new();
    let mut current_group: MatchGroup = MatchGroup::new(GroupType::Normal, Vec::new());
    
    for item in p.iter().rev() {
        if *item == STAR {
            if current_group.characters.len() > 0 {
                match_groups.push(current_group);
            }
            current_group = MatchGroup::new(GroupType::Star, vec![*item]);
            continue;
        }
        if *item == DOT {
            if current_group.group_type == GroupType::Star {
                current_group.group_type = GroupType::StarDot;
                current_group.characters.push(*item);
                match_groups.push(current_group);
                current_group = MatchGroup::new(GroupType::Normal, Vec::new());
                continue;
            } else {
                if current_group.characters.len() > 0 {
                    match_groups.push(current_group);
                    current_group = MatchGroup::new(GroupType::Normal, Vec::new());
                }
                current_group.group_type = GroupType::Dot;
                current_group.characters.push(*item);
                match_groups.push(current_group);
                current_group = MatchGroup::new(GroupType::Normal, Vec::new());
                continue;
            }
        }
        match current_group.group_type {
            GroupType::Star => {
                current_group.characters.push(*item);
                match_groups.push(current_group);
                current_group = MatchGroup::new(GroupType::Normal, Vec::new());
            }
            GroupType::Normal => {
                current_group.characters.push(*item);
            }
            GroupType::StarDot => {
                panic!()
            }
            GroupType::Dot => {
                panic!()
            }
        }
    }
    if current_group.characters.len() > 0 {
        match_groups.push(current_group);
    }
    for item in match_groups.iter_mut() {
        item.characters.reverse();
    }
    match_groups.reverse();
    match_groups
}

#[cfg(test)]
mod tests {
    use crate::regular_expression_matching::create_match_groups;
    use crate::regular_expression_matching::solution;
    use crate::regular_expression_matching::GroupType;
    use crate::regular_expression_matching::MatchGroup;
    #[test]
    fn test_create_match_groups() {
        let result = create_match_groups(vec![97, 98, 99, 46, 98, 42, 46, 42, 98, 100, 99]);
        let should_be: Vec<MatchGroup> = vec![
            MatchGroup::new(GroupType::Normal, vec![97, 98, 99]),
            MatchGroup::new(GroupType::Dot, vec![46]),
            MatchGroup::new(GroupType::Star, vec![98, 42]),
            MatchGroup::new(GroupType::StarDot, vec![46, 42]),
            MatchGroup::new(GroupType::Normal, vec![98, 100, 99]),
        ];
        assert!(result == should_be)
    }
    #[test]
    fn test_solution(){
        let result:bool = solution(String::from("abccbbbbbdddd"), String::from("abc.b*dddd"));
        assert!(result == true);
    }
    #[test]
    fn test_solution_two(){
        let result:bool = solution(String::from("abc"), String::from("abc"));
        assert!(result == true);
    }
    #[test]
    fn test_solution_three(){
        let result:bool = solution(String::from("abcc"), String::from("abc."));
        assert!(result == true);
    }
    #[test]
    fn test_solution_four(){
        let result:bool = solution(String::from("abcbbbbb"), String::from("abcb*"));
        assert!(result == true);
    }

    #[test]
    fn test_solution_five(){
        let result:bool = solution(String::from("abcbbbbb"), String::from(".*"));
        assert!(result == true);
    }
}
