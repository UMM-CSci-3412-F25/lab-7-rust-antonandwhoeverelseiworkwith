fn main() {
    println!("{}", palindrome("Hello, world!"));
    println!("{}", palindrome("abcdedcba"));
    println!("{}", palindrome("racecar"));
    println!("{}", palindrome("banana"));
}

fn palindrome(s: &str) -> String {

  let string = s.to_string();
  let reversed_string = str_reverse(&string);

  if string == reversed_string {
    let result = "Yes";
    result.to_string()
  } else {
    let result = "No";
    result.to_string()
  }
}

fn str_reverse(s: &str) -> String {
  let string = s.to_string();
  let mut reversed_string: String = "".to_string();
   for c in string.chars().rev(){
      let c = c.to_string();
      reversed_string.push_str(&c);
   }
  reversed_string
}



/*
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "palindrome.h"

char *str_reverse(char const *str) {
  int len, i;
  char *result;

  len = strlen(str);
  result = (char*) calloc(len+1, sizeof(char));
  for (i=0; i<len; ++i) {
    result[i] = str[len-i-1];
  }
  result[len] = '\0';

  return result;
}

char *palindrome(char const *str) {
  char *rev;
  int i;
  bool result = true;
  char *answer;

  rev = str_reverse(str);
  i = 0;
  while (result && str[i]) {
    if (str[i] != rev[i]) {
      result = false;
    }
    ++i;
  }

  if (result) {
    answer = (char*) calloc(4, sizeof(char));
    answer[0] = 'Y';
    answer[1] = 'e';
    answer[2] = 's';
    answer[3] = '\0';
  } else {
    answer = (char*) calloc(3, sizeof(char));
    answer[0] = 'N';
    answer[1] = 'o';
    answer[2] = '\0';
  }

  return answer;
}
*/
