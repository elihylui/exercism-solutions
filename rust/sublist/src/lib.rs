use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Display>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //direct comparison between 2 lists
    if _first_list == _second_list {
        return Comparison::Equal;
    } else {
        //turn the list to a string and use "contains"
        let _first_list:String = _first_list.iter().map(|x| x.to_string()).collect();
        let _second_list:String = _second_list.iter().map(|x| x.to_string()).collect();
        if _first_list.contains(&_second_list) {
            return Comparison::Superlist;
        } else if _second_list.contains(&_first_list) {
            return Comparison::Sublist;
        } else {
            Comparison::Unequal
        }
    }
}
