fn main() {
    let day_str = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gift_str = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese-a-Laying",
        "seven Swans-a-Swimming",
        "eight Maids-a-Milking",
        "nine Ladies Dancing",
        "ten Lords-a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    let mut num: usize = 0;
    while num < 12 {
        println!("On the {} day of Christmas my true love gave to me:", day_str[num]);
        for i in (0..(num + 1)).rev() {
            if i == 0  {
                if num == 0 {
                    println!("{}", gift_str[i]);
                } else {
                    println!("and {}", gift_str[i]);
                }
            } else {
                println!("{}", gift_str[i]);
            }
        }
        num += 1;
        println!("");
    }
}
