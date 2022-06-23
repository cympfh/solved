N = 20000
M = 5000

puts 'a' * N
p M
M.times {|i|
  puts 'a' * ((i%200)+1)
}
M.times {
  puts 3
}
