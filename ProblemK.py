cases = int(input())
count = 0
out = ''

# I gave up on commenting code...

while count < cases:
    inp = input().split(' ')
    purchased = ''
    coins = 20

    for item in inp:
        if coins < 0:
            purchased += '0 '
        else:
            quantity = int(item)
            purchased += str(min(coins, quantity)) + ' '
        coins -= quantity
        
    if count == cases - 1:
        out += purchased.strip()
    else:
        out += purchased.strip() + '\n'
    count += 1

print(out)