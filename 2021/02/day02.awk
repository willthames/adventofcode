$1 == "forward" { distance += $2 }
$1 == "down" { depth += $2 }
$1 == "up" { depth -= $2 }
END { print distance, depth, distance * depth }
