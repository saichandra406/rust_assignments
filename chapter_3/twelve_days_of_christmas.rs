const NUMBERS: [&str; 11] =  [ "Two", "Three", "Four", "Five", "Six", "Seven",
                 "Eight", "Nine", "Ten", "Eleven", "Twelve"];

const ITEMS: [&str; 12] = ["partridge in a pear tree", " turtle doves", " French hens",
             "calling birds", "gold rings", "geese a-laying",
             "swans a-swimming", "maids a-milking", "ladies dancing",
             "lords a-leaping", "pipers piping", "drummers drumming"];

const DAY_NUM: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth",
               "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

fn print_stanza(n: usize) {
    let mut end =  ',';
    let mut start: &str;
    let one_begin = if n == 0 {"A "}
                    else {"And a "};
    for i in (0..n+1).rev() {
        if i == 0 {
            end = '.';
            start = one_begin;
        }
        else{
            start = NUMBERS[i - 1];
        }
        println!("{} {}{}", start.trim(), ITEMS[i].trim(), end);
    }
    println!("");
}

fn main() {
    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", DAY_NUM[i]);
        print_stanza(i);
    }
}
