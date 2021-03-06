/* MIT License
 *
 * Copyright (c) 2021 Brighton Sikarskie
 *  
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use rand::Rng;
use std::io::prelude::*;

fn main() {
    println!(
        "Please think of a number between 1 and 100 (inclusive)\n\
              .... Now, please answer lower, higher, or yes, based on my questions."
    );
    binary_search(1, 100);
}

fn binary_search(lower_bound: u16, upper_bound: u16) {
    if lower_bound > upper_bound {
        println!("You must have cheated!");
        return;
    }
    let mut guess: u16 = lower_bound;
    if lower_bound < upper_bound {
        guess = rand::thread_rng().gen_range(lower_bound, upper_bound);
    }
    print!("Is your number {}? ", guess);
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    match response.as_str() {
        "higher\n" => binary_search(guess + 1, upper_bound),
        "lower\n" => binary_search(lower_bound, guess - 1),
        _ => {
            println!("I win!");
            return;
        }
    }
}
