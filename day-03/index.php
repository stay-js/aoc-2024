<?php
$demoInput = file_get_contents("./demo-input.txt", true);
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

echo "demo-input:" . PHP_EOL;
echo first_part($demoInput) . PHP_EOL;

echo PHP_EOL . "input:" . PHP_EOL;
echo first_part($input) . PHP_EOL;
