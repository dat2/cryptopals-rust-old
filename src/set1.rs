use std::iter;
use std::collections::HashMap;
use std::ascii::AsciiExt;
use std::f64;

fn hex_to_vec(input: &[u8]) -> Vec<u8> {
  // ranges for characters
  const NUMERIC_BEGIN: u8 = '0' as u8;
  const NUMERIC_END: u8 = '9' as u8;
  const LOWER_LETTER_BEGIN: u8 = 'a' as u8;
  const LOWER_LETTER_END: u8 = 'f' as u8;
  const UPPER_LETTER_BEGIN: u8 = 'A' as u8;
  const UPPER_LETTER_END: u8 = 'F' as u8;

  let mut output = Vec::new();

  let mut word = 0;
  let mut is_msb = true;

  for byte in input {
    // convert the character data to the numeric representation
    let true_number = match *byte {
      NUMERIC_BEGIN ... NUMERIC_END => byte - NUMERIC_BEGIN,
      LOWER_LETTER_BEGIN ... LOWER_LETTER_END => byte - LOWER_LETTER_BEGIN + 10,
      UPPER_LETTER_BEGIN ... UPPER_LETTER_END => byte - UPPER_LETTER_BEGIN + 10,
      _ => 0
    };

    word = if is_msb { word ^ (true_number << 4) } else { word ^ true_number };
    if !is_msb {
      output.push(word);
      word = 0;
    }
    is_msb = !is_msb;
  }
  output
}

fn bit_value_to_base64(bit: u8) -> u8 {
  match bit {
    0 ... 25 => ('A' as u8) + bit,
    26 ... 51 => ('a' as u8) + (bit - 26),
    52 ... 61 => ('0' as u8) + (bit - 52),
    62 => '+' as u8,
    63 => '/' as u8,
    _ => '=' as u8
  }
}

fn hex_to_base64(input: Vec<u8>) -> Vec<u8> {
  const NUM_NYBBLES_TO_SHIFT: u8 = 2;

  let mut shift_index = 0;
  let mut temp_word: u32 = 0;

  let mut copy = input.clone();

  // fill in 0s at the end to ensure that we have a divisible by 3 value :)
  let mut n_push = 3 - copy.len() % 3;
  while n_push != 0 {
    copy.push(64); // the 64 is to trigger the bit_value_to_base64 function to return '='
    n_push = n_push - 1;
  }

  let mut triples = Vec::new();
  for byte in input {
    let true_number = byte as u32;

    // pack each hex number to a temporary u32 to keep track of 3 hex digits
    // eg. a: u8, b: u8, c: u8 get packed into a u32 as <0><a><b><c>
    let shift = NUM_NYBBLES_TO_SHIFT - shift_index;
    let shifted = true_number << (shift * 8);
    temp_word = temp_word | shifted;

    // if we have hit the third byte, then push the result into temp_word.
    if shift_index == 2 {
      println!("{:x}", temp_word);
      triples.push(temp_word);
      temp_word = 0;
    }
    shift_index = (shift_index + 1) % 3;
  }

  // now we need to convert triples to an actual result
  let mut result = Vec::new();

  // for a value packed like <0><a><b><c>, where a,b,c are 8 bits
  // we need to convert 24 bits to 4 groups of 6 bits
  // eg. <a><b><c> == <w><x><y><z> where a,b,c are 8 bits, and w,x,y,z are 6 bits
  for quad in triples {
    let first = ((quad >> 18) & 0x003F) as u8;
    let second = ((quad >> 12) & 0x003F) as u8;
    let third = ((quad >> 6) & 0x003F) as u8;
    let fourth = (quad & 0x003F) as u8;

    result.push(bit_value_to_base64(first));
    result.push(bit_value_to_base64(second));
    result.push(bit_value_to_base64(third));
    result.push(bit_value_to_base64(fourth));
  }

  // result
  result
}

#[test]
fn test_hex_to_base64() {
  let input = hex_to_vec(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  let output = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  assert!(hex_to_base64(input.to_vec()) == output.to_vec());
}

fn fixed_xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
  a.iter().zip(b.iter()).map(|(aa,bb)| aa ^ bb).collect()
}

#[test]
fn test_fixed_xor() {
  let a = hex_to_vec(b"1c0111001f010100061a024b53535009181c");
  let b = hex_to_vec(b"686974207468652062756c6c277320657965");

  let result = hex_to_vec(b"746865206b696420646f6e277420706c6179");
  assert!(fixed_xor(&a,&b) == result);
}

// http://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
macro_rules! map {
  { $($key:expr => $value:expr),+ } => {
    {
      let mut m = HashMap::new();
      $(
        m.insert($key,$value);
      )+
      m
    }
  }
}

fn english_error(input: &Vec<u8>) -> f64 {

  // https://en.wikipedia.org/wiki/Letter_frequency
  let english_frequencies: HashMap<u8,f64> = map! {
    'e' as u8 => 0.12702,
    't' as u8 => 0.09056,
    'a' as u8 => 0.08167,
    'o' as u8 => 0.07507,
    'i' as u8 => 0.06966,
    'n' as u8 => 0.06749,
    's' as u8 => 0.06327,
    'h' as u8 => 0.06094,
    'r' as u8 => 0.05987,
    'd' as u8 => 0.04253,
    'l' as u8 => 0.04025,
    'c' as u8 => 0.02782,
    'u' as u8 => 0.02758,
    'm' as u8 => 0.02406,
    'w' as u8 => 0.02360,
    'f' as u8 => 0.02228,
    'g' as u8 => 0.02015,
    'y' as u8 => 0.01974,
    'p' as u8 => 0.01929,
    'b' as u8 => 0.01492,
    'v' as u8 => 0.00978,
    'k' as u8 => 0.00772,
    'j' as u8 => 0.00153,
    'x' as u8 => 0.00150,
    'q' as u8 => 0.00095,
    'z' as u8 => 0.00074
  };


  // create a score of the alphabet
  let mut letter_frequencies: HashMap<u8,f64> = HashMap::new();
  let alphabet = b"abcdefghijklmnopqrstuvwxyz".to_vec();
  for letter in alphabet {
    letter_frequencies.insert(letter, 0.0);
  }

  // count each letter in the input
  let mut invalid = 0.0;
  for b in input {
    let c = (*b as char).to_ascii_lowercase() as u8;

    match letter_frequencies.get_mut(&c) {
      Some(v) => *v += 1.0,
      None => invalid += 1.0
    };
  }
  // calculate the total
  let total_english_chars = letter_frequencies.values().sum();

  // update the letter_frequencies
  for (key,value) in &mut letter_frequencies {
    *value /= total_english_chars;
  }

  if total_english_chars == 0.0 {
    f64::MAX
  } else {
    let sum: f64 = english_frequencies.values().zip(letter_frequencies.values()).map(|(eng,letter)| (eng - letter).abs()).sum();
    // calculate a score that is inverse to the difference between english_frequencies and freqs
    sum + invalid
  }
}

fn single_byte_xor_cipher(input: &Vec<u8>) -> Vec<u8> {
  let mut result = Vec::new();

  let bytes = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 ".to_vec();
  let mut max = f64::MAX;

  for byte in bytes {
    let xor = fixed_xor(&iter::repeat(byte).take(input.len()).collect(), input);
    let score = english_error(&xor);

    if score <= max {
      max = score;
      result = xor;
    }
  }

  result
}

#[test]
fn test_single_byte_xor_cipher() {
  let input = hex_to_vec(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

  match String::from_utf8(single_byte_xor_cipher(&input)) {
    Ok(svalue) => assert!(svalue == String::from("Cooking MC\'s like a pound of bacon")),
    Err(_) => assert!(false),
  }
}
