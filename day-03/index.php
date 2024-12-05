<?php
$demoInput = file_get_contents("./demo-input.txt", true);
$demoInput2 = file_get_contents("./demo-input-2.txt", true);
$input = file_get_contents("./input.txt", true);

function first_part(string $input): int
{
  $curr = "";
  $sum = 0;

  foreach (str_split($input) as $char) {
    $curr .= $char;

    if (preg_match("/mul\((\d{0,3}+),(\d{0,3}+)\)/", $curr, $matches)) {
      $sum += $matches[1] * $matches[2];
      $curr = "";
    }
  }

  return $sum;
}

function second_part(string $input): int
{
  $do = true;
  $curr = "";
  $sum = 0;

  foreach (str_split($input) as $char) {
    $curr .= $char;

    if (preg_match("/do\(\)/", $curr)) {
      $do = true;
      $curr = "";
    }

    if (preg_match("/don't\(\)/", $curr)) {
      $do = false;
      $curr = "";
    }

    if (preg_match("/mul\((\d{0,3}),(\d{0,3})\)/", $curr, $matches)) {
      if ($do) $sum += $matches[1] * $matches[2];
      $curr = "";
    }
  }

  return $sum;
}

echo "demo-input:" . PHP_EOL;
echo first_part($demoInput) . PHP_EOL;
echo second_part($demoInput2) . PHP_EOL;

echo PHP_EOL . "input:" . PHP_EOL;
echo first_part($input) . PHP_EOL;
echo second_part($input) . PHP_EOL;
