NR >=2 && $1 > last {
  count ++
}
{
  last = $1
}
END {
  print count
}
