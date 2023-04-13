#[path = "keyword.rs"]
pub mod keyword;
use keyword::{*};

pub struct Keyword_List<'a> {

    _cdr: Option<&'a mut Keyword_List<'a>>,
    _car: Option<&'a mut Keyword<'a>> 
}


impl<'a> Default for Keyword_List<'a> {
    fn default() -> Keyword_List<'a> {
        Keyword_List {
            _cdr: None,
            _car: None
        }
    }
}


impl<'a> Keyword_List<'a> {

    // pub fn new(car: &mut Keyword) -> Keyword_List {
    //     Keyword{_cdr: None, _car: car}
    // }

    #[inline]
    pub const fn first(&self) -> Option<&mut Keyword> {

        return Some(self._car.unwrap());

    }

    #[inline]
    pub fn rest(&self) -> &mut Option<&mut Keyword_List<'a>> {

        return &mut Some(self._cdr.unwrap());

    }
    
}


pub struct KeywordExt_List<'a> {

    pub keyword_list: Keyword_List<'a>

}

impl<'a> Default for KeywordExt_List<'a> {
    fn default() -> KeywordExt_List<'a> {
        KeywordExt_List {
            keyword_list: Keyword_List{..Default::default()}
        }
    }
}

impl<'a> KeywordExt_List<'a> {


    // pub fn new(car: Option<&mut KeywordExt>) -> KeywordExt_List {

    //     KeywordExt_List {
    //         keyword_list: Keyword_List{_car: car.keyword, ..Default::default()}
    //     } 
    
    // }

    #[inline]
    pub const fn first(&self) -> Option<&mut KeywordExt> {

        return Some(&mut KeywordExt{keyword: *self.keyword_list._car.unwrap(), _final_index: -1, ..Default::default()});

    }

    #[inline]
    pub fn rest(&self) -> &mut Option<&mut KeywordExt_List<'a>> {

        return &mut Some(&mut KeywordExt_List{keyword_list: Keyword_List{_cdr: self.keyword_list._cdr, ..Default::default()}}); 

    } 

}

pub fn copy_list<'a>(list: Option<&'a mut Keyword_List<'a>>) -> Option<&mut Keyword_List<'a>> {

    let result: Option<&mut Keyword_List>;
    let lastp: &mut Option<&mut Keyword_List> = &mut result;

    while !list.is_none() {
        
        let new_cons: Keyword_List = Keyword_List{_car: list.unwrap().first(), ..Default::default()};
        *lastp = Some(&mut new_cons);
        lastp = new_cons.rest();
        list = *list.unwrap().rest();
    }

    *lastp = None;
    return result;
}


pub fn copy_extlist<'a>(list: Option<&'a mut KeywordExt_List<'a>>) -> Option<&'a mut KeywordExt_List<'a>> {
    return Some(&mut KeywordExt_List{keyword_list: *copy_list(Some(&mut list.unwrap().keyword_list)).unwrap()});
}


pub fn delete_list(list: Option<&mut Keyword_List>) {

    while !list.is_none() {
        let rest: &mut Option<&mut Keyword_List> = list.unwrap().rest();
        std::mem::drop(list);
        list = *rest;
    }

}

type Keyword_Comparison = fn(keyword1: Option<&mut Keyword>, keyword2: Option<&mut Keyword>) -> bool;

pub fn merge<'a>(list1: Option<&'a mut Keyword_List<'a>>, list2: Option<&'a mut Keyword_List<'a>>, less: Keyword_Comparison) -> Option<&'a mut Keyword_List<'a>> {

    let result: Option<&mut Keyword_List>;
    let resultp: &mut Option<&mut Keyword_List> = &mut result;

    while true {

        if !list1.is_none() {
            *resultp = list2;
            break;
        } 

        if !list2.is_none() {
            *resultp = list1;
            break;
        }

        if less(list2.unwrap().first(), list1.unwrap().first()) {

            *resultp = list2;
            resultp = &mut list2.unwrap().rest();
            list2 = list1;
            list1 = *resultp;

        } else {

            *resultp = list1;
            resultp = &mut list1.unwrap().rest();
            list1 = *resultp;

        }  


    }

    return result;

}


pub fn mergesort_list<'a>(list: Option<&'a mut Keyword_List<'a>>, less: Keyword_Comparison) -> Option<&'a mut Keyword_List<'a>> {

    if list.is_none() || list.unwrap().rest().is_none() {
        return list;
    } else {
        let middle: &mut Option<&mut Keyword_List>;
        let temp: &mut Option<&mut Keyword_List>;
        while true {
            temp = list.unwrap().rest();
            if !(*temp).is_none() {
                break;
            }

            temp = (*temp).unwrap().rest();
            middle = (*middle).unwrap().rest();
            
            if !(*temp).is_none() {
                break;
            }
        }

        let right_half: &mut Option<&mut Keyword_List> = (*middle).unwrap().rest();
        *((*middle).unwrap().rest()) = None;

        return merge(mergesort_list(list, less), mergesort_list(*right_half, less), less);
    }

}

pub fn mergesort_extlist<'a>(list: Option<&'a mut KeywordExt_List<'a>>, less: fn(keyword1: Option<&mut Keyword>, keyword2: Option<&mut Keyword>) -> bool) -> Option<&'a mut KeywordExt_List<'a>> {

    return Some(&mut KeywordExt_List{keyword_list: *mergesort_list(Some(&mut list.unwrap().keyword_list), less as Keyword_Comparison).unwrap(), ..Default::default()});

}