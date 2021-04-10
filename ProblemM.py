cases = int(input())

out = ''

count = 0

while count < cases:
    inp = input().split(' ')

    players = [i for i in range(1, int(inp[0]) + 1)]

    cursor = 0
    while len(players) > 1:
        cursor += int(inp[1]) - 1
        cursor %= len(players)
        del players[cursor]
        
    if count == cases - 1:
        out += str(players[0])
    else:
        out += str(players[0]) + '\n'
    count += 1

print(out)