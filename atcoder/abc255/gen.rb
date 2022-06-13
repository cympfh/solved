M = 1_000_000_000_000_000_000
N = 1_000_000
K = 10

x = rand(2 * K) - K
a = rand(2 * K) - K
d = rand(2 * K) - K

n = 1 + rand(100_000)

p x,a,d,n
