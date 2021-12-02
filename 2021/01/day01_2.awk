NR >=4 {
  curr = first + second + third
  if (curr > last) {
    count ++
  }
}
{
  last = curr;
  third = second;
  second = first;
  first = $1
}
END {
  print count
}
