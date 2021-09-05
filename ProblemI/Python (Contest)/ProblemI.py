cases = int(input())
inputs = []

while len(inputs) < cases:
    nums = int(input())
    inputs.append(input().split())


for ledger in inputs:
    scoops = 0

    for x in range(0, len(ledger) - 2):
        for y in range(x + 1, len(ledger) - 1):
            for z in range (y + 1, len(ledger)):
                if int(ledger[x]) < int(ledger[y]) and int(ledger[y]) < int(ledger[z]):
                    scoops += 1
    
    print(scoops)
