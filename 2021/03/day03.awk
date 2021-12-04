{
  for (i=0; i<length($1); i++) {
    if (substr($1, i+1, 1) == "1") {
      a[i]++
    } else {
      a[i]--
    }
  }
}

END {
  for (i=0; i< length(a); i++) {
    if (a[i] >= 0) {
      gamma += gamma + 1
      epsilon *= 2
    } else {
      gamma *= 2
      epsilon += epsilon + 1
    }
  }

  print gamma, epsilon, gamma * epsilon
}
