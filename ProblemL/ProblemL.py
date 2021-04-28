
cases = int(input())
count = 0
out = ''

while count < cases:
    inp = input().replace(' ', '_')
    encoded = ''

    # get every 4th index of string starting from 0
    for i in range(0, len(inp), 4):
        # get the 4-character block
        block = inp[i:i+4]
        
        # pad the block if its less than 4 characters
        while len(block) < 4:
            block += '_'
        
        # encode the block with the "encryption algorithm"
        caesar = ''
        for char in block:
            if char == '_':
                caesar += char
            else:
                # (ascii code - 97) gives you the index of the letter
                caesar += chr(97 + ((ord(char) - 97 + 11) % 26))

        # reverse block
        encoded += caesar[::-1]
        
    if count == cases - 1:
        out += encoded
    else:
        out += encoded + '\n'
    
    count += 1

print(out)