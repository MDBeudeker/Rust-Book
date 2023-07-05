fn main() {
    
    // 1, fahrenheit -> Celcius
    let temperature: f32 = 23.0;
    let answer: f32 = fahrenheit_to_celcius(temperature);
    println!("{} degrees fahrenheis = {} degrees celsius\n\r",temperature, answer);

    // 2, Fibonnaci
    let input: u32 = 1;
    let answer: u32 = fibonnaci(input);
    println!("The {input}'th fibonnaci number is {answer}\n \r");

    // days of christmase
    days_of_christmas();
}

// function to convert fahrenheit to celsius

fn fahrenheit_to_celcius (temperature : f32) -> f32{
    (temperature - 32.0) * (5.0/9.0)
}

fn fibonnaci (input: u32) -> u32{
    let mut answer: u32 = 0;
    let mut answer_n: u32=0;
    for _x in 1..input{
        answer_n+=answer;
        answer= answer_n;
        if answer == 0 {answer_n +=1} // make sure the 1st fibonacci number has A value since else is cannot iterate
        println!("{answer_n}");
        
    }

    answer // return calculated answer
}

fn days_of_christmas(){
    let things=["a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming"];

    let mut index = 1; // to record the number of iterations over each day
    
    for _day in things{

        let mut jndex = index; // make j as long as i for the j-teration
        
        let count_word; // to change the count word from 1th to 1st and 2th to 3nd
        if index ==1 { count_word = "st";}
        else if index ==2  { count_word = "nd"; }
        else if index ==3  { count_word = "rd"; }
        else { count_word = "th";}

        // The main line of the song:
        println!("\n\rOn the {}{} day of christmas my true love gave to me ", index, count_word);

        // The 2nd/jteration through all the 'things' we got for christmas
        for _x in 0..index{
            if jndex ==2{
                println!("{}, and", things[jndex-1]);
            }
            else {
                println!("{}", things[jndex-1]);
            }
            
            jndex -=1; // we go backwards through the summation, so subtract from jndex
        }
        index +=1;
    }
}