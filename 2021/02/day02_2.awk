$1 == "forward" { distance += $2; depth += aim * $2 }
$1 == "down" { aim += $2 }
$1 == "up" { aim -= $2 }
END { print distance, depth, distance * depth }
